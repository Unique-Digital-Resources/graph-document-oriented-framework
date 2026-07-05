# UI State (`src/ui/headless/state/`)

Tracks transient, non-persisted interaction state that affects how the UI is rendered, but does not alter the document data itself.

## Architectural Role

This module manages the "ephemeral" state of the user interface. It answers questions like: *What is currently highlighted?* or *Which input field has the blinking cursor?*

### Transient State vs. Document Mutation

A critical rule enforced here is that **changing UI state does not mutate the Document Graph**. 
- If a user clicks a `TaskNode` in a list to select it, the `TaskNode` itself is not modified. 
- Instead, the `SelectionState` simply records the `NodeId` in an internal `HashSet`.
- When the state changes, it emits an `Immediate` `Signal` (`SelectionChanged` or `FocusChanged`). 
- The renderer hears this signal and repaints the list row with a blue highlight.

This strict separation ensures that selecting an item never creates an Undo/Redo history step, and never marks the document as "dirty" for saving.

### Focus Traversal (Tab Navigation)

Keyboard focus is handled by the `FocusState` struct. It implements standard Tab / Shift-Tab traversal semantics:
1. It queries the `ViewGraph` to perform a Depth-First Search (DFS) starting from the root.
2. It filters the tree, collecting only nodes that are `is_interactive()` (visible + enabled) and have a focusable `UiNodeRole` (e.g., `Button`, `TextInput`).
3. When `focus_next()` is called, it finds the currently focused node in the cached DFS list and moves focus to the next index, wrapping around if necessary.

### Lifecycle Cleanup

Both state subsystems are defensive about graph mutations. If a Document Node or a UI Node is deleted, the `on_node_deleted` hooks are called. 
- If a selected document is deleted, it is removed from the selection set.
- If a focused UI widget is removed, focus automatically shifts to the next available focusable node in the View Graph, ensuring the keyboard is never "trapped" on a deleted element.

## Key Files

- **`selection.rs`**: `SelectionState`. Tracks a `HashSet` of selected document nodes, handles single-select, multi-select (toggle/extend), and primary anchor tracking.
- **`focus.rs`**: `FocusState`. Tracks the currently focused `UiNodeId` and provides DFS-based Tab traversal logic.