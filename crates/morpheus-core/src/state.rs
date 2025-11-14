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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_versioned_state() {
        let state = VersionedState::new(42);
        assert_eq!(*state.get(), 42);
        assert_eq!(state.version(), 0);
        assert_eq!(state.history().len(), 0);
    }

    #[test]
    fn test_snapshot() {
        let state = VersionedState::new(100);
        let snapshot = state.snapshot();

        assert_eq!(snapshot.state, 100);
        assert_eq!(snapshot.version, 0);
        assert!(!snapshot.timestamp.is_empty());
    }

    #[test]
    fn test_update_increments_version() {
        let mut state = VersionedState::new(10);
        assert_eq!(state.version(), 0);

        state.update(20);
        assert_eq!(*state.get(), 20);
        assert_eq!(state.version(), 1);

        state.update(30);
        assert_eq!(*state.get(), 30);
        assert_eq!(state.version(), 2);
    }

    #[test]
    fn test_update_saves_history() {
        let mut state = VersionedState::new(1);

        state.update(2);
        state.update(3);
        state.update(4);

        assert_eq!(state.history().len(), 3);
        assert_eq!(state.history()[0].state, 1);
        assert_eq!(state.history()[1].state, 2);
        assert_eq!(state.history()[2].state, 3);
    }

    #[test]
    fn test_rollback_to_previous_state() {
        let mut state = VersionedState::new(100);
        state.update(200);
        state.update(300);

        assert_eq!(*state.get(), 300);
        assert_eq!(state.version(), 2);

        // Rollback once
        let success = state.rollback();
        assert!(success);
        assert_eq!(*state.get(), 200);
        assert_eq!(state.version(), 1);

        // Rollback again
        let success = state.rollback();
        assert!(success);
        assert_eq!(*state.get(), 100);
        assert_eq!(state.version(), 0);
    }

    #[test]
    fn test_rollback_with_no_history() {
        let mut state = VersionedState::new(42);

        let success = state.rollback();
        assert!(!success);
        assert_eq!(*state.get(), 42);
        assert_eq!(state.version(), 0);
    }

    #[test]
    fn test_restore_to_specific_snapshot() {
        let mut state = VersionedState::new(1);
        state.update(2);
        state.update(3);

        let snapshot_at_2 = state.history()[1].clone();

        state.update(4);
        state.update(5);

        assert_eq!(*state.get(), 5);

        state.restore(snapshot_at_2);
        assert_eq!(*state.get(), 2);
        assert_eq!(state.version(), 1);
    }

    #[test]
    fn test_history_size_limit() {
        let mut state = VersionedState::new(0);

        // Add more than MAX_HISTORY updates
        for i in 1..=60 {
            state.update(i);
        }

        // Should be capped at MAX_HISTORY
        assert_eq!(state.history().len(), MAX_HISTORY);

        // Oldest snapshot should be state 10 (we keep snapshots 10-59, which is 50 snapshots)
        assert_eq!(state.history()[0].state, 10);
    }

    #[test]
    fn test_clear_history() {
        let mut state = VersionedState::new(1);
        state.update(2);
        state.update(3);

        assert_eq!(state.history().len(), 2);

        state.clear_history();
        assert_eq!(state.history().len(), 0);

        // Current state should be unchanged
        assert_eq!(*state.get(), 3);
        assert_eq!(state.version(), 2);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut state = VersionedState::new("hello".to_string());
        state.update("world".to_string());

        let json = serde_json::to_string(&state).expect("Failed to serialize");
        let deserialized: VersionedState<String> =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(deserialized.get(), state.get());
        assert_eq!(deserialized.version(), state.version());
        assert_eq!(deserialized.history().len(), state.history().len());
    }

    #[test]
    fn test_complex_state_type() {
        #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
        struct ComplexState {
            count: i32,
            items: Vec<String>,
            active: bool,
        }

        let initial = ComplexState {
            count: 0,
            items: vec![],
            active: false,
        };

        let mut state = VersionedState::new(initial);

        state.update(ComplexState {
            count: 5,
            items: vec!["a".to_string(), "b".to_string()],
            active: true,
        });

        assert_eq!(state.get().count, 5);
        assert_eq!(state.get().items.len(), 2);
        assert!(state.get().active);

        state.rollback();
        assert_eq!(state.get().count, 0);
        assert_eq!(state.get().items.len(), 0);
        assert!(!state.get().active);
    }

    #[test]
    fn test_snapshot_preserves_state() {
        let mut state = VersionedState::new(vec![1, 2, 3]);
        let snapshot1 = state.snapshot();

        state.update(vec![4, 5, 6]);
        let snapshot2 = state.snapshot();

        // Original snapshot should still have original data
        assert_eq!(snapshot1.state, vec![1, 2, 3]);
        assert_eq!(snapshot2.state, vec![4, 5, 6]);
    }
}
