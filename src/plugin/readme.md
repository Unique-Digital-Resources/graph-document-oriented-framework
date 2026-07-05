# Plugin System (`src/plugin/`)

Phase 4 of the framework. Allows third parties to extend the framework dynamically at runtime.

## Architectural Role

The framework is designed to be "closed for modification, open for extension." The core engine (`src/core/`) knows nothing about specific application features (like "Tasks", "MindMaps", or "CAD Tools"). It only understands nodes, commands, and graphs. 

The `plugin` module provides the infrastructure for loading external code, verifying that it is safe to run, and granting it restricted access to the engine. A plugin can register new Node types, new Commands, new Relations, and new background Systems.

## Design Decisions

- **Capability-Based Security:** Plugins run in a `Sandbox`. They must explicitly declare the permissions they need (e.g., `filesystem`, `network`) in their manifest. The framework checks these permissions at runtime before allowing the plugin to perform sensitive actions.
- **Deterministic Lifecycle:** Plugins cannot just execute code randomly. They are driven by a strict state machine (`Loaded -> Initialized -> Activated -> Deactivated -> Unloaded`). This ensures the framework can safely spin up a plugin's dependencies before the plugin itself starts.
- **Dependency Resolution:** Plugins can depend on other plugins. The framework topologically sorts all loaded plugins to guarantee that if Plugin A depends on Plugin B, Plugin B is initialized and activated first.

## Modules

- **`manager/`**: Handles the plugin state machine and resolves dependency graphs.
- **`permissions/`**: Defines the capability tokens and enforces them via a Sandbox.
- **`api/`**: Provides the restricted `PluginContext` (the `ctx` object) that plugins use to interact with the engine safely.