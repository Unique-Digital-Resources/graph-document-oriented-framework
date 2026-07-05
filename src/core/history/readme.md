# History & Transactions (`src/core/history/`)

A sub-system of Pillar 3. Manages the Undo/Redo stacks and provides transactional safety for graph mutations.

## Architectural Role

Without a centralized history system, implementing Undo/Redo in a graph-based application is a nightmare—developers would have to manually track reverse operations for every action. 

This module automates that process. It works in tandem with the `CommandPipeline`. Before a command mutates a node, the history system takes a snapshot of the node's state. If the user clicks "Undo", the history system pops the last entry and restores the snapshot.

## Design Decisions

- **Macro Batching (Transactions):** Complex user actions (like "Duplicate Object") might involve creating a node, renaming it, and moving it. If these were separate history steps, the user would have to click Undo three times. The `Transaction` struct groups multiple `CommandStep`s together so they count as a single entry on the Undo stack.
- **Partial Snapshots:** Instead of cloning the entire 1-million-node graph before every command (which would destroy performance), the `GraphSnapshot` only clones the specific nodes that the command explicitly captures during its execution.
- **Reverse Execution:** For explicit undo logic defined by the developer, the history stack stores the `UndoFn` closure and the parameters passed to the original command, executing them in reverse order when an undo is requested.

## Files

- **`snapshot.rs`**: `GraphSnapshot` captures the state of specific nodes before mutation. Used to rollback failed transactions or auto-revert properties if a command lacks an explicit undo function.
- **`macro.rs`**: Defines the `Transaction` struct and `CommandStep`. A transaction accumulates steps and snapshots during a command execution and is finalized into a `HistoryEntry` upon commit.
- **`stack.rs`**: The `HistoryStack` manages the LIFO (Last-In, First-Out) undo queue and the redo queue. It exposes `push`, `undo`, and `redo` operations to the application.