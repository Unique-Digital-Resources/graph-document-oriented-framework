# Declarative Scripting (`src/scripting/`)

Phase 4 of the framework. Translates developer-written DSL scripts into core graph registrations.

## Architectural Role

The biggest barrier to entry in complex frameworks is boilerplate. Forcing developers to write Rust structs, implement traits, and manually register graph edges for every new data type is tedious. 

The `scripting` module provides a Domain Specific Language (DSL) that brings the "zero boilerplate" developer experience to life. Developers write simple, declarative text files (e.g., `define node TaskNode { ... }`). This module reads that text, parses it into an Abstract Syntax Tree (AST), and compiles it into typed schemas that the `core` engine can register and enforce at runtime.

## Design Decisions

- **Declarative over Imperative:** The DSL only allows developers to declare *what* the data is, not *how* it should be stored or wired. The framework automatically handles ID generation, edge map initialization, and serialization based on the declarative tags (e.g., `transient:`).
- **Macro-Based Relations:** Developers do not need to understand the 7 low-level relation dimensions. They just write `relation CHILDREN -> TaskNode`. The compiler resolves `CHILDREN` against the `RelationPresetRegistry` in the core engine.
- **Separation of Parsing and Compilation:** The module is strictly divided into a `parser` (which understands syntax) and a `runtime` (which understands semantics). This allows the parser to be reused or swapped out (e.g., for a YAML or JSON-based schema format) without touching the core engine integration.

## Modules

- **`parser/`**: The Lexer and Parser. Responsible for reading raw text strings and converting them into a structured Abstract Syntax Tree (AST) without validating any business logic.
- **`runtime/`**: The Compiler. Takes the AST, resolves types and relation presets, and generates `CompiledNodeSchema` objects ready for the `Graph` and `CommandRegistry` to consume.