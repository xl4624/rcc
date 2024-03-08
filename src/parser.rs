use std::{iter::Peekable, slice::Iter};

use crate::lexer::Token;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDefinition>,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub ty: String,
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
    UnaryOp { op: UnaryOperator, expr: Box<Expression> },
    BinaryOp { op: BinaryOperator, left: Box<Expression>, right: Box<Expression> },
}

#[derive(Debug)]
pub enum UnaryOperator {
    Minus,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, String> {
    let mut tokens = tokens.iter().peekable();
    let program = parse_program(&mut tokens)?;

    Ok(program)
}

fn parse_program(tokens: &mut Peekable<Iter<Token>>) -> Result<Program, String> {
    let mut functions = Vec::new();

    while tokens.peek().is_some() {
        let function = parse_definition(tokens)?;
        functions.push(function);
    }

    Ok(Program { functions })
}

fn parse_definition(tokens: &mut Peekable<Iter<Token>>) -> Result<FunctionDefinition, String> {
    let ty = match tokens.next() {
        Some(Token::Int) => "int".to_string(),
        _ => return Err("PARSER ERROR: Expected 'int' keyword".into()),
    };

    let name = match tokens.next() {
        Some(Token::Ident(name)) => name.clone(),
        _ => return Err("PARSER ERROR: Expected an identifier".into()),
    };

    match tokens.next() {
        Some(Token::LParen) => {
            let params = parse_params(tokens)?;
            match tokens.next() {
                Some(Token::LBrace) => {
                    let body = parse_compound_statement(tokens)?;
                    Ok(FunctionDefinition { ty, name, params, body })
                }
                _ => Err("PARSER ERROR: Expected an opening brace".into()),
            }
        }
        Some(Token::Equal) => {
            Err("PARSER ERROR: Function definitions are not yet supported".into())
            // match tokens.peek() {
            //     Some(Token::Semi) => Err("PARSER ERROR: Variable declarations are not yet supported".into()),
            //     _ => {
            //         let expression = parse_expression(tokens)?;
            //         match tokens.next() {
            //             Some(Token::Semi) => {
            //                 Ok(Definition::Variable(VariableDefinition { ty, name, value: expression }))
            //             }
            //             _ => Err("PARSER ERROR: Expected a semicolon".into()),
            //         }
            //     }
            // }
        }
        _ => Err("PARSER ERROR: Expected an opening parenthesis".into()),
    }
}

fn parse_params(tokens: &mut Peekable<Iter<Token>>) -> Result<Vec<String>, String> {
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
            _ => return Err("PARSER ERROR: Expected a comma or closing parenthesis".into()),
        }
    }

    Err("PARSER ERROR: Expected a closing parenthesis".into())
}

fn parse_compound_statement(tokens: &mut Peekable<Iter<Token>>) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();

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

    Err("PARSER ERROR: Expected a closing brace".into())
}

fn parse_statement(tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    match tokens.peek() {
        Some(Token::Return) => {
            tokens.next();
            match tokens.peek() {
                Some(Token::Semi) => {
                    tokens.next();
                    Ok(Statement::Return(Expression::Constant(0)))
                }
                _ => {
                    let expression = parse_expression(tokens)?;
                    match tokens.next() {
                        Some(Token::Semi) => Ok(Statement::Return(expression)),
                        _ => Err("PARSER ERROR: Expected a semicolon".into()),
                    }
                }
            }
        }
        // Some(Token::Break) => match tokens.next() {
        //     Some(Token::Semi) => Ok(Statement::Break),
        //     _ => Err("PARSER ERROR: Expected a semicolon".into()),
        // },
        _ => Err("PARSER ERROR: Expected 'return' keyword".into()),
    }
}

fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    let mut expr = match tokens.peek() {
        Some(Token::Minus) => {
            tokens.next();
            let expr = parse_expression(tokens)?;
            Expression::UnaryOp { op: UnaryOperator::Minus, expr: Box::new(expr) }
        }
        Some(Token::IntLit(value)) => {
            tokens.next();
            Expression::Constant(*value)
        }
        _ => return Err("PARSER ERROR: Expected a unary operator or a constant".into()),
    };

    loop {
        match tokens.peek() {
            Some(Token::Plus) => {
                tokens.next();
                let right = parse_expression(tokens)?;
                expr = Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Some(Token::Minus) => {
                tokens.next();
                let right = parse_expression(tokens)?;
                expr = Expression::BinaryOp {
                    op: BinaryOperator::Subtract,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(expr)
}
