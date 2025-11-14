//! # Morpheus Compiler
//!
//! Compiles AI-generated Rust code to WASM modules for safe hot-loading.
//!
//! ## The Core Safety Mechanism
//!
//! This is where Morpheus differs from TypeScript/JavaScript:
//! AI-generated code MUST compile and type-check before it can run.
//!
//! ```rust,ignore
//! let ai_code = ai.generate("Add dark mode toggle").await?;
//!
//! match compiler.compile(&ai_code).await {
//!     Ok(wasm) => {
//!         // Type-checked! Safe to load.
//!         app.hot_reload(wasm)?;
//!     }
//!     Err(errors) => {
//!         // App still works! Show errors to user and AI.
//!         show_user("AI made a mistake: {}", errors);
//!         ai.fix_errors(&ai_code, errors).await?;
//!     }
//! }
//! ```
//!
//! ## Implementation Approaches
//!
//! Several options for runtime Rust compilation:
//!
//! 1. **Spawn rustc/wasm-pack** - Simple but heavyweight
//! 2. **Cargo as library** - More integrated
//! 3. **wasm-bindgen directly** - Skip cargo overhead
//! 4. **Rust interpreter** - Faster iteration but less safe
//!
//! ## To Be Determined
//!
//! This crate is a placeholder. Actual implementation will be determined
//! after researching the best approach for:
//! - Compilation speed (<5 seconds target)
//! - Sandboxing during compilation
//! - Error message quality
//! - Integration with existing tools

use morpheus_core::errors::{MorpheusError, Result};
use async_trait::async_trait;

/// A compiler that can turn Rust code into WASM modules.
#[async_trait]
pub trait Compiler {
    /// Compile Rust source code to a WASM module.
    ///
    /// Returns the compiled WASM bytes if successful, or compilation errors.
    async fn compile(&self, source: &str) -> Result<Vec<u8>>;

    /// Check if source code type-checks without generating WASM.
    ///
    /// Faster than full compilation for quick validation.
    async fn check(&self, source: &str) -> Result<()>;
}

/// Compilation errors with source locations.
#[derive(Debug, Clone)]
pub struct CompilationError {
    /// Error message from rustc.
    pub message: String,

    /// File path (if available).
    pub file: Option<String>,

    /// Line number (1-indexed).
    pub line: Option<usize>,

    /// Column number (1-indexed).
    pub column: Option<usize>,

    /// Severity (error, warning, note).
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

// Placeholder implementation
pub struct PlaceholderCompiler;

#[async_trait]
impl Compiler for PlaceholderCompiler {
    async fn compile(&self, _source: &str) -> Result<Vec<u8>> {
        Err(MorpheusError::CompilationError(
            "Compiler not yet implemented - placeholder only".to_string()
        ))
    }

    async fn check(&self, _source: &str) -> Result<()> {
        Err(MorpheusError::CompilationError(
            "Compiler not yet implemented - placeholder only".to_string()
        ))
    }
}
