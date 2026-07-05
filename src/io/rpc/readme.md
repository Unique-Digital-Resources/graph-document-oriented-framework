# Remote Procedure Call (`src/io/rpc/`)

The external API layer. Maps network requests to internal Commands and Queries.

## Architectural Role

Because the framework is headless, it needs a way for external clients (like a React web UI, a Python AI script, or a CLI terminal) to interact with the engine. 

The `rpc` module acts as the translator. It receives raw JSON payloads from a network socket, parses them to determine if the client wants to *do* something (Execute a Command) or *read* something (Run a Query), and routes the request to the appropriate core engine system.

## Design Decisions

- **CQRS (Command Query Responsibility Segregation):** The module strictly separates mutations from reads. 
  - Mutations go through `command_router.rs` -> `CommandPipeline`. This ensures external API calls are subject to the same Undo/Redo history, transactions, and permission checks as internal UI actions.
  - Reads go through `query_router.rs` -> `GraphQuery`. This ensures external clients get fast, read-only access to the graph without triggering unintended side effects.
- **Protocol Agnostic Routers:** The `command_router` and `query_router` are pure functions that take a JSON string and return a JSON string. They contain no networking logic. This allows the framework to plug them into any web framework (like Axum, Warp, or Actix-Web) without rewriting the business logic.
- **Universal API Surface:** By routing everything through this JSON layer, the framework achieves a universal API. The AI agent, the web frontend, and the CLI tool all use the exact same JSON protocol to talk to the engine.

## Files

- **`server.rs`**: The `RpcServer` mock struct. In a real application, this would hold the Axum/Warp route handlers. Here, it provides `handle_query_http` and `handle_command_ws` methods to simulate the network boundary.
- **`command_router.rs`**: Parses a `CommandRequest` JSON, calls the `CommandPipeline`, and formats the success/failure result into a `CommandResponse` JSON string.
- **`query_router.rs`**: Parses a `QueryRequest` JSON (e.g., `find_by_type`, `children`), executes the `GraphQuery`, and formats the resulting `NodeId` list into a `QueryResponse` JSON string.