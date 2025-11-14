//! # Morpheus Runtime
//!
//! Loads, manages, and hot-reloads dynamic WASM components.
//!
//! ## Key Responsibilities
//!
//! 1. **Load WASM modules** - Turn compiled bytes into running components
//! 2. **Hot-reload** - Replace running components without breaking the app
//! 3. **Sandbox** - Enforce permission restrictions
//! 4. **Rollback** - Atomically undo bad modifications
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │        MorpheusApp                  │
//! │  (User-facing application)          │
//! └──────────────┬──────────────────────┘
//!                │
//!                ↓
//! ┌─────────────────────────────────────┐
//! │    ComponentRegistry                │
//! │  - Tracks all loaded components     │
//! │  - Manages component lifecycle      │
//! │  - Handles hot-reload               │
//! └──────────────┬──────────────────────┘
//!                │
//!                ↓
//! ┌─────────────────────────────────────┐
//! │    WasmComponent                    │
//! │  - Loads WASM modules               │
//! │  - Enforces permissions             │
//! │  - Provides sandboxed execution     │
//! │  - Hot-reload capability            │
//! └─────────────────────────────────────┘
//! ```

pub mod wasm_loader;

pub use wasm_loader::WasmComponent;

use morpheus_core::component::{ComponentId, ComponentMetadata};
use std::collections::HashMap;

/// Registry of dynamically loaded components.
pub struct ComponentRegistry {
    /// Loaded components by ID.
    components: HashMap<ComponentId, WasmComponent>,

    /// Component metadata.
    metadata: HashMap<ComponentId, ComponentMetadata>,
}

impl ComponentRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Register a loaded component.
    pub fn register(&mut self, id: ComponentId, component: WasmComponent, metadata: ComponentMetadata) {
        self.components.insert(id, component);
        self.metadata.insert(id, metadata);
    }

    /// Get a component by ID.
    pub fn get(&self, id: &ComponentId) -> Option<&WasmComponent> {
        self.components.get(id)
    }

    /// Get a mutable component by ID.
    pub fn get_mut(&mut self, id: &ComponentId) -> Option<&mut WasmComponent> {
        self.components.get_mut(id)
    }

    /// Get component metadata.
    pub fn metadata(&self, id: &ComponentId) -> Option<&ComponentMetadata> {
        self.metadata.get(id)
    }

    /// List all loaded components.
    pub fn list(&self) -> impl Iterator<Item = &ComponentMetadata> {
        self.metadata.values()
    }

    /// Remove a component.
    pub fn remove(&mut self, id: &ComponentId) -> Option<WasmComponent> {
        self.metadata.remove(id);
        self.components.remove(id)
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
