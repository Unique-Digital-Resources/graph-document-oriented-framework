# Persistence (`src/io/persistence/`)

Graph serialization, deserialization, and schema migration.

## Architectural Role

This module gives the document permanence. It translates the live, in-memory `Graph`—with all its nodes, properties, edges, and metadata—into a static format that can be saved to a hard drive or database, and later reconstructed exactly as it was.

## Design Decisions

- **Property Filtering:** The serializer is aware of `PropertyKind`. It automatically filters out `Transient`, `Computed`, and `Cached` properties, ensuring that only `Persistent` data is written to disk. This prevents bloated save files and ensures runtime state isn't accidentally loaded back into a fresh session.
- **Step-Wise Migrations:** When a plugin updates and adds a new property to a Node, old save files will lack that property. The `Migrator` handles this by running a sequence of step-wise upgrade functions (e.g., `migrate_v1_to_v2`), injecting default values or transforming old data structures so the application never crashes when opening a legacy file.
- **Snapshot Restoration:** Deserialization uses the `Node::with_id` constructor to preserve the original UUIDs from the save file. This is critical because graph edges reference specific UUIDs; if IDs changed on load, the entire document structure would break.

## Files

- **`serializer.rs`**: Contains the `serialize_graph` function. Iterates over the graph, collects nodes into a `Vec`, and uses `serde_json` to output a pretty-printed JSON string.
- **`deserializer.rs`**: Contains the `deserialize_graph` function. Parses a JSON string, reconstructs the `Node` structs (with their original IDs), and inserts them into a fresh `Graph` instance.
- **`migrator.rs`**: The `Migrator` struct. Exposes a `migrate` function that takes a loaded graph and its file version, applying sequential migration steps until it matches the current engine version.