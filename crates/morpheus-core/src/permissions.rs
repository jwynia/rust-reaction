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
