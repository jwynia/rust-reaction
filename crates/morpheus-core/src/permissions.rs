//! Permission system for sandboxing dynamic components.
//!
//! AI-generated components run with restricted permissions to prevent
//! malicious or buggy code from compromising the application.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Permissions granted to a component.
///
/// Components declare what they need, and the runtime enforces limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions {
    /// Network access permissions.
    pub network: NetworkPermissions,

    /// Local storage access.
    pub storage: StoragePermissions,

    /// Which JavaScript APIs can be accessed.
    pub apis: HashSet<ApiPermission>,
}

impl Default for Permissions {
    /// By default, components have NO permissions.
    ///
    /// This is especially important for AI-generated components.
    fn default() -> Self {
        Self {
            network: NetworkPermissions::Denied,
            storage: StoragePermissions::None,
            apis: HashSet::new(),
        }
    }
}

/// Network access permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkPermissions {
    /// No network access allowed.
    Denied,

    /// Can access specific domains only.
    AllowList(Vec<String>),

    /// Can access any domain (use sparingly!).
    Unrestricted,
}

/// Storage access permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoragePermissions {
    /// No storage access.
    None,

    /// Can access specific keys only.
    Limited(Vec<String>),

    /// Full access (use sparingly!).
    Full,
}

/// Specific JavaScript APIs that can be accessed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiPermission {
    /// Geolocation API.
    Geolocation,

    /// Notification API.
    Notifications,

    /// Camera access.
    Camera,

    /// Microphone access.
    Microphone,

    /// Clipboard access.
    Clipboard,

    /// WebGL/Canvas rendering.
    Graphics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_permissions_are_restrictive() {
        let perms = Permissions::default();

        // Should have NO permissions by default
        assert!(matches!(perms.network, NetworkPermissions::Denied));
        assert!(matches!(perms.storage, StoragePermissions::None));
        assert!(perms.apis.is_empty());
    }

    #[test]
    fn test_network_permissions_variants() {
        let denied = NetworkPermissions::Denied;
        assert!(matches!(denied, NetworkPermissions::Denied));

        let allow_list = NetworkPermissions::AllowList(vec!["api.example.com".to_string()]);
        if let NetworkPermissions::AllowList(domains) = &allow_list {
            assert_eq!(domains.len(), 1);
            assert_eq!(domains[0], "api.example.com");
        } else {
            panic!("Expected AllowList variant");
        }

        let unrestricted = NetworkPermissions::Unrestricted;
        assert!(matches!(unrestricted, NetworkPermissions::Unrestricted));
    }

    #[test]
    fn test_storage_permissions_variants() {
        let none = StoragePermissions::None;
        assert!(matches!(none, StoragePermissions::None));

        let limited = StoragePermissions::Limited(vec!["user_prefs".to_string()]);
        if let StoragePermissions::Limited(keys) = &limited {
            assert_eq!(keys.len(), 1);
            assert_eq!(keys[0], "user_prefs");
        } else {
            panic!("Expected Limited variant");
        }

        let full = StoragePermissions::Full;
        assert!(matches!(full, StoragePermissions::Full));
    }

    #[test]
    fn test_api_permissions() {
        let mut perms = Permissions::default();

        // Start with no API permissions
        assert!(!perms.apis.contains(&ApiPermission::Geolocation));
        assert!(!perms.apis.contains(&ApiPermission::Camera));

        // Add specific permissions
        perms.apis.insert(ApiPermission::Geolocation);
        perms.apis.insert(ApiPermission::Clipboard);

        assert!(perms.apis.contains(&ApiPermission::Geolocation));
        assert!(perms.apis.contains(&ApiPermission::Clipboard));
        assert!(!perms.apis.contains(&ApiPermission::Camera));
        assert_eq!(perms.apis.len(), 2);
    }

    #[test]
    fn test_permissions_serialization() {
        let mut perms = Permissions {
            network: NetworkPermissions::AllowList(vec![
                "api.example.com".to_string(),
                "cdn.example.com".to_string(),
            ]),
            storage: StoragePermissions::Limited(vec!["cache".to_string()]),
            apis: HashSet::new(),
        };
        perms.apis.insert(ApiPermission::Notifications);
        perms.apis.insert(ApiPermission::Graphics);

        let json = serde_json::to_string(&perms).expect("Failed to serialize");
        let deserialized: Permissions =
            serde_json::from_str(&json).expect("Failed to deserialize");

        // Verify network permissions
        if let NetworkPermissions::AllowList(domains) = &deserialized.network {
            assert_eq!(domains.len(), 2);
        } else {
            panic!("Expected AllowList variant");
        }

        // Verify storage permissions
        if let StoragePermissions::Limited(keys) = &deserialized.storage {
            assert_eq!(keys.len(), 1);
        } else {
            panic!("Expected Limited variant");
        }

        // Verify API permissions
        assert_eq!(deserialized.apis.len(), 2);
        assert!(deserialized.apis.contains(&ApiPermission::Notifications));
        assert!(deserialized.apis.contains(&ApiPermission::Graphics));
    }

    #[test]
    fn test_api_permission_equality() {
        let geo1 = ApiPermission::Geolocation;
        let geo2 = ApiPermission::Geolocation;
        let cam = ApiPermission::Camera;

        assert_eq!(geo1, geo2);
        assert_ne!(geo1, cam);
    }

    #[test]
    fn test_api_permission_in_hashset() {
        let mut apis = HashSet::new();

        // Insert same permission twice
        apis.insert(ApiPermission::Camera);
        apis.insert(ApiPermission::Camera);

        // Should only have one entry
        assert_eq!(apis.len(), 1);

        apis.insert(ApiPermission::Microphone);
        assert_eq!(apis.len(), 2);
    }

    #[test]
    fn test_restrictive_permissions() {
        // AI-generated components should start with minimal permissions
        let ai_component_perms = Permissions::default();

        // Verify they can't do anything dangerous by default
        assert!(matches!(
            ai_component_perms.network,
            NetworkPermissions::Denied
        ));
        assert!(matches!(
            ai_component_perms.storage,
            StoragePermissions::None
        ));
        assert!(ai_component_perms.apis.is_empty());
    }

    #[test]
    fn test_permissive_configuration() {
        // Trusted components can be granted more permissions
        let mut trusted_perms = Permissions {
            network: NetworkPermissions::Unrestricted,
            storage: StoragePermissions::Full,
            apis: HashSet::new(),
        };

        // Grant all API permissions
        trusted_perms.apis.insert(ApiPermission::Geolocation);
        trusted_perms.apis.insert(ApiPermission::Notifications);
        trusted_perms.apis.insert(ApiPermission::Camera);
        trusted_perms.apis.insert(ApiPermission::Microphone);
        trusted_perms.apis.insert(ApiPermission::Clipboard);
        trusted_perms.apis.insert(ApiPermission::Graphics);

        assert!(matches!(
            trusted_perms.network,
            NetworkPermissions::Unrestricted
        ));
        assert!(matches!(trusted_perms.storage, StoragePermissions::Full));
        assert_eq!(trusted_perms.apis.len(), 6);
    }
}
