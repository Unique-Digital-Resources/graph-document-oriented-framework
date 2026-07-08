# The Dependency Graph (`src/core/graph/`)

Pillar 4 (Engine) of the framework. The `Graph` is the master source of truth for all document state.

## Architectural Role

While individual `Node`s hold their own property data and a local cache of their outgoing edges, they do not know about the global graph structure. The `Graph` module is the central authority that wires nodes together, enforces structural integrity, and provides the query engine that the UI, AI, and Systems use to read the document.

It is designed to handle millions of nodes by ensuring O(1) lookups for direct relations, utilizing secondary indexes for fast type/tag filtering, and strictly validating all structural changes *before* they are applied.

## Design Decisions

- **Bidirectional Edge Tracking:** When an edge is added (e.g., `A -> B` via `CHILDREN`), the graph updates the forward map (`A.children = [B]`) and the reverse map (`B.parents = [A]`). This allows O(1) lookups in both directions, which is critical for cascade-deletes (finding children) and dependency resolution (finding parents).
- **Preemptive Validation:** The `add_edge` operation is atomic. Before an edge is written to storage, it is passed to the `validation` module. If adding the edge would violate a `Topology` rule (like creating a cycle in a DAG) or a `Cardinality` rule (like giving a node two parents in a Tree), the operation is rejected entirely.
- **Separation of Storage and Queries:** The `Graph` struct itself only exposes basic `insert`, `remove`, `get`, and `add_edge` methods. High-level developer queries (like `find_children` or `find_by_type`) are isolated in the `queries` module, keeping the storage logic clean.

## Files

- **`storage.rs`**: The core `Graph` struct. Maintains the `HashMap` of nodes, forward/reverse edge maps, and the secondary indexes. Handles node insertion, deletion (including cascading cleanup of dangling edges), and edge management.
- **`validation.rs`**: The rule enforcer. Checks `Cardinality` (e.g., 1:1 means a target can only have one source) and `Topology` (uses DFS to ensure adding an edge doesn't create a cycle in a Tree or DAG).
- **`traversal.rs`**: Standard graph algorithms (BFS, DFS, Topological Sort) used by the query engine and background systems to walk the document structure.
- **`index.rs`**: `GraphIndex` maintains `HashMap`s from `TypeId` -> `HashSet<NodeId>` and `Tag` -> `HashSet<NodeId>`. This prevents the engine from having to scan every node when a user queries `find_by_type("TaskNode")`.
- **`queries.rs`**: The `GraphQuery` API. A read-only wrapper that exposes developer-friendly methods like `children(id)`, `find_ancestors(id)`, and `find_by_tag(tag)`.