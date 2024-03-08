use std::{iter::Peekable, slice::Iter};

use crate::lexer::Token;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDefinition>,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub return_type: String,
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(u32),
}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, String> {
    let mut tokens = tokens.iter().peekable();
    let program = parse_program(&mut tokens)?;

    Ok(program)
}

fn parse_program(tokens: &mut Peekable<Iter<Token>>) -> Result<Program, String> {
    let mut function_declarations = Vec::new();

    while tokens.peek().is_some() {
        let function_declaration = parse_function_declaration(tokens)?;
        function_declarations.push(function_declaration);
    }

    Ok(Program { functions: function_declarations })
}

fn parse_function_declaration(
    tokens: &mut Peekable<Iter<Token>>,
) -> Result<FunctionDefinition, String> {
    let return_type = match tokens.next() {
        Some(Token::Int) => "int".into(),
        _ => return Err("Expected 'int' keyword".into()),
    };

    let name = match tokens.next() {
        Some(Token::Ident(name)) => name.clone(),
        _ => return Err("Expected an identifier".into()),
    };

    let params = parse_params(tokens)?;

    let body = parse_compound_statement(tokens)?;

    Ok(FunctionDefinition { return_type, name, params, body })
}

fn parse_params(tokens: &mut Peekable<Iter<Token>>) -> Result<Vec<String>, String> {
    match tokens.next() {
        Some(Token::LParen) => (),
        _ => return Err("Expected an opening parenthesis".into()),
    }

    #[allow(unused_mut)]
    let mut params = Vec::new();

    while let Some(&next_token) = tokens.peek() {
        match next_token {
            Token::RParen => {
                tokens.next();
                return Ok(params);
            }
            // Token::Ident(param_name) => {
            //     tokens.next();
            //     params.push(param_name.clone());
            // }
            _ => return Err("Expected a comma or closing parenthesis".into()),
        }
    }

    Err("Expected a closing parenthesis".into())
}

fn parse_compound_statement(tokens: &mut Peekable<Iter<Token>>) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();

    match tokens.next() {
        Some(Token::LBrace) => (),
        _ => return Err("Expected an opening brace".into()),
    }

    while let Some(&next_token) = tokens.peek() {
        match next_token {
            Token::RBrace => {
                tokens.next();
                return Ok(statements);
            }
            _ => {
                let statement = parse_statement(tokens)?;
                statements.push(statement);
            }
        }
    }

    Err("Expected a closing brace".into())
}

fn parse_statement(tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    match tokens.next() {
        Some(Token::Return) => match tokens.peek() {
            Some(Token::Semi) => {
                tokens.next();
                Ok(Statement::Return(Expression::Constant(0)))
            }
            _ => {
                let expression = parse_expression(tokens)?;
                match tokens.next() {
                    Some(Token::Semi) => Ok(Statement::Return(expression)),
                    _ => Err("Expected a semicolon".into()),
                }
            }
        },
        // Some(Token::Break) => match tokens.next() {
        //     Some(Token::Semi) => Ok(Statement::Break),
        //     _ => Err("Expected a semicolon".into()),
        // },
        _ => Err("Expected 'return' keyword".into()),
    }
}

fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    match tokens.next() {
        Some(Token::IntLit(value)) => Ok(Expression::Constant(*value)),
        _ => Err("Expected a constant expression".into()),
    }
}
