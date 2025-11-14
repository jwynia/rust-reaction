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

#[cfg(test)]
mod tests {
    use super::*;
    use morpheus_core::permissions::Permissions;
    use morpheus_core::component::ComponentMetadata;

    fn create_test_metadata(id: u64, name: &str, version: u32) -> ComponentMetadata {
        ComponentMetadata {
            id: ComponentId(id),
            name: name.to_string(),
            version,
            loaded_at: "2025-01-01T00:00:00Z".to_string(),
            ai_generated: false,
        }
    }

    #[tokio::test]
    async fn test_registry_new() {
        let registry = ComponentRegistry::new();
        assert_eq!(registry.components.len(), 0);
        assert_eq!(registry.metadata.len(), 0);
    }

    #[tokio::test]
    async fn test_registry_default() {
        let registry = ComponentRegistry::default();
        assert_eq!(registry.components.len(), 0);
        assert_eq!(registry.metadata.len(), 0);
    }

    #[tokio::test]
    async fn test_register_component() {
        let mut registry = ComponentRegistry::new();

        let wasm_bytes = vec![0x00, 0x61, 0x73, 0x6d]; // WASM magic number
        let component = WasmComponent::load(&wasm_bytes, Permissions::default())
            .await
            .expect("Failed to load component");

        let id = component.id();
        let metadata = create_test_metadata(id.0, "test-component", 1);

        registry.register(id, component, metadata.clone());

        assert_eq!(registry.components.len(), 1);
        assert_eq!(registry.metadata.len(), 1);
        assert!(registry.get(&id).is_some());
        assert!(registry.metadata(&id).is_some());
    }

    #[tokio::test]
    async fn test_get_component() {
        let mut registry = ComponentRegistry::new();

        let wasm_bytes = vec![0x00, 0x61, 0x73, 0x6d];
        let component = WasmComponent::load(&wasm_bytes, Permissions::default())
            .await
            .expect("Failed to load component");

        let id = component.id();
        let metadata = create_test_metadata(id.0, "test-component", 1);

        registry.register(id, component, metadata);

        let retrieved = registry.get(&id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), id);
    }

    #[tokio::test]
    async fn test_get_mut_component() {
        let mut registry = ComponentRegistry::new();

        let wasm_bytes = vec![0x00, 0x61, 0x73, 0x6d];
        let component = WasmComponent::load(&wasm_bytes, Permissions::default())
            .await
            .expect("Failed to load component");

        let id = component.id();
        let metadata = create_test_metadata(id.0, "test-component", 1);

        registry.register(id, component, metadata);

        let component_mut = registry.get_mut(&id);
        assert!(component_mut.is_some());
    }

    #[tokio::test]
    async fn test_get_metadata() {
        let mut registry = ComponentRegistry::new();

        let wasm_bytes = vec![0x00, 0x61, 0x73, 0x6d];
        let component = WasmComponent::load(&wasm_bytes, Permissions::default())
            .await
            .expect("Failed to load component");

        let id = component.id();
        let metadata = create_test_metadata(id.0, "test-component", 1);

        registry.register(id, component, metadata.clone());

        let retrieved_metadata = registry.metadata(&id);
        assert!(retrieved_metadata.is_some());
        assert_eq!(retrieved_metadata.unwrap().name, "test-component");
        assert_eq!(retrieved_metadata.unwrap().version, 1);
    }

    #[tokio::test]
    async fn test_list_components() {
        let mut registry = ComponentRegistry::new();

        // Register multiple components
        for i in 0..3 {
            let wasm_bytes = vec![i as u8; 4];
            let component = WasmComponent::load(&wasm_bytes, Permissions::default())
                .await
                .expect("Failed to load component");

            let id = component.id();
            let metadata = create_test_metadata(id.0, &format!("component-{}", i), 1);

            registry.register(id, component, metadata);
        }

        let list: Vec<_> = registry.list().collect();
        assert_eq!(list.len(), 3);
    }

    #[tokio::test]
    async fn test_remove_component() {
        let mut registry = ComponentRegistry::new();

        let wasm_bytes = vec![0x00, 0x61, 0x73, 0x6d];
        let component = WasmComponent::load(&wasm_bytes, Permissions::default())
            .await
            .expect("Failed to load component");

        let id = component.id();
        let metadata = create_test_metadata(id.0, "test-component", 1);

        registry.register(id, component, metadata);
        assert_eq!(registry.components.len(), 1);

        let removed = registry.remove(&id);
        assert!(removed.is_some());
        assert_eq!(registry.components.len(), 0);
        assert_eq!(registry.metadata.len(), 0);
        assert!(registry.get(&id).is_none());
    }

    #[tokio::test]
    async fn test_remove_nonexistent_component() {
        let mut registry = ComponentRegistry::new();
        let fake_id = ComponentId(999);

        let removed = registry.remove(&fake_id);
        assert!(removed.is_none());
    }

    #[tokio::test]
    async fn test_multiple_components() {
        let mut registry = ComponentRegistry::new();

        let component1_bytes = vec![1, 2, 3, 4];
        let component2_bytes = vec![5, 6, 7, 8];

        let comp1 = WasmComponent::load(&component1_bytes, Permissions::default())
            .await
            .unwrap();
        let comp2 = WasmComponent::load(&component2_bytes, Permissions::default())
            .await
            .unwrap();

        let id1 = comp1.id();
        let id2 = comp2.id();

        let meta1 = create_test_metadata(id1.0, "component-1", 1);
        let meta2 = create_test_metadata(id2.0, "component-2", 1);

        registry.register(id1, comp1, meta1);
        registry.register(id2, comp2, meta2);

        assert_eq!(registry.components.len(), 2);
        assert!(registry.get(&id1).is_some());
        assert!(registry.get(&id2).is_some());
        assert_ne!(id1, id2);
    }

    #[tokio::test]
    async fn test_overwrite_component() {
        let mut registry = ComponentRegistry::new();

        let wasm_bytes = vec![0x00, 0x61, 0x73, 0x6d];
        let component1 = WasmComponent::load(&wasm_bytes, Permissions::default())
            .await
            .unwrap();

        let id = component1.id();
        let metadata1 = create_test_metadata(id.0, "version-1", 1);

        registry.register(id, component1, metadata1);

        // Register again with same ID but different metadata
        let component2 = WasmComponent::load(&wasm_bytes, Permissions::default())
            .await
            .unwrap();
        let metadata2 = create_test_metadata(id.0, "version-2", 2);

        registry.register(id, component2, metadata2);

        // Should have overwritten
        assert_eq!(registry.components.len(), 1);
        assert_eq!(registry.metadata(&id).unwrap().name, "version-2");
        assert_eq!(registry.metadata(&id).unwrap().version, 2);
    }
}
