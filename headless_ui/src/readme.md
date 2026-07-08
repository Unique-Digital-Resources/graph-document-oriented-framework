# Headless UI (`src/ui/headless/`)

The `headless/` module introduces a stateful, platform-agnostic UI representation into the graph. It models the structure, layout, and interaction state of the user interface without rendering a single pixel.

## Architectural Role & Invariants

This module bridges the gap between pure document data and visual representation. To maintain the integrity of the 5-pillar architecture, it enforces the following strict invariants:

### 1. Strict Transience
All UI nodes are marked `Transient`. A document saved to disk contains zero UI state. When an application closes, the entire View Graph is destroyed. This prevents UI layout details (like scroll position or window size) from polluting the user's data model. UI state is rebuilt dynamically from the document graph on load.

### 2. The View Graph Boundary (One-Way Binding)
The framework maintains a strict, one-directional boundary between the Document Graph and the View Graph:
- **UI Nodes reference Document Nodes:** A `TaskRowUI` node points to a `TaskNode` to read its `title` and `isCompleted` state.
- **Document Nodes NEVER reference UI Nodes:** A `TaskNode` has absolute zero knowledge of how it is being rendered. 

This is enforced by the `BindingRegistry`, which maintains a separate lookup table mapping `UiNodeId` to `NodeId`, rather than using the core `Graph`'s edge system (which could allow two-way edges). This keeps the data model pristine and allows a single document to be rendered by multiple different UIs simultaneously.

### 3. Graph-Native UI
The UI tree is not a custom data structure; it is managed as a standard instance of the core `Graph`, configured with `Topology::Tree` and the `CHILDREN` relation. This means the UI inherits O(1) lookups, bidirectional edge tracking, and cycle validation for free. Attaching a UI node to itself is structurally impossible.

### 4. Layout as a Background System
UI layout is not calculated synchronously during a command execution. Instead, the `LayoutSystem` implements the framework's `System` trait. When a document node changes, the layout system is notified via a Signal, marks the dependent UI nodes as "dirty", and schedules a layout pass with the `Scheduler`. The actual bounds (x, y, width, height) are computed asynchronously on the next scheduler tick.

## Mental Model: How a UI is Built

1. A plugin or the core application defines UI Widgets (`ButtonNode`, `ListViewNode`).
2. These widgets are inserted into the `ViewGraph` and attached via parent/child edges.
3. The `BindingRegistry` links a `ButtonNode` to a specific `TaskNode` in the document graph.
4. When the document changes, the `LayoutSystem` recalculates the `Bounds` of the button.
5. The `SelectionState` and `FocusState` track whether the button is currently highlighted or focused.
6. A renderer (from Phase 7) reads this abstract state and draws the button on screen.

## Module Breakdown

- **`nodes/`**: Defines the base `UiNode` wrapper and specific widget types (`ButtonNode`, `ListViewNode`, `TextFieldNode`, etc.). Widgets are dumb data structs holding UI-specific properties.
- **`view_graph/`**: Contains `ViewGraph` (the separate subgraph instance for UI nodes) and `BindingRegistry` (the one-way mapping layer between UI and Document).
- **`layout/`**: The `LayoutSystem`. Listens for graph changes and recursively calculates `Bounds` (x, y, width, height) for UI nodes based on layout rules (Row, Column, Grid).
- **`state/`**: Tracks transient interaction state, specifically `SelectionState` (which document nodes are selected) and `FocusState` (which UI node has keyboard focus). These emit immediate Signals when they change.