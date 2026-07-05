//! Signal data structures and emission timing categories.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// When the EventBus should dispatch this signal to listeners.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmitTiming {
    /// Dispatched immediately on emission (e.g., UI selection changes).
    Immediate,
    /// Dispatched on the next scheduler tick (e.g., background index updates).
    Deferred,
    /// Dispatched only after the current Command transaction commits successfully.
    Transactional,
    /// Dispatched to a background thread pool (Phase 3).
    Async,
}

/// A notification that something happened in the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// e.g., "NodeCreated", "PropertyChanged", "CommandExecuted"
    pub signal_type: String,
    /// The NodeId that originated the event, if applicable.
    pub source: Option<Uuid>,
    /// Polymorphic payload (e.g., `{"property": "title", "old": "A", "new": "B"}`)
    pub payload: Value,
    pub timestamp: DateTime<Utc>,
    pub timing: EmitTiming,
}

impl Signal {
    pub fn new(signal_type: impl Into<String>, timing: EmitTiming) -> Self {
        Self {
            signal_type: signal_type.into(),
            source: None,
            payload: Value::Null,
            timestamp: Utc::now(),
            timing,
        }
    }

    pub fn with_source(mut self, source: Uuid) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_payload(mut self, payload: Value) -> Self {
        self.payload = payload;
        self
    }
}