use std::{collections::HashMap, fmt, iter::Peekable, str::Chars};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

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
    IntLit(u32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Void,
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
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                ';' => TokenKind::Semi,
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
            TokenKind::Return => write!(f, "return 'return'"),
            TokenKind::Type(Type::Int) => write!(f, "int 'int'"),
            TokenKind::Type(Type::Void) => write!(f, "void 'void'"),
            TokenKind::LParen => write!(f, "l_paren '('"),
            TokenKind::RParen => write!(f, "r_paren ')'"),
            TokenKind::LBrace => write!(f, "l_brace '{{'"),
            TokenKind::RBrace => write!(f, "r_brace '}}'"),
            TokenKind::Semi => write!(f, "semi ';'"),
            TokenKind::IntLit(value) => write!(f, "numeric_constant '{}'", value),
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

    keywords.insert("return".to_string(), TokenKind::Return);
    keywords.insert("int".to_string(), TokenKind::Type(Type::Int));
    keywords.insert("void".to_string(), TokenKind::Type(Type::Void));

    keywords
}
