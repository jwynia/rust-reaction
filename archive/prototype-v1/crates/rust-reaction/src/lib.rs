//! # Rust Reaction
//!
//! A truly Rust-native frontend framework for building web UIs.
//!
//! ## Philosophy
//!
//! Rust Reaction embraces Rust idioms rather than translating JavaScript patterns:
//!
//! - **Components as structs** - Not function components with hooks
//! - **Builder pattern** - Not JSX-like macros
//! - **Trait-based composition** - Not higher-order components
//! - **RAII for effects** - Not cleanup functions
//! - **Result for errors** - Not error boundaries
//! - **Type-state pattern** - For compile-time validation
//! - **Ownership-based lifecycle** - Not manual memory management
//!
//! ## Example
//!
//! ```rust
//! use rust_reaction::prelude::*;
//!
//! struct Counter {
//!     count: i32,
//! }
//!
//! impl Counter {
//!     fn new() -> Self {
//!         Self { count: 0 }
//!     }
//!
//!     fn increment(&mut self) {
//!         self.count += 1;
//!     }
//! }
//!
//! impl Component for Counter {
//!     type Message = CounterMsg;
//!
//!     fn view(&self) -> impl View {
//!         div()
//!             .class("counter")
//!             .child(
//!                 button()
//!                     .text("Increment")
//!                     .on_click(CounterMsg::Increment)
//!             )
//!             .child(
//!                 text(format!("Count: {}", self.count))
//!             )
//!     }
//!
//!     fn update(&mut self, msg: Self::Message) {
//!         match msg {
//!             CounterMsg::Increment => self.increment(),
//!         }
//!     }
//! }
//! ```

pub mod component;
pub mod dom;
pub mod event;
pub mod state;
pub mod view;
pub mod routing;

pub mod prelude {
    //! Commonly used types and traits.

    pub use crate::component::{Component, ComponentHandle};
    pub use crate::dom::*;
    pub use crate::event::*;
    pub use crate::state::*;
    pub use crate::view::*;
    pub use crate::routing::*;
}
