//! AST definitions and Parser logic.

use super::lexer::Token;

#[derive(Debug, Clone)]
pub struct PropertyAst {
    pub name: String,
    pub type_name: String,
    pub default_value: Option<String>,
    pub is_transient: bool,
}

#[derive(Debug, Clone)]
pub struct RelationAst {
    pub field_name: String,
    pub preset_name: String,
    pub target_type: String,
}

#[derive(Debug, Clone)]
pub struct NodeDefinitionAst {
    pub type_name: String,
    pub properties: Vec<PropertyAst>,
    pub relations: Vec<RelationAst>,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Node(NodeDefinitionAst),
}

pub struct DslParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl DslParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.peek()))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<AstNode>, String> {
        let mut nodes = Vec::new();
        while self.peek() != &Token::EOF {
            nodes.push(self.parse_define()?);
        }
        Ok(nodes)
    }

    fn parse_define(&mut self) -> Result<AstNode, String> {
        self.expect(Token::Define)?;
        match self.advance() {
            Token::Node => self.parse_node().map(AstNode::Node),
            _ => Err("Expected 'node' after 'define'".into()),
        }
    }

    fn parse_node(&mut self) -> Result<NodeDefinitionAst, String> {
        let type_name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected node type name".into()),
        };
        self.expect(Token::LBrace)?;

        let mut properties = Vec::new();
        let mut relations = Vec::new();

        while self.peek() != &Token::RBrace {
            // Check for the `transient:` shorthand syntax
            if self.peek() == &Token::Transient {
                self.advance(); // consume 'transient'
                self.expect(Token::Colon)?;
                let field_name = match self.advance() {
                    Token::Identifier(s) => s,
                    _ => return Err("Expected field name after 'transient:'".into()),
                };
                properties.push(PropertyAst {
                    name: field_name,
                    type_name: "any".to_string(), // Type is implicit
                    default_value: None,
                    is_transient: true,
                });
                continue;
            }

            // Normal property or relation
            let field_name = match self.advance() {
                Token::Identifier(s) => s,
                _ => return Err("Expected field name".into()),
            };

            self.expect(Token::Colon)?;

            if self.peek() == &Token::Relation {
                relations.push(self.parse_relation(field_name)?);
            } else {
                properties.push(self.parse_property(field_name, false)?);
            }
        }
        self.expect(Token::RBrace)?;
        Ok(NodeDefinitionAst { type_name, properties, relations })
    }

    fn parse_property(&mut self, name: String, is_transient: bool) -> Result<PropertyAst, String> {
        let type_name = match self.advance() {
            Token::StringType => "string".to_string(),
            Token::BooleanType => "boolean".to_string(),
            Token::Identifier(s) => s, // Custom types
            _ => return Err("Expected property type".into()),
        };

        let mut default_value = None;
        if self.peek() == &Token::Equals {
            self.advance();
            default_value = Some(match self.advance() {
                Token::StringLit(s) => s,
                Token::BooleanLit(b) => b.to_string(),
                _ => return Err("Expected literal after '='".into()),
            });
        }

        Ok(PropertyAst { name, type_name, default_value, is_transient })
    }

    fn parse_relation(&mut self, field_name: String) -> Result<RelationAst, String> {
        self.expect(Token::Relation)?;
        let preset_name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected relation preset name".into()),
        };
        self.expect(Token::Arrow)?;
        let target_type = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected target node type".into()),
        };
        Ok(RelationAst { field_name, preset_name, target_type })
    }
}