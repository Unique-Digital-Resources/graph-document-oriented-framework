//! Per-node metadata: tags, timestamps, version, dirty flag.
//!
//! `Metadata` is **not** the same as `Properties`. Properties are
//! domain data declared in the DSL (`title`, `isCompleted`).
//! Metadata is framework bookkeeping that every node has regardless
//! of type.

use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Wall-clock creation time. Set once by `Metadata::now()`.
    pub created_at: DateTime<Utc>,
    /// Wall-clock last-mutation time. Bumped by `touch()`.
    pub modified_at: DateTime<Utc>,
    /// Free-form string tags. Indexed by `core/graph/index.rs` (Phase 1).
    pub tags: HashSet<String>,
    /// Monotonic version counter, bumped on every `touch()`. Used by
    /// the scheduler (Phase 3) to detect stale reads and by the
    /// history stack (Phase 2) to detect conflicts.
    pub version: u64,
    /// `true` if the node has unsaved mutations. Cleared by the
    /// serializer (Phase 5) after a successful write.
    pub dirty: bool,
}

impl Default for Metadata {
    fn default() -> Self {
        Self::now()
    }
}

impl Metadata {
    /// Construct with `created_at == modified_at == now`, `version == 1`.
    pub fn now() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            modified_at: now,
            tags: HashSet::new(),
            version: 1,
            dirty: false,
        }
    }

    /// Bump `modified_at` and `version`, set `dirty = true`.
    /// Called by the Command pipeline after a mutation lands.
    pub fn touch(&mut self) {
        self.modified_at = Utc::now();
        self.version = self.version.saturating_add(1);
        self.dirty = true;
    }

    /// Mark the node as persisted. Called by the serializer after a
    /// successful save.
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    // ---- tag API ----------------------------------------------------

    pub fn add_tag(&mut self, tag: impl Into<String>) -> bool {
        self.tags.insert(tag.into())
    }

    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    pub fn tags(&self) -> impl Iterator<Item = &str> {
        self.tags.iter().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_initializes_consistently() {
        let m = Metadata::now();
        assert_eq!(m.created_at, m.modified_at);
        assert_eq!(m.version, 1);
        assert!(!m.dirty);
        assert!(m.tags.is_empty());
    }

    #[test]
    fn touch_advances_version_and_dirty() {
        let mut m = Metadata::now();
        let v0 = m.version;
        std::thread::sleep(std::time::Duration::from_millis(2));
        m.touch();
        assert_eq!(m.version, v0 + 1);
        assert!(m.modified_at > m.created_at);
        assert!(m.dirty);
    }

    #[test]
    fn mark_clears_dirty_flag() {
        let mut m = Metadata::now();
        m.touch();
        assert!(m.dirty);
        m.mark_clean();
        assert!(!m.dirty);
    }

    #[test]
    fn tags_dedupe_and_remove() {
        let mut m = Metadata::now();
        assert!(m.add_tag("urgent"));
        assert!(!m.add_tag("urgent")); // second insert returns false
        assert!(m.has_tag("urgent"));
        assert!(m.remove_tag("urgent"));
        assert!(!m.has_tag("urgent"));
    }

    #[test]
    fn serde_round_trip_preserves_timestamps_and_tags() {
        let mut m = Metadata::now();
        m.add_tag("x");
        m.touch();

        let json = serde_json::to_string(&m).unwrap();
        let back: Metadata = serde_json::from_str(&json).unwrap();
        assert_eq!(back.created_at, m.created_at);
        assert_eq!(back.modified_at, m.modified_at);
        assert_eq!(back.version, m.version);
        assert!(back.has_tag("x"));
    }
}