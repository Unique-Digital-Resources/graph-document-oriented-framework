# Input / Output (`src/io/`)

Phase 5 of the framework. Bridges the isolated `core/` engine with the outside world.

## Architectural Role

The `core/` engine is deliberately hermetic. It knows nothing about hard drives, file systems, networks, or HTTP protocols. It only understands in-memory data structures. 

The `io` module is the boundary layer. It takes the purely logical graph and translates it into bytes for disk storage, and it exposes the Command Registry and Graph Query engine to external clients (like WebSockets, HTTP APIs, or CLI tools). This separation is what allows the exact same core engine to be embedded in a desktop app, run as a headless server, or execute inside a test harness.

## Design Decisions

- **Strict Boundary Enforcement:** The `io` module never bypasses the core rules. If an external API requests a mutation, the `io/rpc` layer does not mutate the graph directly; it routes the request into the `CommandPipeline`.
- **Versioned Data:** Documents evolve over time. The persistence layer doesn't just save and load; it stamps data with a schema version and uses a `Migrator` to update old documents to the current schema upon loading.
- **Future-Proofed Scaling:** While currently using simple JSON for serialization, the file structure is designed to be swapped out for chunked binary formats (like MessagePack or custom binary chunks) to support 1,000,000+ node documents without memory spikes.

## Modules

- **`persistence/`**: Handles serializing the graph to disk, deserializing it back, and migrating old schemas to new ones.
- **`rpc/`**: The external API layer. Maps JSON network requests to internal Commands and Queries.