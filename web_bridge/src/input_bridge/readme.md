# Input Bridge (`src/ui/web/input_bridge/`)

The translation layer that catches raw browser events (mouse clicks, keystrokes) and routes them into the framework's `CommandPipeline`.

## Architectural Role

This module is the final piece of the "Dumb View" puzzle. When a user interacts with the UI, the frontend (whether DOM or Canvas) captures a raw browser event. It doesn't know what that event means in a business context—it only knows that an element with a specific ID was clicked.

The Input Bridge receives these raw events, gives them semantic meaning by looking up the target UI node, and executes the bound Command.

### The Dispatch Flow

1. **Event Capture:** In the browser, an event listener catches a `click` or `input` event.
2. **Serialization:** The frontend sends a minimal JSON payload over WebSocket (or directly to WASM memory) containing the `target` (UiNodeId), `type` (e.g., "click"), and optional `value` (for text inputs).
3. **Parsing (`listeners.rs`):** The `EventListener` parses this JSON into a typed `DomEvent` enum.
4. **Dispatching (`dispatcher.rs`):** The `InputDispatcher` takes the `DomEvent` and queries the `ViewGraph` for the target `WidgetKind`.
5. **Command Execution:** 
   - If the widget is a `ButtonNode`, it reads the bound `command_id` and `command_params`.
   - If the widget is a `TextFieldNode`, it maps the input to a generic `SetProperty` command, targeting the bound Document Node.
   - It then invokes the `CommandPipeline`, executing the mutation safely within a transaction.

### Parameter Flattening

A critical detail of the `InputDispatcher` is how it handles parameters. UI widgets store their command parameters using the framework's internal `PropertyValue` enum (which includes tags for serialization). 

However, end-developers writing Command logic expect clean, primitive JSON values (like `"taskId": "uuid-string"`). The `InputDispatcher` contains logic to "flatten" `PropertyValue::Uuid(u)` into a standard JSON string before passing it to the `CommandPipeline`. This ensures the developer experience (DX) for writing commands remains clean and intuitive, completely hiding the framework's internal property tags.

## Key Files

- **`listeners.rs`**: Defines the `DomEvent` enum and the `EventListener` responsible for parsing incoming JSON payloads from the browser.
- **`dispatcher.rs`**: The `InputDispatcher` that maps UI events to their bound Commands and triggers the `CommandPipeline`.