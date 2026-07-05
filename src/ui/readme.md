# UI Layer (`src/ui/`)

The `ui/` directory contains the user interface representation for the framework. It is strictly divided into two independent layers: Abstract Headless State (`headless/`) and Platform Rendering (`web/`).

## Architectural Role

The primary design philosophy of this module is that **the UI is a dumb view**. It holds zero business logic. It does not execute commands directly, nor does it mutate the document graph. 

Instead, it reads abstract UI state computed by the framework, turns that state into pixels (DOM elements or Canvas draw calls), and routes raw user inputs (mouse clicks, keystrokes) back to the framework's `CommandPipeline`. 

### Why a "Dumb View"?
By stripping the UI of all business logic, we achieve several critical architectural goals:
1. **Perfect Undo/Redo:** Because the UI cannot mutate data directly, every user action is forced through the Command Pipeline. This guarantees that every UI interaction is automatically recorded in the History Stack and can be undone.
2. **AI & Automation Parity:** An AI agent or a scripting engine interacts with the application by dispatching the exact same Commands as the UI. There is no separate "UI API" vs "Backend API".
3. **Headless Testing:** The entire application's UI state, layout, and focus traversal can be tested in pure Rust without launching a browser or a window.

### The Two-Phase UI Architecture
The UI layer is split into two distinct phases to separate platform-agnostic logic from platform-specific rendering:

- **Phase 6 (Headless):** Computes *what* should be shown and *where* it should be placed. It calculates layout bounds (x, y, width, height), tracks which item is selected, and knows which widget has keyboard focus. It produces zero pixels.
- **Phase 7 (Web):** Takes the headless state and maps it to actual screen elements. It generates a Virtual DOM tree for React/Svelte, or a list of Canvas Draw Calls for high-performance rendering. It also catches raw browser events and translates them into framework Commands.

## Unidirectional Data Flow

The UI layer participates in a strict, unbreakable cycle:

1. **User Input:** The user clicks a button in the browser.
2. **Dispatch:** The `input_bridge` translates this DOM click into a JSON payload and invokes the `CommandPipeline`.
3. **Mutation:** The Command mutates the Document Graph.
4. **Signal:** The Graph emits a `NodePropertyChanged` signal.
5. **Invalidation:** The Headless `LayoutSystem` hears the signal, queries the Document Graph, and marks the affected UI nodes as dirty.
6. **Re-layout:** The Scheduler runs the Layout System, which recomputes the `Bounds` of the UI nodes.
7. **Render:** The Web renderer reads the updated View Graph and repaints the screen.

## Module Breakdown

- **`headless/`**: The platform-agnostic, stateful representation of the user interface. It models UI elements as transient nodes in a "View Graph", calculates their layout bounds, and tracks focus and selection.
- **`web/`**: The rendering and input layer. Maps the headless View Graph to standard web technologies (Virtual DOM, Canvas Draw Calls) and translates browser DOM events into framework Commands.