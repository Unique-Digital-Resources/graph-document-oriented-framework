# Core Engine (`src/core/`)

The `core/` module is the isolated, deterministic heart of the Graph Document Framework. It enforces the strict, unbreakable rules that govern all application state.

## Architectural Role & Invariants

This module is designed to be completely self-contained. It does not know about JSON serialization, network requests, or DSL parsing. It only understands in-memory data structures and strict execution pipelines.

The code here enforces the following architectural invariants:

1. **Strict Immutability:** The `Graph` is immutable except through the `CommandPipeline`. There is no `graph.add_node()` method available to the outside world; all mutations must go through a Command.
2. **Unidirectional Flow:** `UI/AI -> Command -> Mutates Graph -> Emits Signal -> System hears Signal -> System schedules work -> System dispatches new Command`.
3. **Reactive Invalidation, Pull-based Evaluation:** When a node changes, the `Graph` reacts by marking dependent nodes as "Dirty" (invalidation). It does *not* immediately recompute everything downstream. The `Scheduler` decides *when* the actual recomputation happens.
4. **Transactional History:** Every mutation is wrapped in a transaction. If a command fails mid-execution, the `Graph` is rolled back to its previous state via a snapshot. If it succeeds, it is pushed to the `HistoryStack` for Undo/Redo capabilities.
5. **High-Performance Bulk Operations:** To support 1,000,000+ node documents, the graph provides `with_capacity` for pre-allocation and `add_edge_unchecked` for rapid bulk loading from trusted save files, bypassing cycle validation during heavy imports.

## Module Breakdown

### Pillar 1: Data & Relations
- **`node/`**: Defines the atomic unit of data. A `Node` is a dumb data bag containing an auto-generated UUID, a Type ID, a property bag, local relation caches, and metadata (timestamps, version, dirty flag). Properties are strictly typed (Persistent, Transient, Computed, Derived, Cached) to control serialization and lifecycle.
- **`relation/`**: Defines how nodes connect. It separates relations into low-level orthogonal dimensions (`Topology`, `Cardinality`, `Ownership`, etc.) and high-level presets (`CHILDREN`, `REFERENCE`, `DEPENDENCY`). This allows the framework to automatically enforce rules, such as cascade-deleting children or preventing cyclic dependencies.

### Pillar 4: The Dependency Graph
- **`graph/`**: The master source of truth. Provides O(1) node lookups, bidirectional edge tracking, and secondary indexes (by Type and Tag). Crucially, it contains `validation.rs`, which ensures that no edge can be added if it violates the `RelationSchema` (e.g., preventing a Tree relation from having multiple parents). It also provides query APIs (`find_children`, `find_by_type`) and traversal algorithms (BFS, DFS, Topological Sort).

### Pillar 2 & 3: Mutation & Notification
- **`command/`**: The only mutator. The `CommandPipeline` takes a command ID and JSON parameters, looks up the command in the `registry/`, begins a transaction, executes the logic, emits transactional signals, and commits to history.
- **`history/`**: Manages the Undo/Redo stacks. Uses `GraphSnapshot` to capture state before mutations. Groups multiple commands into a single `Transaction` (Macro) so that complex user actions (like "Duplicate Object") only count as one Undo step.
- **`signal/`**: The notification system. The `EventBus` routes `Signal`s to listeners. Signals are categorized by `EmitTiming` (Immediate, Deferred, Async, Transactional) to ensure that UI updates happen instantly, but expensive background index updates only happen after a transaction safely commits.

### Pillar 4 (Sub-systems): Background Processing
- **`scheduler/`**: The task queue. Manages priorities (Immediate, High, Low, Idle) and handles dirty propagation. Crucially, it includes `cancellation.rs`, which allows the engine to cancel "Replaceable" tasks (e.g., if a user types rapidly, only the last thumbnail generation task is kept; the obsolete ones are cancelled).
- **`system/`**: Long-running background services (like a Search Indexer, AI Agent, or the UI Layout System). Systems implement the `System` trait, listen for specific Signals, query the Graph for data, and if they need to change state, they *must* dispatch a Command. They cannot mutate the graph directly.