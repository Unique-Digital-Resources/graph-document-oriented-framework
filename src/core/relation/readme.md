# Relations (`src/core/relation/`)

Pillar 4 (Data Model) of the framework. Defines the grammar of how nodes connect.

## Architectural Role

Relations are not just pointers between nodes; they are behavioral contracts. When a developer declares `subtasks: relation CHILDREN -> TaskNode`, the framework uses the relation's schema to automatically manage cascading deletes, cycle detection, dirty propagation, and memory lifecycle.

This module separates relations into two distinct layers: **Low-Level Dimensions** (the grammar) and **High-Level Presets** (the vocabulary).

## Design Decisions

- **Orthogonal Dimensions:** Instead of having a single enum that tries to describe every aspect of a relation (e.g., `TreeOwnership`, `DagReference`), relations are broken down into 7 independent axes (Topology, Cardinality, Propagation, etc.). This makes the system infinitely extensible.
- **Presets as Macros:** Developers should never have to spell out 7 enums every time they define a relation field. Instead, they use a high-level name like `CHILDREN`. The framework resolves this name to a `RelationSchema` (a bundle of the 7 enums). 
- **Plugin Extensibility:** Plugins can define their own custom presets (e.g., `BLOCKED_BY`) by combining the low-level enums and registering them in the `RelationPresetRegistry`.

## Files

- **`enums.rs`**: Defines the 7 orthogonal axes of relation behavior:
  - `Topology`: Tree (strict hierarchy), DAG (acyclic), Graph (cycles allowed).
  - `Cardinality`: 1:1, 1:N, N:M.
  - `Propagation`: None, Forward, Backward, Bidirectional (controls dirty mark flow).
  - `Evaluation`: Immediate, Deferred, Lazy, Async (controls when dependents recompute).
  - `Lifetime`: Persistent, Cancelable, Replaceable, Weak (controls edge strength).
  - `Persistence`: Saved, Transient, Derived (controls if the edge itself is serialized).
  - `Ownership`: None, Containment, Shared, Borrowed (controls cascade-delete and copy semantics).
- **`presets.rs`**: Defines `RelationSchema` (the frozen bundle of the 7 enums) and the `RelationSchemaBuilder`. It pre-defines the core presets (`CHILDREN`, `REFERENCE`, `DEPENDENCY`, `BLOCKED_BY`) exactly as specified by the DSL specification. It also houses the `RelationPresetRegistry`, the symbol table the DSL parser uses to resolve preset names at runtime.