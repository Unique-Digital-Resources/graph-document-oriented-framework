# Commands (`src/core/command/`)

Pillar 3 of the framework. Commands are the sole mutators of the graph and represent executable behavior.

## Architectural Role

If Nodes are the nouns of the application, Commands are the verbs. The framework enforces a strict rule: **no code outside the Command Pipeline can mutate the `Graph` or a `Node`'s properties.** 

This is the bedrock of the framework's reliability. By forcing all state changes through a single pipeline, the framework provides free Undo/Redo, free audit logging, universal API exposure (for AI and RPC), and permission checking without scattering that logic across hundreds of developer-written functions.

## Design Decisions

- **Intent vs. Fact:** Commands represent *Intent* (Future tense: "Do this"). They are triggered by the UI, API, or AI. This is strictly contrasted by Signals, which represent *Fact* (Past tense: "This happened"). 
- **Implicit & Explicit Transactions:** The Pipeline automatically wraps every `execute` call in an implicit transaction. If a developer needs to batch multiple commands into a single Undo step, they can explicitly call `begin_transaction` and `commit_transaction` on the pipeline.
- **Auto-Rollback:** If a command's execution function returns an `Err`, the pipeline intercepts it, restores the `Graph` to its pre-mutation state using a `GraphSnapshot`, and discards any transactional signals, ensuring the document is never left in a corrupted state.

## Files

- **`registry.rs`**: The `CommandRegistry` holds `CommandDefinition` instances. Commands are registered at startup by the core framework or plugins. It allows lookup by string ID (e.g., `"CompleteTask"`).
- **`pipeline.rs`**: The `CommandPipeline` executor. Orchestrates the strict flow: `Lookup Command -> Begin Transaction -> Execute -> Emit Transactional Signal -> Commit to History -> Flush Signals`. It also handles the rollback logic upon failure.
- **`transaction.rs`**: Re-exports the `Transaction` struct (defined in `history/macro.rs`) to fulfill the architectural file layout, grouping multiple command steps into an atomic batch.