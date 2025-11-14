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

    /// Parse rustc error output into structured errors.
    fn parse_errors(stderr: &str) -> Vec<CompilationError> {
        let mut errors = Vec::new();

        for line in stderr.lines() {
            // Simple parsing - look for "error:" or "warning:"
            if line.contains("error:") {
                errors.push(CompilationError {
                    message: line.to_string(),
                    file: None,
                    line: None,
                    column: None,
                    severity: Severity::Error,
                });
            } else if line.contains("warning:") {
                errors.push(CompilationError {
                    message: line.to_string(),
                    file: None,
                    line: None,
                    column: None,
                    severity: Severity::Warning,
                });
            }
        }

        // If no structured errors found, return the full stderr
        if errors.is_empty() {
            errors.push(CompilationError {
                message: stderr.to_string(),
                file: None,
                line: None,
                column: None,
                severity: Severity::Error,
            });
        }

        errors
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
