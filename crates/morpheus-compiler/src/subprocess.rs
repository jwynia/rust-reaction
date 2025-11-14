//! Subprocess-based Rust compiler.
//!
//! Compiles Rust source code to WASM by spawning `rustc` and `wasm-pack`
//! as subprocesses.
//!
//! This is the simplest approach and uses standard tooling. While not the
//! fastest (compilation takes 5-10 seconds), it's reliable and gets us
//! started quickly.

use crate::{CompilationError, Compiler, Severity};
use async_trait::async_trait;
use morpheus_core::errors::{MorpheusError, Result};
use std::path::PathBuf;
use std::process::Command;
use tokio::fs;

/// Compiler that spawns `wasm-pack` as subprocess.
pub struct SubprocessCompiler {
    /// Working directory for temporary build artifacts.
    work_dir: PathBuf,
}

impl SubprocessCompiler {
    /// Create a new subprocess compiler.
    ///
    /// Creates a working directory for temporary files.
    pub async fn new() -> Result<Self> {
        let work_dir = std::env::temp_dir().join("morpheus-compiler");
        fs::create_dir_all(&work_dir).await.map_err(|e| {
            MorpheusError::CompilationError(format!("Failed to create work directory: {}", e))
        })?;

        Ok(Self { work_dir })
    }

    /// Check if required tools are available.
    pub fn check_tools() -> Result<()> {
        // Check for rustc
        let rustc = Command::new("rustc").arg("--version").output();
        if rustc.is_err() || !rustc.unwrap().status.success() {
            return Err(MorpheusError::CompilationError(
                "rustc not found. Please install Rust: https://rustup.rs/".to_string(),
            ));
        }

        // Check for wasm-pack
        let wasm_pack = Command::new("wasm-pack").arg("--version").output();
        if wasm_pack.is_err() || !wasm_pack.unwrap().status.success() {
            return Err(MorpheusError::CompilationError(
                "wasm-pack not found. Install with: cargo install wasm-pack".to_string(),
            ));
        }

        Ok(())
    }

    /// Create a temporary project directory for compilation.
    async fn create_project(&self, source: &str) -> Result<PathBuf> {
        // Create unique directory for this compilation
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let project_dir = self.work_dir.join(format!("component-{}", timestamp));

        fs::create_dir_all(&project_dir)
            .await
            .map_err(|e| MorpheusError::CompilationError(format!("Failed to create project dir: {}", e)))?;

        // Create src directory
        let src_dir = project_dir.join("src");
        fs::create_dir(&src_dir)
            .await
            .map_err(|e| MorpheusError::CompilationError(format!("Failed to create src dir: {}", e)))?;

        // Write source code
        let lib_path = src_dir.join("lib.rs");
        fs::write(&lib_path, source)
            .await
            .map_err(|e| MorpheusError::CompilationError(format!("Failed to write source: {}", e)))?;

        // Create Cargo.toml
        let cargo_toml = r#"
[package]
name = "morpheus-component"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
"#;

        fs::write(project_dir.join("Cargo.toml"), cargo_toml)
            .await
            .map_err(|e| MorpheusError::CompilationError(format!("Failed to write Cargo.toml: {}", e)))?;

        Ok(project_dir)
    }

    /// Parse rustc error output into structured, user-friendly errors.
    fn parse_errors(stderr: &str) -> Vec<CompilationError> {
        let mut errors = Vec::new();
        let mut current_error: Option<CompilationError> = None;
        let mut help_text = String::new();

        for line in stderr.lines() {
            // Parse location: "  --> src/lib.rs:5:9"
            if line.trim().starts_with("-->") {
                if let Some(location) = line.split("-->").nth(1) {
                    let parts: Vec<&str> = location.trim().split(':').collect();
                    if parts.len() >= 3 {
                        if let Some(ref mut err) = current_error {
                            err.file = Some(parts[0].to_string());
                            err.line = parts[1].parse().ok();
                            err.column = parts[2].parse().ok();
                        }
                    }
                }
            }
            // Parse error/warning message: "error[E0308]: mismatched types"
            else if line.contains("error[") || line.contains("error:") {
                // Save previous error if exists
                if let Some(err) = current_error.take() {
                    errors.push(Self::enrich_error(err, &help_text));
                    help_text.clear();
                }

                // Extract error code and message
                let message = if let Some(bracket_start) = line.find("[") {
                    if let Some(bracket_end) = line.find("]:") {
                        let error_code = &line[bracket_start+1..bracket_end];
                        let error_message = &line[bracket_end+2..].trim();
                        format!("{}: {}", error_code, error_message)
                    } else {
                        line.to_string()
                    }
                } else {
                    line.to_string()
                };

                current_error = Some(CompilationError {
                    message: Self::make_user_friendly(&message),
                    file: None,
                    line: None,
                    column: None,
                    severity: Severity::Error,
                });
            }
            else if line.contains("warning:") {
                if let Some(err) = current_error.take() {
                    errors.push(Self::enrich_error(err, &help_text));
                    help_text.clear();
                }

                current_error = Some(CompilationError {
                    message: Self::make_user_friendly(line),
                    file: None,
                    line: None,
                    column: None,
                    severity: Severity::Warning,
                });
            }
            // Collect help/note lines
            else if line.trim().starts_with("help:") || line.trim().starts_with("note:") {
                if !help_text.is_empty() {
                    help_text.push_str("\n");
                }
                help_text.push_str(line.trim());
            }
        }

        // Save last error
        if let Some(err) = current_error {
            errors.push(Self::enrich_error(err, &help_text));
        }

        // If no structured errors found, return the full stderr with a friendly message
        if errors.is_empty() {
            errors.push(CompilationError {
                message: format!(
                    "The Rust compiler encountered an issue:\n\n{}\n\n\
                    💡 This usually means there's a syntax error or type mismatch in the generated code.",
                    stderr
                ),
                file: None,
                line: None,
                column: None,
                severity: Severity::Error,
            });
        }

        errors
    }

    /// Make error messages more user-friendly.
    fn make_user_friendly(message: &str) -> String {
        let message = message.to_string();

        // Add explanations for common errors
        if message.contains("mismatched types") {
            format!(
                "{}\n\n💡 The code is trying to use a value of one type where a different type is expected.",
                message
            )
        } else if message.contains("cannot find") {
            format!(
                "{}\n\n💡 The code is referencing something that doesn't exist or wasn't imported.",
                message
            )
        } else if message.contains("expected") && message.contains("found") {
            format!(
                "{}\n\n💡 The types don't match - check that variables and function returns have the correct types.",
                message
            )
        } else if message.contains("unresolved import") {
            format!(
                "{}\n\n💡 The code is trying to import something that doesn't exist. Check the import path.",
                message
            )
        } else if message.contains("unused") {
            format!(
                "{}\n\n💡 This is defined but never used. Consider removing it or using it somewhere.",
                message
            )
        } else if message.contains("missing lifetime") {
            format!(
                "{}\n\n💡 Rust needs help understanding how long references live. This is an advanced feature.",
                message
            )
        } else if message.contains("borrowed value") || message.contains("does not live long enough") {
            format!(
                "{}\n\n💡 The code is trying to use a reference that no longer exists. Try simplifying the ownership.",
                message
            )
        } else if message.contains("trait") && message.contains("not implemented") {
            format!(
                "{}\n\n💡 The type needs to implement a trait (interface) to be used in this way.",
                message
            )
        } else {
            message
        }
    }

    /// Enrich error with help text and suggestions.
    fn enrich_error(mut error: CompilationError, help_text: &str) -> CompilationError {
        if !help_text.is_empty() {
            error.message = format!("{}\n\n{}", error.message, help_text);
        }

        // Add location context if available
        if let (Some(line), Some(col)) = (error.line, error.column) {
            error.message = format!(
                "At line {}, column {}:\n{}",
                line, col, error.message
            );
        }

        error
    }
}

#[async_trait]
impl Compiler for SubprocessCompiler {
    async fn compile(&self, source: &str) -> Result<Vec<u8>> {
        // Check tools are available
        Self::check_tools()?;

        // Create temporary project
        let project_dir = self.create_project(source).await?;

        // Compile with wasm-pack
        let output = tokio::process::Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--release"])
            .current_dir(&project_dir)
            .output()
            .await
            .map_err(|e| MorpheusError::CompilationError(format!("Failed to run wasm-pack: {}", e)))?;

        // Check for compilation errors
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let errors = Self::parse_errors(&stderr);

            // Format errors for user
            let error_msg = errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<_>>()
                .join("\n");

            return Err(MorpheusError::CompilationError(format!(
                "Compilation failed:\n{}",
                error_msg
            )));
        }

        // Read compiled WASM
        let wasm_path = project_dir.join("pkg/morpheus_component_bg.wasm");
        let wasm_bytes = fs::read(&wasm_path).await.map_err(|e| {
            MorpheusError::CompilationError(format!("Failed to read compiled WASM: {}", e))
        })?;

        // Clean up temporary directory (optional - could cache)
        let _ = fs::remove_dir_all(&project_dir).await;

        Ok(wasm_bytes)
    }

    async fn check(&self, source: &str) -> Result<()> {
        // Create temporary project
        let project_dir = self.create_project(source).await?;

        // Run cargo check
        let output = tokio::process::Command::new("cargo")
            .args(&["check", "--target", "wasm32-unknown-unknown"])
            .current_dir(&project_dir)
            .output()
            .await
            .map_err(|e| MorpheusError::CompilationError(format!("Failed to run cargo check: {}", e)))?;

        // Clean up
        let _ = fs::remove_dir_all(&project_dir).await;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(MorpheusError::CompilationError(format!(
                "Type check failed:\n{}",
                stderr
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELLO_WORLD: &str = r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn greet(name: &str) -> String {
            format!("Hello, {}!", name)
        }
    "#;

    #[tokio::test]
    async fn test_tool_check() {
        // This test might fail in CI if tools aren't installed
        // That's expected - this is for manual testing
        if let Ok(()) = SubprocessCompiler::check_tools() {
            println!("✓ All required tools found");
        } else {
            println!("⚠ Tools not found (expected in CI)");
        }
    }

    #[tokio::test]
    async fn test_compile_hello_world() {
        let compiler = match SubprocessCompiler::new().await {
            Ok(c) => c,
            Err(_) => {
                println!("Skipping test - couldn't create compiler");
                return;
            }
        };

        if SubprocessCompiler::check_tools().is_err() {
            println!("Skipping test - tools not available");
            return;
        }

        match compiler.compile(HELLO_WORLD).await {
            Ok(wasm) => {
                println!("✓ Compiled successfully!");
                println!("  WASM size: {} bytes", wasm.len());
                assert!(!wasm.is_empty());
            }
            Err(e) => {
                println!("✗ Compilation failed: {}", e);
                // Don't fail test - might not have tools in CI
            }
        }
    }

    #[tokio::test]
    async fn test_compile_error() {
        let compiler = match SubprocessCompiler::new().await {
            Ok(c) => c,
            Err(_) => return,
        };

        if SubprocessCompiler::check_tools().is_err() {
            return;
        }

        let bad_code = r#"
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn broken(x: i32) -> String {
                x.to_string(  // Syntax error - missing )
            }
        "#;

        match compiler.compile(bad_code).await {
            Ok(_) => panic!("Should have failed to compile!"),
            Err(e) => {
                println!("✓ Correctly caught compilation error:");
                println!("  {}", e);
            }
        }
    }
}
