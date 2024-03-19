use std::{collections::HashMap, iter::Peekable, str::Chars};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Void,
}

pub fn lex(contents: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = contents.chars().peekable();
    let keywords = keyword_token_map();

    while let Some(c) = chars.next() {
        tokens.push(match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::Semi,
            _ if c.is_whitespace() => continue,
            _ if c.is_ascii_alphabetic() => lex_identifier_or_keyword(&mut chars, c, &keywords),
            _ if c.is_numeric() => lex_number(&mut chars, c),
            _ => return Err(anyhow!("Unexpected character: {:?}", c)),
        });
    }

    Ok(tokens)
}

fn lex_identifier_or_keyword(
    chars: &mut Peekable<Chars>,
    c: char,
    keywords: &HashMap<String, Token>,
) -> Token {
    let mut word = c.to_string();

    while let Some(&next_c) = chars.peek() {
        if !(next_c.is_ascii_alphabetic() || next_c == '_') {
            break;
        }
        // unwrap() is safe here because we know next_c is Some
        word.push(chars.next().unwrap());
    }

    match keywords.get(&word) {
        Some(token) => token.clone(),
        None => Token::Identifier(word),
    }
}

fn lex_number(chars: &mut Peekable<Chars>, c: char) -> Token {
    let mut number = c.to_string();
    while let Some(&next_c) = chars.peek() {
        if !next_c.is_numeric() {
            break;
        }
        // unwrap() is safe here because we know next_c is Some
        number.push(chars.next().unwrap());
    }

    Token::IntLit(number.parse().unwrap())
}

fn keyword_token_map() -> HashMap<String, Token> {
    let mut keywords = HashMap::new();

    keywords.insert("return".to_string(), Token::Return);
    keywords.insert("int".to_string(), Token::Type(Type::Int));
    keywords.insert("void".to_string(), Token::Type(Type::Void));

    keywords
}
