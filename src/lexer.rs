use std::{collections::HashMap, fmt, iter::Peekable, str::Chars};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::lexer::{Keyword::*, Operator::*, Separator::*, Type::*};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pos: Position,
    keywords: HashMap<String, TokenKind>,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
}

// https://en.wikipedia.org/wiki/Lexical_analysis
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Keyword(Keyword),
    Type(Type),
    Separator(Separator),
    Operator(Operator),

    // Literals
    IntLit(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Return,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Separator {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub struct Position {
    file: String,
    line: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(file: String, contents: &'a str) -> Self {
        Lexer {
            chars: contents.chars().peekable(),
            pos: Position::new(file),
            keywords: keyword_token_map(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(c) = self.chars.next() {
            let start_pos = self.pos.clone();
            let token_kind = match c {
                '(' => TokenKind::Separator(LParen),
                ')' => TokenKind::Separator(RParen),
                '{' => TokenKind::Separator(LBrace),
                '}' => TokenKind::Separator(RBrace),
                ';' => TokenKind::Separator(Semi),
                '+' => TokenKind::Operator(Plus),
                '-' => TokenKind::Operator(Minus),
                '*' => TokenKind::Operator(Star),
                '/' => TokenKind::Operator(Slash),
                _ if c.is_whitespace() => {
                    self.pos.advance(c);
                    continue;
                }
                _ if c.is_ascii_alphabetic() => self.lex_identifier_or_keyword(c),
                _ if c.is_numeric() => self.lex_number(c)?,
                _ => return Err(anyhow!("Unexpected character: {:?}", c)),
            };
            tokens.push(Token { kind: token_kind, pos: start_pos });
        }
        Ok(tokens)
    }

    fn lex_identifier_or_keyword(&mut self, c: char) -> TokenKind {
        let mut word = c.to_string();

        while let Some(&next_c) = self.chars.peek() {
            if !(next_c.is_ascii_alphanumeric() || next_c == '_') {
                break;
            }
            word.push(self.chars.next().unwrap());
            self.pos.advance(next_c);
        }

        match self.keywords.get(&word) {
            Some(token) => token.clone(),
            None => TokenKind::Identifier(word),
        }
    }

    fn lex_number(&mut self, c: char) -> Result<TokenKind> {
        let mut number = c.to_string();
        while let Some(&next_c) = self.chars.peek() {
            if !next_c.is_numeric() {
                break;
            }
            number.push(self.chars.next().unwrap());
            self.pos.advance(next_c);
        }

        match number.parse::<u32>() {
            Ok(parsed_number) => Ok(TokenKind::IntLit(parsed_number)),
            Err(e) => Err(anyhow!("Failed to parse number {}", e)),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}      Loc=<{}>", self.kind, self.pos)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Identifier(name) => write!(f, "identifier '{}'", name),
            TokenKind::Keyword(Return) => write!(f, "return 'return'"),
            TokenKind::Type(Int) => write!(f, "int 'int'"),
            TokenKind::Type(Void) => write!(f, "void 'void'"),
            TokenKind::Separator(LParen) => write!(f, "left parenthesis '('"),
            TokenKind::Separator(RParen) => write!(f, "right parenthesis ')'"),
            TokenKind::Separator(LBrace) => write!(f, "left brace '{{'"),
            TokenKind::Separator(RBrace) => write!(f, "right brace '}}'"),
            TokenKind::Separator(Semi) => write!(f, "semicolon ';'"),
            TokenKind::Operator(Plus) => write!(f, "plus '+'"),
            TokenKind::Operator(Minus) => write!(f, "minus '-'"),
            TokenKind::Operator(Star) => write!(f, "star '*'"),
            TokenKind::Operator(Slash) => write!(f, "slash '/'"),
            TokenKind::IntLit(value) => write!(f, "numeric_constant '{}'", value),
        }
    }
}

pub trait Precedence {
    fn precedence(&self) -> u8;
}

impl Precedence for Operator {
    fn precedence(&self) -> u8 {
        match self {
            Plus | Minus => 1,
            Star | Slash => 2,
        }
    }
}

impl Position {
    fn new(file: String) -> Self {
        Position { file, line: 1, col: 1 }
    }

    fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { file: "unknown".to_string(), line: 1, col: 1 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

fn keyword_token_map() -> HashMap<String, TokenKind> {
    let mut keywords = HashMap::new();

    keywords.insert("return".to_string(), TokenKind::Keyword(Return));
    keywords.insert("int".to_string(), TokenKind::Type(Int));
    keywords.insert("void".to_string(), TokenKind::Type(Void));

    keywords
}
