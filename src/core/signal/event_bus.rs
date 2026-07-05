//! Publish/subscribe Event Bus.
//!
//! Listeners subscribe to a `signal_type`. The bus routes emissions
//! to listeners based on `EmitTiming`.

use std::collections::HashMap;
use std::sync::Arc;

use super::types::{EmitTiming, Signal};

/// A listener is a function that takes an immutable reference to the Signal.
pub type Listener = Arc<dyn Fn(&Signal) + Send + Sync>;

pub struct EventBus {
    // FIX: Removed the redundant inner Arc. Listener is already an Arc.
    listeners: HashMap<String, Vec<Listener>>,
    deferred_queue: Vec<Signal>,
    transactional_queue: Vec<Signal>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            deferred_queue: Vec::new(),
            transactional_queue: Vec::new(),
        }
    }

    /// Subscribe to a specific signal type.
    pub fn subscribe(&mut self, signal_type: &str, listener: Listener) {
        self.listeners
            .entry(signal_type.to_string())
            .or_default()
            .push(listener);
    }

    /// Emit a signal. Depending on its `EmitTiming`, it will be dispatched
    /// immediately or queued.
    pub fn emit(&mut self, signal: Signal) {
        match signal.timing {
            EmitTiming::Immediate => self.dispatch(&signal),
            EmitTiming::Deferred => self.deferred_queue.push(signal),
            EmitTiming::Transactional => self.transactional_queue.push(signal),
            EmitTiming::Async => {
                // Phase 3 will offload this to a thread pool via the Scheduler.
                // For now, we dispatch it immediately to avoid losing events.
                self.dispatch(&signal);
            }
        }
    }

    /// Dispatch all queued deferred signals.
    pub fn flush_deferred(&mut self) {
        let queue = std::mem::take(&mut self.deferred_queue);
        for signal in queue {
            self.dispatch(&signal);
        }
    }

    /// Dispatch all queued transactional signals.
    /// Called by the Command Pipeline after a successful commit.
    pub fn flush_transactional(&mut self) {
        let queue = std::mem::take(&mut self.transactional_queue);
        for signal in queue {
            self.dispatch(&signal);
        }
    }

    fn dispatch(&self, signal: &Signal) {
        if let Some(listeners) = self.listeners.get(&signal.signal_type) {
            for listener in listeners {
                listener(signal);
            }
        }
    }
}