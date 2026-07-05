# Systems (`src/core/system/`)

A sub-system of Pillar 4. Systems are the long-running, continuous logic observers of the framework.

## Architectural Role

If Nodes are data and Commands are discrete actions, Systems are the "Daemons" of the application. They handle continuous background processes like search indexing, AI agent loops, autosave timers, and layout calculations. 

Systems sit at the boundary between the Event Bus and the Scheduler. They listen for Signals (to know *when* to wake up), query the Graph (to know *what* to work on), and schedule Tasks via the Scheduler (to do the actual heavy lifting). 

## Design Decisions

- **Strict Read-Only Execution:** Systems are allowed to read the graph and schedule background work, but they are strictly forbidden from mutating the graph directly. If a System determines that a state change is required (e.g., the Search Indexer found a broken link), it must dispatch a Command. This preserves the Undo/Redo history and transactional safety.
- **Filter-Execute Pattern:** To prevent every System from waking up on every single signal, the `System` trait implements a `filter` method. The `SystemRegistry` calls `filter` first. Only if it returns `true` does the system's `execute` logic run, saving CPU cycles.
- **Dynamic Registration:** Systems are registered dynamically at startup by the core framework or by plugins. This allows a plugin (like an AI Assistant) to inject its own background observer into the engine without modifying the core code.

## Files

- **`interface.rs`**: Defines the `System` trait. Implementors must provide a `name`, a `filter` function (to determine if a signal is relevant), and an `execute` function (to schedule work via the `Scheduler`).
- **`registry.rs`**: `SystemRegistry` holds all active `System` instances. It exposes a `route_signal` method which is called by the application's main loop to efficiently fan-out incoming signals to all interested systems.
