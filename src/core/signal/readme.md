# Signals (`src/core/signal/`)

Pillar 2 of the framework. Signals are the "nervous system" of the engine, providing decoupled, fact-based notifications.

## Architectural Role

In complex applications, tightly coupling components quickly leads to spaghetti code (e.g., Node A calls a method on Node B, which updates the UI, which saves a file). 

Signals solve this by enforcing a strict rule: **Signals describe the past.** They never perform actions. When a Command mutates the graph, the Event Bus emits a Signal (e.g., `NodePropertyChanged`). Any interested party (UI, Search Indexer, AI Agent) listens for this signal and reacts accordingly. The mutator does not need to know who is listening.

## Design Decisions

- **Emit Timing Categories:** Not all signals should be processed immediately. If a command mutates 1,000 nodes in a transaction, emitting 1,000 immediate UI updates would crash the app. Signals are categorized:
  - `Immediate`: Dispatched instantly (e.g., UI selection changes).
  - `Deferred`: Queued and dispatched on the next scheduler tick.
  - `Transactional`: Queued and *only* dispatched if the command's transaction successfully commits. If the transaction rolls back, the signals are discarded.
- **Pub/Sub Model:** Producers (Commands) and Consumers (Systems/UI) only agree on a `signal_type` string. They never hold references to each other, allowing the core engine to remain completely headless and agnostic to what consumes its events.

## Files

- **`types.rs`**: Defines the `Signal` struct (type, source node, payload, timestamp) and the `EmitTiming` enum.
- **`event_bus.rs`**: The `EventBus` implementation. Allows systems to `subscribe` to signal types. Contains the internal queues for `Deferred` and `Transactional` signals, exposing `flush_deferred` and `flush_transactional` methods to the Command Pipeline for controlled dispatch.