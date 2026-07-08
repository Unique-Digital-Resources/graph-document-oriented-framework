# Parser (`src/scripting/parser/`)

The Lexer and Parser for the declarative DSL.

## Architectural Role

This module acts as the front-end of the scripting engine. It takes a raw string of developer-authored DSL code and transforms it into a machine-readable Abstract Syntax Tree (AST). It is purely concerned with syntax and grammar; it does not know what a "Node" or a "Relation" actually does in the engine.

## Design Decisions

- **Hand-Written Lexer:** Instead of pulling in heavy external parser-generator crates (like `pest` or `lalrpop`), the lexer is a lightweight, hand-written character iterator. This keeps the framework's dependency footprint minimal and compilation fast.
- **Token Categorization:** The lexer distinguishes between keywords (`define`, `node`, `relation`, `transient`), primitive types (`string`, `boolean`), identifiers (custom names like `TaskNode`), and symbols (`->`, `{`, `}`). 
- **Robust Shorthand Parsing:** The parser explicitly handles the DSL's shorthand syntax. For example, `transient: uiSelected` is parsed directly into a `PropertyAst` with the `is_transient` flag set, rather than forcing the developer to write out a full type definition for runtime-only properties.

## Files

- **`lexer.rs`**: The `Lexer` struct. Iterates over a string character-by-character, skipping whitespace, grouping alphanumeric characters into identifiers/keywords, and detecting symbols. Outputs a `Vec<Token>`.
- **`syntax_tree.rs`**: The `DslParser` struct. Consumes the `Vec<Token>` using recursive descent. Enforces grammar rules (e.g., a `define` must be followed by `node` and an identifier). Outputs an AST composed of `AstNode`, `NodeDefinitionAst`, `PropertyAst`, and `RelationAst`.