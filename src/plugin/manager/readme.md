# Plugin Manager (`src/plugin/manager/`)

Handles the plugin lifecycle state machine and dependency resolution.

## Architectural Role

Loading untrusted or third-party code into a core engine is risky. The `manager` module mitigates this by enforcing a strict, deterministic lifecycle. It prevents plugins from accessing engine resources before their dependencies are ready, and ensures clean teardown when the application shuts down or a plugin is unloaded.

## Design Decisions

- **Topological Loading:** If Plugin A requires Plugin B, loading A first would cause a crash. The `dependencies` module performs a Depth-First Search (DFS) to topologically sort all manifests, guaranteeing the correct initialization order.
- **Cycle Detection:** If a developer accidentally creates a circular dependency (A depends on B, B depends on A), the framework catches this at load time and rejects the plugins rather than entering an infinite loop.
- **State Isolation:** The `PluginManager` holds plugins in a `HashMap` alongside their current `PluginState`. A plugin's `initialize` or `activate` method is only called if it is in the correct preceding state, preventing double-initialization or activation of deactivated plugins.

## Files

- **`lifecycle.rs`**: Defines the `Plugin` trait, `PluginManifest`, and `PluginState` enum. The `PluginManager` struct exposes methods like `load`, `initialize_all`, `activate_all`, and `unload_all`, safely transitioning plugins between states.
- **`dependencies.rs`**: Contains the `resolve_plugin_order` function. Takes a list of `PluginManifest`s, builds a dependency graph, detects cycles, and returns a safely-ordered `Vec` of plugin IDs.