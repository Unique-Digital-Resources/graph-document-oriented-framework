# View Graph (`src/ui/headless/view_graph/`)

Contains the storage layer for the headless UI and the strict boundary that connects it to the Document Graph.

## Architectural Role

The View Graph is the structural representation of the user interface at a given moment in time. It defines what widgets exist, how they are nested (parent/child relationships), and what document data they are observing.

### Reusing the Core Graph Engine

Instead of building a custom tree data structure for the UI, the `ViewGraph` is simply a standard instance of the core `Graph`, pre-configured with the `CHILDREN` relation and `Topology::Tree`.

This architectural reuse provides massive benefits:
1. **Free Validation:** Because the UI uses the core graph engine, it is impossible to create a cyclic UI layout (e.g., attaching a window to a button inside that window). The core `validation.rs` module rejects it.
2. **Free Traversal:** The `LayoutSystem` and `FocusState` can use the core graph's `get_targets` and `get_sources` to walk the UI tree with O(1) lookups.
3. **Containment Semantics:** Because the `CHILDREN` relation implies `Ownership::Containment`, deleting a UI container automatically cleans up its children in memory.

### The Binding Registry: The One-Way Boundary

The most critical rule of the UI architecture is: **Document Nodes never reference UI Nodes.** If they did, saving a document would pollute the file with transient UI state, and deleting a UI panel would risk deleting document data.

To enforce this, the `BindingRegistry` exists as a completely separate side-table, rather than using edges in the core graph. 

- **Storage:** It maps a `UiNodeId` to a document `NodeId` (and an optional specific property).
- **Inverse Index:** It maintains a reverse lookup (`document_node -> Vec<UiNodeId>`) so that when a document node changes, the framework can instantly find all UI widgets that need to be repainted.
- **Purging:** When a document node is deleted, the registry's `purge_document_node` method is called, returning a list of UI nodes that must now be destroyed because their data source is gone.

## Key Files

- **`storage.rs`**: The `ViewGraph` struct, which wraps the core `Graph` and provides typed access to `WidgetKind` nodes.
- **`bindings.rs`**: The `BindingRegistry` and `Binding` struct, managing the strict, one-way reference from UI state to document data.