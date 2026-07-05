//! Task cancellation logic for Replaceable lifetimes.

use std::collections::HashMap;
use uuid::Uuid;

pub struct CancellationManager {
    /// Maps `task_type` to the currently active `task_id`.
    active_replaceable: HashMap<String, Uuid>,
}

impl Default for CancellationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationManager {
    pub fn new() -> Self {
        Self {
            active_replaceable: HashMap::new(),
        }
    }

    /// Registers a replaceable task. Returns the old task_id if one exists,
    /// so the scheduler can remove it from the queue.
    pub fn replace(&mut self, task_type: &str, new_task_id: Uuid) -> Option<Uuid> {
        self.active_replaceable.insert(task_type.to_string(), new_task_id)
    }

    /// Removes a task from active tracking (e.g., after it completes).
    pub fn complete(&mut self, task_type: &str) {
        self.active_replaceable.remove(task_type);
    }
}