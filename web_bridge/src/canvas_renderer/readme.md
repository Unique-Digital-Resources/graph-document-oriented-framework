# Canvas Renderer (`src/ui/web/canvas_renderer/`)

An alternative rendering strategy that bypasses the browser's DOM entirely, generating a flat list of 2D drawing instructions for an HTML5 `<canvas>` or WebGPU context.

## Architectural Role

While the Virtual DOM (`dom_mapper.rs`) is excellent for standard applications (forms, lists, text), the DOM layout engine can become a bottleneck for highly dynamic, dense interfaces—like node-based editors, infinite canvases, or interactive maps.

The Canvas Renderer solves this by treating the UI as a paint program. Instead of generating a nested tree of HTML tags, the `Painter` traverses the View Graph and emits a flat `Vec<DrawCall>`. 

### How it Works

1. **Traversal:** The `Painter` starts at the root of the View Graph and recursively visits children.
2. **Geometry Mapping:** It reads the `Bounds` (x, y, width, height) computed by the Headless `LayoutSystem`.
3. **Command Generation:** Based on the `WidgetKind`, it generates specific drawing commands:
   - `Container` -> `StrokeRect` (to draw borders)
   - `Button` -> `FillRect` (blue background) + `FillText` (white label)
   - `Label` -> `FillText` (black text)
4. **Serialization:** The `Vec<DrawCall>` is serialized to JSON and sent to the frontend.
5. **Execution:** A lightweight JavaScript loop receives the array and executes the commands in order on a Canvas 2D context.

### Implications
Because this approach generates a flat list, there is no "diffing" algorithm. The frontend simply clears the canvas and executes the new list of commands every frame. This provides consistent, sub-millisecond rendering times regardless of how complex the UI tree becomes.

## Key Files

- **`draw_calls.rs`**: Defines the `DrawCall` enum (e.g., `ClearRect`, `FillRect`, `FillText`, `StrokeRect`). This is the serializable instruction set sent to the browser.
- **`painter.rs`**: The `Painter` struct that walks the `ViewGraph` and generates the `Vec<DrawCall>`.