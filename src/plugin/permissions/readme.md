# Permissions (`src/plugin/permissions/`)

Capability checking and sandboxing for untrusted plugin code.

## Architectural Role

If a plugin could access the host OS freely, a malicious plugin could delete files or leak network data. The `permissions` module acts as the framework's border control. 

It translates human-readable strings from a plugin's manifest (`"permissions": ["filesystem"]`) into typed, machine-checkable tokens, and enforces them at runtime whenever the plugin attempts to interact with the outside world.

## Design Decisions

- **Fail-Fast Enforcement:** Instead of silently ignoring unauthorized actions, the `Sandbox` returns a `PermissionDenied` error immediately. This forces plugin developers to catch missing permissions during development rather than shipping silent bugs.
- **Decoupled from OS:** This module only defines the *policy* (what is allowed). The actual *enforcement* (e.g., blocking a file read system call) is deferred to the `io/persistence` and `io/rpc` layers, keeping the plugin module pure and testable.
- **Centralized Registry:** A plugin's permissions are registered once when it is loaded. Checking a permission at runtime is an O(1) `HashMap` lookup, adding zero overhead to hot paths.

## Files

- **`policy.rs`**: Defines the `Permission` enum (Filesystem, Network, Clipboard, Database, Secrets, Commands) and the `PermissionDenied` error struct.
- **`sandbox.rs`**: The `Sandbox` struct. Stores active permissions per plugin ID. Exposes the `check` method used by the `PluginContext` to validate actions before execution.