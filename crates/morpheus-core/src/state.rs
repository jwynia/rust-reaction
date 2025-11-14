//! Versioned state management with rollback support.
//!
//! All state changes are tracked so modifications can be rolled back atomically.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Maximum number of snapshots to keep in history.
const MAX_HISTORY: usize = 50;

/// Versioned state with snapshot/rollback capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedState<T> {
    /// Current state.
    current: T,

    /// Version number (increments on each change).
    version: u64,

    /// History of snapshots for rollback.
    history: VecDeque<Snapshot<T>>,
}

/// A snapshot of state at a specific version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot<T> {
    /// The state at this version.
    pub state: T,

    /// Version number.
    pub version: u64,

    /// When this snapshot was taken (ISO 8601).
    pub timestamp: String,
}

impl<T: Clone> VersionedState<T> {
    /// Create new versioned state with initial value.
    pub fn new(initial: T) -> Self {
        Self {
            current: initial,
            version: 0,
            history: VecDeque::new(),
        }
    }

    /// Get current state.
    pub fn get(&self) -> &T {
        &self.current
    }

    /// Get current version number.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Take a snapshot of current state.
    pub fn snapshot(&self) -> Snapshot<T> {
        Snapshot {
            state: self.current.clone(),
            version: self.version,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Update state and save snapshot.
    pub fn update(&mut self, new_state: T) {
        // Save current state to history
        let snapshot = self.snapshot();
        self.history.push_back(snapshot);

        // Limit history size
        if self.history.len() > MAX_HISTORY {
            self.history.pop_front();
        }

        // Update to new state
        self.current = new_state;
        self.version += 1;
    }

    /// Rollback to previous snapshot.
    ///
    /// Returns true if rollback succeeded, false if no history.
    pub fn rollback(&mut self) -> bool {
        if let Some(snapshot) = self.history.pop_back() {
            self.current = snapshot.state;
            self.version = snapshot.version;
            true
        } else {
            false
        }
    }

    /// Restore to a specific snapshot.
    pub fn restore(&mut self, snapshot: Snapshot<T>) {
        self.current = snapshot.state;
        self.version = snapshot.version;
    }

    /// Get history of snapshots.
    pub fn history(&self) -> &VecDeque<Snapshot<T>> {
        &self.history
    }

    /// Clear all history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

// Temporary workaround: use strings for timestamps instead of chrono
// Will add chrono dependency when we need proper time handling
mod chrono {
    pub struct Utc;
    impl Utc {
        pub fn now() -> Self {
            Self
        }
    }
    impl Utc {
        pub fn to_rfc3339(&self) -> String {
            // Placeholder - would use actual chrono in real implementation
            "2025-01-01T00:00:00Z".to_string()
        }
    }
}
