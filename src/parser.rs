use std::{iter::Peekable, slice::Iter};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::lexer::{Token, TokenKind, Type};

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Statement {
    Return(Option<Expression>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Expression {
    IntLit(u32),
    FunctionCall { name: String },
}

pub struct Parser<'a> {
    token_stream: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { token_stream: TokenStream::new(tokens) }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut functions = Vec::new();
        loop {
            match self.token_stream.peek() {
                Some(_) => functions.push(self.parse_function()?),
                None => break,
            }
        }
        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<Function> {
        let return_type = self.parse_type()?;
        let name = self.token_stream.expect_identifier()?;
        // TODO: Here we should check for both declarations and definitions.
        self.token_stream.expect(TokenKind::LParen)?;
        self.token_stream.expect(TokenKind::RParen)?;
        self.token_stream.expect(TokenKind::LBrace)?;
        let body = self.parse_compound_statement()?;
        self.token_stream.expect(TokenKind::RBrace)?;
        Ok(Function { return_type, name, body })
    }

    fn parse_type(&mut self) -> Result<Type> {
        match self.token_stream.next() {
            Some(token) => match &token.kind {
                TokenKind::Type(ty) => Ok(ty.clone()),
                _ => Err(anyhow!("Expected type, found {:?}", token)),
            },
            None => Err(anyhow!("Expected type, found EOF")),
        }
    }

    fn parse_compound_statement(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        loop {
            statements.push(match self.token_stream.peek() {
                Some(token) => match &token.kind {
                    TokenKind::RBrace => break,
                    TokenKind::Return => self.parse_return_statement()?,
                    _ => return Err(anyhow!("Not implemented yet")),
                },
                None => return Err(anyhow!("Expected statement or '}}', found EOF")),
            })
        }
        Ok(statements)
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.token_stream.expect(TokenKind::Return)?;
        let expression = self.parse_expression()?;
        self.token_stream.expect(TokenKind::Semi)?;
        match expression {
            Some(expression) => Ok(Statement::Return(Some(expression))),
            None => Ok(Statement::Return(None)),
        }
    }

    fn parse_expression(&mut self) -> Result<Option<Expression>> {
        match self.token_stream.peek() {
            Some(token) => match &token.kind {
                TokenKind::IntLit(n) => {
                    let n: u32 = *n; // Copy the value out of the reference to avoid multiple borrows.
                    self.token_stream.next(); // Second borrow would've happened here (mutable).
                    Ok(Some(Expression::IntLit(n)))
                }
                TokenKind::Identifier(ref name) => {
                    let name = name.clone();
                    self.token_stream.next();
                    self.token_stream.expect(TokenKind::LParen)?;
                    self.token_stream.expect(TokenKind::RParen)?;
                    Ok(Some(Expression::FunctionCall { name }))
                }
                TokenKind::LParen => {
                    self.token_stream.next();
                    let expr = self.parse_expression()?;
                    self.token_stream.expect(TokenKind::RParen)?;
                    Ok(expr)
                }
                _ => Ok(None),
            },
            None => Err(anyhow!("Expected expression, found EOF")),
        }
    }
}

struct TokenStream<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        TokenStream { tokens: tokens.iter().peekable() }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek().copied()
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.tokens.next()
    }

    pub fn expect(&mut self, expected: TokenKind) -> Result<()> {
        match self.next() {
            Some(token) if token.kind == expected => Ok(()),
            Some(token) => Err(anyhow!("Expected {:?}, found {:?}", expected, token)),
            None => Err(anyhow!("Expected {:?}, found EOF", expected)),
        }
    }

    pub fn expect_identifier(&mut self) -> Result<String> {
        match self.next() {
            Some(token) => match &token.kind {
                TokenKind::Identifier(name) => Ok(name.clone()),
                token => Err(anyhow!("Expected identifier, found {:?}", token)),
            },
            None => Err(anyhow!("Expected identifier, found EOF")),
        }
    }
}
