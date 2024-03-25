use std::{iter::Peekable, slice::Iter};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::lexer::{
    Keyword::*,
    Operator::{self},
    Precedence,
    Separator::*,
    Token, TokenKind, Type,
};

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Expression {
    IntLit(u32),
    FunctionCall { name: String },
    Binary { left: Box<Expression>, op: Operator, right: Box<Expression> },
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
        self.token_stream.expect(TokenKind::Separator(LParen))?;
        self.token_stream.expect(TokenKind::Separator(RParen))?;
        self.token_stream.expect(TokenKind::Separator(LBrace))?;
        let body = self.parse_compound_statement()?;
        self.token_stream.expect(TokenKind::Separator(RBrace))?;
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
                    TokenKind::Separator(RBrace) => break,
                    TokenKind::Keyword(Return) => self.parse_return_statement()?,
                    _ => return Err(anyhow!("Not implemented yet")),
                },
                None => return Err(anyhow!("Expected statement or '}}', found EOF")),
            })
        }
        Ok(statements)
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.token_stream.expect(TokenKind::Keyword(Return))?;
        let expression = self.parse_expression()?;
        self.token_stream.expect(TokenKind::Separator(Semi))?;
        match expression {
            Some(expression) => Ok(Statement::Return(Some(expression))),
            None => Ok(Statement::Return(None)),
        }
    }

    fn parse_expression(&mut self) -> Result<Option<Expression>> {
        let mut operand_stack: Vec<Expression> = Vec::new();
        // Vec<TokeKind> to hold Separator::LParen as well as operators.
        let mut operator_stack: Vec<TokenKind> = Vec::new();
        loop {
            match self.token_stream.peek() {
                Some(token) => match &token.kind {
                    TokenKind::IntLit(n) => {
                        let n: u32 = *n; // Copy the value out of the reference to avoid multiple borrows.
                        self.token_stream.next(); // Second borrow would've happened here (mutable).

                        operand_stack.push(Expression::IntLit(n));
                    }
                    TokenKind::Identifier(ref name) => {
                        let name = name.clone();
                        self.token_stream.next();

                        self.token_stream.expect(TokenKind::Separator(LParen))?;
                        self.token_stream.expect(TokenKind::Separator(RParen))?;
                        operand_stack.push(Expression::FunctionCall { name });
                    }
                    TokenKind::Separator(LParen) => {
                        self.token_stream.next();

                        operator_stack.push(TokenKind::Separator(LParen));
                    }
                    TokenKind::Separator(RParen) => {
                        self.token_stream.next();

                        if operator_stack.is_empty() {
                            return Err(anyhow!("Invalid expression: Mismatched parentheses"));
                        }
                        while let Some(TokenKind::Operator(op)) = operator_stack.pop() {
                            self.apply_operator(&mut operand_stack, op)?;
                        }
                    }
                    TokenKind::Operator(op) => {
                        let op = op.clone();
                        self.token_stream.next();

                        let op_precedence = op.precedence();
                        while let Some(TokenKind::Operator(top_op)) = operator_stack.last() {
                            if top_op.precedence() >= op_precedence {
                                let op_to_apply = match operator_stack.pop().unwrap() {
                                    TokenKind::Operator(op) => op,
                                    _ => {
                                        return Err(anyhow!(
                                            "Invalid expression: Expected operator"
                                        ))
                                    }
                                };
                                self.apply_operator(&mut operand_stack, op_to_apply)?;
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(TokenKind::Operator(op));
                    }
                    _ => break,
                },
                None => break,
            }
        }
        while let Some(TokenKind::Operator(op)) = operator_stack.pop() {
            self.apply_operator(&mut operand_stack, op)?;
        }
        match operand_stack.len() {
            0 => Ok(None),
            1 => Ok(Some(operand_stack.pop().unwrap())),
            _ => Err(anyhow!("Invalid expression: Too many operands")),
        }
    }

    fn apply_operator(&self, operand_stack: &mut Vec<Expression>, op: Operator) -> Result<()> {
        if let (Some(right), Some(left)) = (operand_stack.pop(), operand_stack.pop()) {
            operand_stack.push(Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
            Ok(())
        } else {
            Err(anyhow!("Invalid expression: Not enough operands"))
        }
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Operator::*;

    #[test]
    fn test_parse_expression() {
        let tokens = vec![
            Token { kind: TokenKind::IntLit(1), pos: Default::default() },
            Token { kind: TokenKind::Operator(Plus), pos: Default::default() },
            Token { kind: TokenKind::IntLit(2), pos: Default::default() },
            Token { kind: TokenKind::Separator(Semi), pos: Default::default() },
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert_eq!(
            expression.unwrap(),
            Some(Expression::Binary {
                left: Box::new(Expression::IntLit(1)),
                op: Plus,
                right: Box::new(Expression::IntLit(2)),
            })
        );
    }

    #[test]
    fn test_parse_expression_parentheses() {
        let tokens = vec![
            Token { kind: TokenKind::Separator(LParen), pos: Default::default() },
            Token { kind: TokenKind::IntLit(1), pos: Default::default() },
            Token { kind: TokenKind::Operator(Plus), pos: Default::default() },
            Token { kind: TokenKind::IntLit(2), pos: Default::default() },
            Token { kind: TokenKind::Separator(RParen), pos: Default::default() },
            Token { kind: TokenKind::Separator(Semi), pos: Default::default() },
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert_eq!(
            expression.unwrap(),
            Some(Expression::Binary {
                left: Box::new(Expression::IntLit(1)),
                op: Plus,
                right: Box::new(Expression::IntLit(2)),
            })
        );
    }

    #[test]
    fn test_parse_expression_precedence() {
        let tokens = vec![
            Token { kind: TokenKind::IntLit(1), pos: Default::default() },
            Token { kind: TokenKind::Operator(Star), pos: Default::default() },
            Token { kind: TokenKind::IntLit(2), pos: Default::default() },
            Token { kind: TokenKind::Operator(Plus), pos: Default::default() },
            Token { kind: TokenKind::IntLit(3), pos: Default::default() },
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert_eq!(
            expression.unwrap(),
            Some(Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::IntLit(1)),
                    op: Star,
                    right: Box::new(Expression::IntLit(2)),
                }),
                op: Plus,
                right: Box::new(Expression::IntLit(3)),
            })
        );
    }

    #[test]
    fn test_parse_expression_precedence_parentheses() {
        let tokens = vec![
            Token { kind: TokenKind::IntLit(1), pos: Default::default() },
            Token { kind: TokenKind::Operator(Star), pos: Default::default() },
            Token { kind: TokenKind::Separator(LParen), pos: Default::default() },
            Token { kind: TokenKind::IntLit(2), pos: Default::default() },
            Token { kind: TokenKind::Operator(Plus), pos: Default::default() },
            Token { kind: TokenKind::IntLit(3), pos: Default::default() },
            Token { kind: TokenKind::Separator(RParen), pos: Default::default() },
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert_eq!(
            expression.unwrap(),
            Some(Expression::Binary {
                left: Box::new(Expression::IntLit(1)),
                op: Star,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::IntLit(2)),
                    op: Plus,
                    right: Box::new(Expression::IntLit(3)),
                }),
            })
        );
    }

    #[test]
    fn test_parse_expression_invalid() {
        let tokens = vec![
            Token { kind: TokenKind::Operator(Star), pos: Default::default() },
            Token { kind: TokenKind::Operator(Slash), pos: Default::default() },
        ];

        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert!(expression.is_err());
    }

    #[test]
    fn test_parse_expression_invalid2() {
        let tokens = vec![
            Token { kind: TokenKind::IntLit(1), pos: Default::default() },
            Token { kind: TokenKind::IntLit(2), pos: Default::default() },
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert!(expression.is_err());
    }

    #[test]
    fn test_parse_expression_invalid3() {
        let tokens = vec![
            Token { kind: TokenKind::IntLit(1), pos: Default::default() },
            Token { kind: TokenKind::Operator(Plus), pos: Default::default() },
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert!(expression.is_err());
    }

    #[test]
    fn test_parse_expression_invalid4() {
        let tokens = vec![
            Token { kind: TokenKind::Separator(LParen), pos: Default::default() },
            Token { kind: TokenKind::IntLit(1), pos: Default::default() },
            Token { kind: TokenKind::Operator(Plus), pos: Default::default() },
            Token { kind: TokenKind::IntLit(2), pos: Default::default() },
            Token { kind: TokenKind::Separator(RParen), pos: Default::default() },
            Token { kind: TokenKind::IntLit(3), pos: Default::default() },
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert!(expression.is_err());
    }

    #[test]
    fn test_parse_expression_invalid_parentheses() {
        let tokens = vec![
            Token { kind: TokenKind::Separator(LParen), pos: Default::default() },
            Token { kind: TokenKind::Separator(RParen), pos: Default::default() },
            Token { kind: TokenKind::Separator(RParen), pos: Default::default() },
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse_expression();
        assert!(expression.is_err());
    }
}
