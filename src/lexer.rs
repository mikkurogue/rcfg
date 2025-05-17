#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Struct,
    Ident(String),
    Colon,
    Comma,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Decorator(String),
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.input.get(self.position).copied();
        self.position += 1;

        ch
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(ch) if ch.is_whitespace()) {
            self.next();
        }
    }

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.next();
            } else {
                break;
            }
        }

        ident
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.next() {
            Some(':') => Token::Colon,
            Some(',') => Token::Comma,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => {
                let ident = self.read_ident();
                if self.peek() == Some(']') {
                    self.next();
                    if !ident.is_empty() {
                        Token::Decorator(ident)
                    } else {
                        Token::LBracket
                    }
                } else {
                    Token::LBracket
                }
            }
            Some(']') => Token::RBracket,
            Some(ch) if ch.is_alphabetic() => {
                let mut ident = ch.to_string();

                ident.push_str(&self.read_ident());
                match ident.as_str() {
                    "struct" => Token::Struct,
                    _ => Token::Ident(ident),
                }
            }
            None => Token::Eof,
            Some(other) => {
                panic!("Unexpected token {}", other)
            }
        }
    }
}
