# Web UI (`src/ui/web/`)

The rendering and input layer for web-based frontends. It acts as the final boundary between the Rust framework and the user's screen and peripherals.

## Architectural Role

This module is the physical manifestation of the "Dumb View" philosophy. Its only jobs are:
1. **Read:** Query the Headless View Graph.
2. **Draw:** Turn the abstract UI nodes into actual pixels (DOM elements or Canvas draw calls).
3. **Catch:** Listen for raw user inputs (mouse clicks, keystrokes) from the browser.
4. **Dispatch:** Translate those inputs into JSON payloads and invoke the framework's `CommandPipeline`.

It contains absolutely zero business logic. It does not know what a "Task" is; it only knows how to draw a `Button` and forward a `click` event.

### Two Rendering Strategies

Because different applications have different rendering needs, this module provides two completely decoupled rendering strategies:

1. **Virtual DOM (`dom_mapper.rs`):** Generates a serializable JSON tree of `DomNode`s. This tree is sent to a frontend (like React, Vue, or Vanilla JS Web Components), which diffs it against the current browser DOM and applies the minimal necessary patches. This is ideal for standard forms, lists, and text-heavy apps.
2. **Canvas / WebGPU (`canvas_renderer/`):** Generates a flat `Vec<DrawCall>`. The frontend receives this list and executes the commands on an HTML5 `<canvas>` 2D context. This bypasses the browser's DOM layout engine entirely, offering massive performance for dense UIs (like infinite canvas node editors or CAD tools).

### The Unidirectional Input Loop

When running in a browser (either via WebSockets or in-memory via WebAssembly), the flow of data through this module is strictly unidirectional:

1. User clicks a `<button>` in the browser.
2. The `input_bridge` parses the browser's raw DOM event into a `DomEvent` JSON payload.
3. The `InputDispatcher` looks up the target `UiNodeId` in the View Graph.
4. It finds the `CommandId` bound to that widget and executes it via the `CommandPipeline`.
5. The graph mutates, the Headless UI Systems re-layout, and the Web UI generates a new Virtual DOM/Draw Calls to repaint the screen.

## Module Breakdown

- **`dom_mapper.rs`**: The Virtual DOM generator. Maps View Graph nodes to HTML tags and styles.
- **`js_bridge.rs`**: Contains the JavaScript bridge code as a string constant, allowing the Rust server to serve the frontend sync logic dynamically.
- **`canvas_renderer/`**: The alternative Canvas/WebGPU renderer. Generates flat lists of drawing commands.
- **`input_bridge/`**: The event router. Parses DOM events and invokes the `CommandPipeline`.