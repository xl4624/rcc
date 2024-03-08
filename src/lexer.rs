use std::{
    fmt::{self, Display, Formatter},
    iter::Peekable,
    str::Chars,
};

#[derive(Debug)]
pub enum Token {
    // Separators
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semi,

    // Keywords
    Int,
    Return,

    Ident(String),

    // Literals
    IntLit(u32),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::LBrace => write!(f, "l_brace '{{'"),
            Token::RBrace => write!(f, "r_brace '}}'"),
            Token::LParen => write!(f, "l_paren '('"),
            Token::RParen => write!(f, "r_paren ')'"),
            Token::Semi => write!(f, "semi ';'"),
            Token::Int => write!(f, "int 'int'"),
            Token::Return => write!(f, "return 'return'"),
            Token::Ident(ident) => write!(f, "identifier '{}'", ident),
            Token::IntLit(lit) => write!(f, "numeric_constant '{}'", lit),
        }
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let token = match c {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ';' => Token::Semi,
            _ if c.is_whitespace() => continue,
            _ if c.is_alphabetic() => lex_word(&mut chars, c)?,
            _ if c.is_numeric() => lex_number(&mut chars, c)?,
            _ => return Err(format!("LEXER ERROR: Unexpected character {}", c)),
        };
        tokens.push(token);
    }
    Ok(tokens)
}

fn lex_word(chars: &mut Peekable<Chars>, c: char) -> Result<Token, String> {
    let mut identifier = c.to_string();
    while let Some(&next_c) = chars.peek() {
        if next_c.is_ascii_alphanumeric() || next_c == '_' {
            identifier.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    let token = match identifier.as_str() {
        "int" => Token::Int,
        "return" => Token::Return,
        _ => Token::Ident(identifier),
    };
    Ok(token)
}

fn lex_number(chars: &mut Peekable<Chars>, c: char) -> Result<Token, String> {
    let mut number_str = c.to_string();

    while let Some(&next_c) = chars.peek() {
        if next_c.is_numeric() {
            number_str.push(chars.next().unwrap());
            chars.next();
        } else {
            break;
        }
    }
    let number: u32 = number_str
        .parse()
        .map_err(|_| format!("LEXER ERROR: Failed to parse integer literal '{}'", number_str))?;
    Ok(Token::IntLit(number))
}
