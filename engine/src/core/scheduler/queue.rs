//! Priority task queue and Scheduler execution loop.

use std::collections::BinaryHeap;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::graph::Graph;
use crate::core::relation::Lifetime;

use super::cancellation::CancellationManager;

/// Task priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Runs on the next `tick()` call. (e.g., UI layout updates)
    Immediate = 3,
    /// Runs after all Immediate tasks are done.
    High = 2,
    /// Background work (e.g., search indexing)
    Normal = 1,
    /// Runs only if nothing else is queued.
    Low = 0,
    /// Runs when the system is completely idle.
    Idle = -1,
}

/// The function executed by the task. 
/// Takes an immutable graph reference for read-only queries.
pub type TaskFn = Arc<dyn Fn(&Graph) + Send + Sync>;

/// A schedulable unit of work.
pub struct Task {
    pub id: Uuid,
    pub priority: Priority,
    pub task_type: String, // e.g., "ThumbnailGenerator"
    pub lifetime: Lifetime,
    pub execute: TaskFn,
}

// Implement PartialEq, Eq, PartialOrd, Ord for BinaryHeap to sort by priority.
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
impl Eq for Task {}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse because BinaryHeap is a max-heap, and we want highest priority first.
        Some(self.priority.cmp(&other.priority).reverse())
    }
}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Scheduler {
    queue: BinaryHeap<Task>,
    cancellation: CancellationManager,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            cancellation: CancellationManager::new(),
        }
    }

    /// Schedule a new task. If the task is `Replaceable`, it cancels
    /// any existing task of the same `task_type`.
    pub fn schedule(&mut self, task: Task) {
        if task.lifetime == Lifetime::Replaceable {
            if let Some(old_task_id) = self.cancellation.replace(&task.task_type, task.id) {
                self.cancel_task(old_task_id);
            }
        }
        self.queue.push(task);
    }

    /// Remove a task from the queue. (O(N) lookup, acceptable for Phase 3).
    fn cancel_task(&mut self, task_id: Uuid) {
        let remaining: Vec<Task> = self.queue.drain().filter(|t| t.id != task_id).collect();
        self.queue.extend(remaining);
    }

    /// Explicitly cancel a task by its ID.
    pub fn cancel(&mut self, task_id: Uuid) {
        self.cancel_task(task_id);
    }

    /// Process all `Immediate` and `High` priority tasks currently in the queue.
    /// In a full engine, this would dispatch to a thread pool.
    pub fn tick(&mut self, graph: &Graph) {
        let mut to_execute = Vec::new();
        let mut remaining = Vec::new();

        while let Some(task) = self.queue.pop() {
            if task.priority >= Priority::High {
                to_execute.push(task);
            } else {
                remaining.push(task);
            }
        }

        // Put lower priority tasks back
        self.queue.extend(remaining);

        // Execute the high-priority tasks
        for task in to_execute {
            (task.execute)(graph);
        }
    }

    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}