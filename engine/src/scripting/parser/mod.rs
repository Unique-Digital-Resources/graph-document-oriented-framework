//! Lexer and Parser for the declarative DSL.

pub mod lexer;
pub mod syntax_tree;

pub use lexer::{Lexer, Token};
pub use syntax_tree::{AstNode, DslParser, NodeDefinitionAst, PropertyAst, RelationAst};