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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilation_error() {
        let error = MorpheusError::CompilationError("missing semicolon".to_string());
        let message = error.to_string();

        assert!(message.contains("Compilation failed"));
        assert!(message.contains("missing semicolon"));
    }

    #[test]
    fn test_load_error() {
        let error = MorpheusError::LoadError("invalid WASM module".to_string());
        let message = error.to_string();

        assert!(message.contains("Failed to load component"));
        assert!(message.contains("invalid WASM module"));
    }

    #[test]
    fn test_permission_denied() {
        let error = MorpheusError::PermissionDenied("network access not allowed".to_string());
        let message = error.to_string();

        assert!(message.contains("Permission denied"));
        assert!(message.contains("network access not allowed"));
    }

    #[test]
    fn test_invalid_state() {
        let error = MorpheusError::InvalidState("state version mismatch".to_string());
        let message = error.to_string();

        assert!(message.contains("Invalid state"));
        assert!(message.contains("state version mismatch"));
    }

    #[test]
    fn test_serialization_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json")
            .expect_err("Should fail to parse");

        let morpheus_error: MorpheusError = json_error.into();

        match morpheus_error {
            MorpheusError::SerializationError(_) => {
                // Correct error type
            }
            _ => panic!("Expected SerializationError variant"),
        }
    }

    #[test]
    fn test_other_error() {
        let error = MorpheusError::Other("something went wrong".to_string());
        let message = error.to_string();

        assert_eq!(message, "something went wrong");
    }

    #[test]
    fn test_result_type_ok() {
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_type_err() {
        let result: Result<i32> = Err(MorpheusError::Other("test error".to_string()));
        assert!(result.is_err());

        match result {
            Err(MorpheusError::Other(msg)) => assert_eq!(msg, "test error"),
            _ => panic!("Expected error"),
        }
    }

    #[test]
    fn test_error_chaining() {
        fn inner_fn() -> Result<()> {
            Err(MorpheusError::CompilationError("inner error".to_string()))
        }

        fn outer_fn() -> Result<()> {
            inner_fn()?;
            Ok(())
        }

        let result = outer_fn();
        assert!(result.is_err());

        match result {
            Err(MorpheusError::CompilationError(msg)) => {
                assert_eq!(msg, "inner error");
            }
            _ => panic!("Expected CompilationError"),
        }
    }

    #[test]
    fn test_error_types_are_distinct() {
        let comp_err = MorpheusError::CompilationError("a".to_string());
        let load_err = MorpheusError::LoadError("b".to_string());
        let perm_err = MorpheusError::PermissionDenied("c".to_string());

        // Each should have a different string representation
        assert_ne!(comp_err.to_string(), load_err.to_string());
        assert_ne!(load_err.to_string(), perm_err.to_string());
        assert_ne!(comp_err.to_string(), perm_err.to_string());
    }
}
