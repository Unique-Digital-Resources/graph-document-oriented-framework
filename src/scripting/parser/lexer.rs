//! A simple tokenizer for the framework DSL.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Keywords
    Define,
    Node,
    Relation,
    Transient,
    
    // Types
    StringType, // `string`
    BooleanType, // `boolean`
    
    // Symbols
    Identifier(String),
    LBrace,      // {
    RBrace,      // }
    Colon,       // :
    Arrow,       // ->
    Equals,      // =
    
    // Literals
    StringLit(String),
    BooleanLit(bool),
    
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.input.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string_lit(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if *c == '"' {
                self.advance();
                break;
            }
            s.push(self.advance().unwrap());
        }
        Token::StringLit(s)
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || *c == '_' {
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        match s.as_str() {
            "define" => Token::Define,
            "node" => Token::Node,
            "relation" => Token::Relation,
            "transient" => Token::Transient,
            "string" => Token::StringType,
            "boolean" => Token::BooleanType,
            "true" => Token::BooleanLit(true),
            "false" => Token::BooleanLit(false),
            _ => Token::Identifier(s),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            let token = match c {
                '{' => { self.advance(); Token::LBrace }
                '}' => { self.advance(); Token::RBrace }
                ':' => { self.advance(); Token::Colon }
                '=' => { self.advance(); Token::Equals }
                '-' => {
                    self.advance();
                    if self.peek() == Some(&'>') {
                        self.advance();
                        Token::Arrow
                    } else {
                        panic!("Expected '>' after '-'");
                    }
                }
                '"' => self.read_string_lit(),
                _ if c.is_alphanumeric() || *c == '_' => self.read_identifier_or_keyword(),
                _ => panic!("Unexpected character: {}", c),
            };
            tokens.push(token);
        }
        tokens.push(Token::EOF);
        tokens
    }
}