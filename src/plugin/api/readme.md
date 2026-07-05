# Plugin API (`src/plugin/api/`)

The restricted interfaces plugins use to interact with the core engine.

## Architectural Role

If plugins were handed a mutable reference to the `Graph` directly, they could bypass the Command Pipeline, break Undo/Redo history, and violate the strict architectural invariants. 

The `api` module solves this by providing a `PluginContext` (often referred to as `ctx` in the DSL). This context object is a tightly controlled wrapper around the engine. It gives plugins the tools they need (querying data, emitting signals, scheduling tasks) while physically preventing them from mutating state directly or accessing unauthorized OS resources.

## Design Decisions

- **Read-Only Graph Access:** The `PluginContext` only holds an immutable reference (`&Graph`) to the document. To mutate data, the plugin must dispatch a Command.
- **Integrated Sandbox:** The context holds a reference to the `Sandbox`. Any action that touches the outside world (like reading a file) must pass through the context's helper methods, which check permissions first.
- **Controlled Mutability:** While the graph is read-only, plugins *are* allowed to emit Signals and schedule background Tasks during their initialization phase. The context provides mutable references to the `EventBus` and `Scheduler` to enable this.

## Files

- **`context.rs`**: Defines the `PluginContext` struct. It bundles references to the `Graph`, `Sandbox`, `CommandRegistry`, `EventBus`, and `Scheduler`. It provides safe, sandboxed helper methods (like `try_read_file`) that enforce the rules before executing logic.