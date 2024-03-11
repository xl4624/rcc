use std::error::Error;

use crate::token::{Token, TokenStream, Type};

use super::ast::{Expression, Function, Program, Statement};

pub fn parse(tokens: &[Token]) -> Result<Program, Box<dyn Error>> {
    let mut tokens = TokenStream::new(tokens);
    let function = parse_function(&mut tokens)?;
    Ok(Program { function })
}

fn parse_function(tokens: &mut TokenStream) -> Result<Function, Box<dyn Error>> {
    let ty = parse_type(tokens)?;
    let name = tokens.expect_identifier()?;
    tokens.expect(Token::LParen)?;
    tokens.expect(Token::RParen)?;
    tokens.expect(Token::LBrace)?;
    let body = parse_compound_statement(tokens)?;
    tokens.expect(Token::RBrace)?;
    Ok(Function { ty, name, body })
}

fn parse_type(tokens: &mut TokenStream) -> Result<Type, Box<dyn Error>> {
    match tokens.next() {
        Some(Token::Type(ty)) => Ok(ty.clone()),
        Some(token) => Err(format!("Expected int, found {:?}", token).into()),
        None => Err("Expected int, found EOF".into()),
    }
}

fn parse_compound_statement(tokens: &mut TokenStream) -> Result<Vec<Statement>, Box<dyn Error>> {
    let mut statements = Vec::new();
    loop {
        match tokens.peek() {
            Some(Token::RBrace) => break,
            Some(Token::Return) => statements.push(parse_statement(tokens)?),
            _ => return Err("Not implemented".into()),
        }
    }
    Ok(statements)
}

fn parse_statement(tokens: &mut TokenStream) -> Result<Statement, Box<dyn Error>> {
    match tokens.peek() {
        Some(&Token::Return) => parse_return_statement(tokens),
        _ => Err("Expected return statement".into()),
    }
}

fn parse_return_statement(tokens: &mut TokenStream) -> Result<Statement, Box<dyn Error>> {
    tokens.expect(Token::Return)?;
    let expression = parse_expression(tokens)?;
    if expression.is_some() {
        tokens.expect(Token::Semi)?;
    }
    match expression {
        Some(expression) => Ok(Statement::Return(Some(expression))),
        None => Ok(Statement::Return(None)),
    }
}

fn parse_expression(tokens: &mut TokenStream) -> Result<Option<Expression>, Box<dyn Error>> {
    match tokens.next() {
        Some(Token::IntLit(n)) => Ok(Some(Expression::IntLit(*n))),
        Some(Token::Semi) => Ok(None),
        Some(token) => Err(format!("Expected expression, found {:?}", token).into()),
        None => Err("Expected expression, found EOF".into()),
    }
}
