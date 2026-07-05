# Layout System (`src/ui/headless/layout/`)

Calculates the geometric bounds (`x`, `y`, `width`, `height`) of UI nodes within the View Graph. It translates abstract tree structures into spatial rectangles that a renderer can draw.

## Architectural Role

In many frameworks, layout is a synchronous, deeply nested function call that blocks the main thread. In the Graph Document Framework, layout is treated as a **background `System`**. 

This module implements the framework's `System` trait. It does not run continuously; instead, it reacts to graph mutations. When a UI node is added, removed, or a bound Document Node changes its properties, the `LayoutSystem` is notified via Signals, queues a layout pass, and lets the `Scheduler` decide when to execute it.

### The Layout Algorithm

The layout pass is a recursive, top-down algorithm:
1. **Root Assignment:** The system queries the `ViewGraph` for its root node and assigns it the full viewport size (e.g., `1920x1080`).
2. **Space Distribution:** It recursively visits children. For `Container` nodes, it divides the available space based on the `ContainerLayout` rule:
   - `Row`: Divides width equally among children.
   - `Column`: Divides height equally among children.
   - `Grid`: Distributes space into a square grid.
3. **Leaf Sizing:** Leaf widgets (like `Button` or `Label`) take the width provided by their parent, but often demand a fixed height (e.g., `24.0` pixels).
4. **Committing Bounds:** Once the pure math is calculated, the system locks the `ViewGraph` (via interior mutability `Arc<Mutex>`) and writes the final `Bounds` struct into each `UiNode`.

### Scheduler Integration

Layout is critical for user experience. If a user clicks a button, the UI must re-layout before the screen repaints so the click hits the correct target. Therefore, the `LayoutSystem` schedules its work with `Priority::High` (or `Immediate`), ensuring the `Scheduler` processes it before the next render frame.

## Key Files

- **`system.rs`**: The `LayoutSystem` struct implementing the `System` trait, including the recursive bounds calculation logic.