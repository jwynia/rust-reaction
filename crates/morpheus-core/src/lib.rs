//! # Morpheus Core
//!
//! Core types and traits for building self-modifying applications with AI.
//!
//! ## Philosophy
//!
//! Traditional web frameworks ask: "How do I build web apps?"
//!
//! Morpheus asks: "How do I build apps that safely modify themselves with AI?"
//!
//! ## Key Concepts
//!
//! - **Dynamic Components**: WASM modules that can be loaded at runtime
//! - **Safe Hot Reload**: Type-checked component updates without breaking the app
//! - **Sandboxing**: Components run with restricted permissions
//! - **Transactional Updates**: All modifications can be rolled back atomically
//! - **Compilation Gate**: AI code must type-check before running
//!
//! ## Example
//!
//! ```rust,ignore
//! use morpheus_core::*;
//!
//! // User says: "Add a dark mode toggle"
//! let modification = ai.generate("Add dark mode toggle").await?;
//!
//! // Framework compiles and validates
//! match compiler.compile(&modification) {
//!     Ok(wasm) => {
//!         // Type-checked! Safe to load.
//!         app.hot_reload(wasm)?;
//!     }
//!     Err(errors) => {
//!         // App still works! Show errors.
//!         show_user("AI made a mistake: {}", errors);
//!         ai.fix_errors(&modification, errors).await?;
//!     }
//! }
//! ```

pub mod component;
pub mod permissions;
pub mod state;
pub mod errors;

pub mod prelude {
    //! Commonly used types and traits.
    pub use crate::component::*;
    pub use crate::permissions::*;
    pub use crate::state::*;
    pub use crate::errors::*;
}
