# Nodes (`src/core/node/`)

Pillar 1 of the framework. Nodes are the atomic units of document data. 

## Architectural Role

In many traditional architectures, objects contain both data and behavior (methods). This framework strictly forbids that. A `Node` in this engine is a pure, dumb data container. It holds no logic for how to draw itself, how to validate its own business rules, or how to interact with other nodes. 

All behavior is externalized into Commands (Pillar 3) and Systems. This separation is what allows the framework to serialize millions of nodes to disk efficiently, serialize them over the network, and process them in background threads without encountering data races or complex inheritance hierarchies.

## Design Decisions

- **No Developer ID Management:** Developers never write `id: uuid::new()` in their DSL. The framework auto-generates UUIDs upon node creation and handles them entirely behind the scenes.
- **Polymorphic Properties:** Instead of compiling rigid structs for every node type (which would require recompilation), nodes use a `HashMap`-based property bag. This allows the DSL engine and plugins to define new node types dynamically at runtime.
- **Local Edge Cache:** While the global `Graph` holds the master list of edges, each `Node` holds a `LocalRelations` cache of its outgoing edges. This allows O(1) access to a node's direct children without querying the global graph, drastically speeding up traversals.

## Files

- **`node.rs`**: Defines the core `Node` struct, the `NodeId` (UUID alias), and `TypeId` (string newtype for runtime type resolution). It also houses `LocalRelations`, the per-node cache of outgoing edges.
- **`properties.rs`**: Defines the `Properties` bag and the `PropertyValue` enum. Properties are categorized by `PropertyKind`:
  - `Persistent`: Saved to disk (e.g., `title`).
  - `Transient`: Runtime-only (e.g., `isUISelected`).
  - `Computed`: Pure functions of other properties on the same node.
  - `Derived`: Functions of graph relations (e.g., `child_count`).
  - `Cached`: Memoized results of expensive operations (e.g., `thumbnail_hash`).
- **`metadata.rs`**: Framework bookkeeping that every node possesses regardless of its type. Includes `created_at`/`modified_at` timestamps, a monotonic `version` counter (used by the scheduler to detect stale reads), a `dirty` flag, and free-form string `tags` for fast querying.