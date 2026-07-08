# UI Nodes (`src/ui/headless/nodes/`)

Defines the data structures for the headless UI representation. This module provides the atomic building blocks (widgets) that populate the View Graph.

## Architectural Role

In the Graph Document Framework, a UI Node is not a DOM element, nor is it a native OS control. It is a pure, serializable data structure that represents *intent* and *state*. 

A `UiNode` acts as a strict wrapper around the core `Node` struct. By wrapping the core node rather than creating a parallel hierarchy, the UI layer leverages the same O(1) storage, indexing, and traversal mechanisms as the Document Graph, while enforcing entirely different lifecycle and persistence rules.

### Core Invariants of UI Nodes

1. **Forced Transience:** The `UiNode` wrapper overrides the core property insertion logic. Any property added to a UI node (e.g., `text`, `scroll_offset`, `is_hovered`) is automatically stamped with `PropertyKind::Transient`. This structurally guarantees that no UI state will ever accidentally leak into a saved document file.
2. **Type-Safe Identifiers:** UI nodes use a dedicated `UiNodeId` newtype (wrapping the standard `NodeId` UUID). This prevents UI node IDs and Document node IDs from being accidentally mixed up at API boundaries, which is critical when passing references between the View Graph and the Document Graph.
3. **Role-Based Classification:** Every `UiNode` has a `UiNodeRole` (e.g., `Container`, `Button`, `TextInput`). This coarse classification is used by the `LayoutSystem` to pick an appropriate layout algorithm (e.g., only `Container` nodes distribute space to children) and by the `FocusState` to determine if a node is eligible to receive keyboard focus.

### The "Dumb Widget" Philosophy

Concrete widgets (`ButtonNode`, `TextFieldNode`, `ListViewNode`, etc.) are intentionally dumb. They contain **zero behavior**. 

A `ButtonNode` does not handle the `onClick` event. It merely stores the `label` to be displayed and a reference to the `CommandId` that should be executed when clicked. The actual execution is deferred to the `input_bridge` and the `CommandPipeline`. 

This separation ensures that:
- Widgets are trivially serializable and testable.
- AI agents or scripts can trigger the exact same "click" action as a human user, simply by dispatching the command the button is bound to.
- The UI layer can be completely re-skinned (e.g., swapping DOM for Canvas) without touching widget definitions.

## Key Files

- **`ui_node.rs`**: Defines the base `UiNode` wrapper, `UiNodeId`, the `Bounds` geometry struct, and the `UiNodeRole` enum.
- **`widgets.rs`**: Defines specific widget structs and the `WidgetKind` tagged union used for polymorphic access within the View Graph's storage.