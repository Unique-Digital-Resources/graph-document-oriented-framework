# Scheduler (`src/core/scheduler/`)

A sub-system of Pillar 4. The Scheduler acts as the engine's traffic controller, managing background work, priorities, and dirty state propagation.

## Architectural Role

When a document scales to hundreds of thousands of nodes, executing commands synchronously would freeze the application. If a user changes an image node, the framework shouldn't immediately regenerate the thumbnail and block the UI. 

Instead, the framework uses a **Reactive Invalidation, Pull-based Evaluation** model. When a command mutates a node, the graph reacts by marking dependent nodes as "Dirty" (Invalidation). The actual recomputation (the thumbnail generation) is deferred to the Scheduler, which decides *when* and *if* that work should happen based on system priorities.

## Design Decisions

- **Priority Queues:** Not all background work is equal. Updating the UI layout must happen immediately, while updating a search index can happen during idle time. The Scheduler uses a priority queue (`Immediate`, `High`, `Normal`, `Low`, `Idle`) to ensure critical work is processed first.
- **Task Cancellation (`Replaceable` Lifetime):** If a user rapidly types in a text field, triggering dozens of "Generate Thumbnail" tasks, the framework shouldn't queue all of them. The `CancellationManager` tracks tasks with a `Replaceable` lifetime. When a new task of the same type is scheduled, the older, obsolete task is automatically cancelled, saving CPU cycles.
- **Push-Back to Commands:** Systems and the Scheduler itself are strictly forbidden from mutating the graph directly. If a background task determines that state needs to change (e.g., the thumbnail finished generating and needs to be saved to a property), it must dispatch a new Command through the pipeline.

## Files

- **`queue.rs`**: The `Scheduler` struct and `Task` definition. Uses a `BinaryHeap` to order tasks by priority. Exposes a `tick()` method to drain immediate/high priority tasks, preparing the architecture for a future multi-threaded thread pool.
- **`dirty_propagation.rs`**: Contains the `propagate_dirty` function. Traverses the graph (following relation `Propagation` rules) to mark downstream or upstream nodes as dirty, returning a list of affected `NodeId`s for Systems to process.
- **`cancellation.rs`**: `CancellationManager` keeps a HashMap of active `Replaceable` tasks. It exposes a `replace` method that returns the old task ID so the `Scheduler` can remove it from the queue.