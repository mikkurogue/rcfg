use crate::ast::{Field, Struct, Type};
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer,
    current: Token,
    peeked: Option<Token>,
    src: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer::new(src);

        let first_token = lexer.next_token();
        Self {
            lexer,
            current: first_token,
            peeked: None,
            src,
        }
    }

    fn next(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn expect(&mut self, expected: &Token) {
        if &self.current != expected {
            panic!("expected {:?} but found {:?}", expected, self.current);
        }

        self.next();
    }

    pub fn parse(&mut self) -> Vec<Struct> {
        let mut structs = Vec::new();

        while self.current != Token::Eof {
            let decorators = self.parse_decorators();

            structs.push(self.parse_struct(decorators));
        }

        structs
    }

    fn parse_decorators(&mut self) -> Vec<String> {
        let mut decorators = Vec::new();

        while let Token::Decorator(name) = &self.current {
            decorators.push(name.clone());
            self.next();
        }

        decorators
    }

    fn parse_struct(&mut self, decorators: Vec<String>) -> Struct {
        self.expect(&Token::Struct);

        let name = match &self.current {
            Token::Ident(name) => {
                let name = name.clone();
                self.next();
                name
            }
            _ => panic!("Expected struct name, found {:?}", self.current),
        };

        self.expect(&Token::LBrace);

        let mut fields = Vec::new();
        while self.current != Token::RBrace {
            fields.push(self.parse_field());

            if self.current == Token::Comma {
                self.next(); // skip comma

                if self.current == Token::LBrace {
                    break;
                }
            }
        }

        self.expect(&Token::RBrace);

        Struct {
            name,
            fields,
            decorators,
        }
    }

    fn parse_field(&mut self) -> Field {
        let name = match &self.current {
            Token::Ident(name) => {
                let name = name.clone();
                self.next();
                name
            }
            _ => panic!("Expected field name, found {:?}", self.current),
        };

        self.expect(&Token::Colon);
        let ty = self.parse_type();

        Field { name, ty }
    }

    fn parse_type(&mut self) -> Type {
        match &self.current {
            Token::Ident(name) => {
                let base = name.clone();
                self.next();

                // Handle `[]` for array
                if self.current == Token::LBracket {
                    self.next();
                    Type::Array(Box::new(self.ident_to_type(&base)))
                } else {
                    self.ident_to_type(&base)
                }
            }
            _ => panic!("Expected type identifier, found {:?}", self.current),
        }
    }

    fn ident_to_type(&self, name: &str) -> Type {
        match name {
            "String" => Type::String,
            "i32" => Type::Int,
            "bool" => Type::Bool,
            other => Type::Custom(other.to_string()),
        }
    }
}
