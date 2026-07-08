# Source Root (`src/`)

The `src/` directory is the foundational layer of the Graph Document Framework. It encapsulates the entire backend engine and headless UI representation as a pure Rust library crate. 

## Architectural Role

The primary design philosophy of this crate is **absolute separation of concerns**. This library contains zero direct OS-specific windowing logic, and zero business logic hardcoded for any specific application (like a text editor or a CAD tool). 

Instead, it provides a general-purpose, graph-oriented application platform. It enforces the "5-Pillar Architecture":
1. **Nodes:** Pure data containers.
2. **Signals:** Fact-based, past-tense notifications.
3. **Commands:** Intent-based, future-tense mutators.
4. **Dependency Graph:** The relational engine and source of truth.
5. **Plugin System:** Runtime extensibility.

By keeping this crate headless, the framework can be compiled for any target: a desktop app, a headless web server, a CLI tool, or entirely in the browser via WebAssembly.

## Top-Level Modules

- **`core/`**: The heart of the engine. Contains the graph storage, node definitions, command execution pipeline, history (undo/redo), and the background scheduler. It has no knowledge of external files, networks, or dynamic plugins.
- **`scripting/`**: The declarative scripting engine. Contains a custom Lexer and Parser that translate human-readable DSL scripts (e.g., `define node TaskNode...`) into abstract syntax trees, and a Compiler that registers these definitions into the `core/` registries.
- **`plugin/`**: The extension manager. Handles the lifecycle (load, activate, unload) of dynamic plugins, resolves dependency graphs, and enforces security sandboxes (e.g., preventing a plugin from accessing the filesystem without permission).
- **`io/`**: The Input/Output boundary. Bridges the isolated `core/` engine with the outside world. Handles serializing the graph to disk (supporting both JSON and high-performance Binary/MessagePack formats) and exposes the engine's Command Registry and Graph Queries over a mock RPC/HTTP API.
- **`ui/`**: The UI layer, strictly divided into Headless state (`ui/headless/`) and Web rendering interfaces (`ui/web/`). The headless layer computes layout, focus, and selection without pixels. The web layer generates Virtual DOM or Canvas draw calls and routes input events back to the Command Pipeline.
- **`wasm_api.rs`**: The WebAssembly bridge. Exposes a lightweight, global API (`init_app`, `get_ui_state`, `handle_dom_event`) allowing the entire framework to run client-side in the browser memory, communicating instantly with JS Web Components without network latency.

## Key File

- **`lib.rs`**: The crate entry point. It simply declares the public modules, establishing the public API surface that external consumers (like UI layers, server apps, or WASM frontends) will import.