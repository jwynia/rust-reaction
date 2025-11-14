//! Error types for Morpheus.

use thiserror::Error;

/// Errors that can occur during component operations.
#[derive(Debug, Error)]
pub enum MorpheusError {
    /// Component failed to compile.
    #[error("Compilation failed: {0}")]
    CompilationError(String),

    /// Component failed to load.
    #[error("Failed to load component: {0}")]
    LoadError(String),

    /// Component violated permissions.
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Invalid component state.
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Serialization/deserialization error.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Generic error.
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, MorpheusError>;
