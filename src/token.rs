use std::{error::Error, iter::Peekable, slice::Iter};

// https://en.wikipedia.org/wiki/Lexical_analysis
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),

    // Keyword
    Return,
    Type(Type),

    // Separators/Punctuators
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,

    // Operators

    // Literals
    IntLit(u64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Void,
}

pub struct TokenStream<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

#[allow(dead_code)]
impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        TokenStream { iter: tokens.iter().peekable() }
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.iter.next()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.iter.peek().copied()
    }

    pub fn expect(&mut self, expected: Token) -> Result<(), Box<dyn Error>> {
        match self.iter.next() {
            Some(token) if *token == expected => Ok(()),
            Some(token) => Err(format!("Expected {:?}, found {:?}", expected, token).into()),
            None => Err(format!("Expected {:?}, found EOF", expected).into()),
        }
    }

    pub fn expect_any_of(&mut self, expected: &[Token]) -> Result<(), Box<dyn Error>> {
        match self.iter.next() {
            Some(token) if expected.contains(token) => Ok(()),
            Some(token) => Err(format!("Expected any of {:?}, found {:?}", expected, token).into()),
            None => Err(format!("Expected any of {:?}, found EOF", expected).into()),
        }
    }

    pub fn expect_identifier(&mut self) -> Result<String, Box<dyn Error>> {
        match self.iter.next() {
            Some(Token::Identifier(s)) => Ok(s.clone()),
            Some(token) => Err(format!("Expected identifier, found {:?}", token).into()),
            None => Err("Expected identifier, found EOF".into()),
        }
    }
}
