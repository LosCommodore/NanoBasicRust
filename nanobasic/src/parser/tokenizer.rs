use super::{Result, ParseError};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum TokenType {
    Comment,
    Whitespace,
    Print,
    If,
    Then,
    Let,
    Goto,
    Gosub,
    Return,
    Comma,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParen,
    CloseParen,
    Variable(String),
    Number(usize),
    String(String),
}

const IGNORE_TOKEN_TYPES: &[TokenType] = &[TokenType::Whitespace, TokenType::Comment];

#[derive(Clone, Copy, Serialize, Debug, PartialEq)]
pub struct Position {
    pub line_num: usize, // line number (in text editor)
    pub col_start: usize,
    pub col_end: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub position: Position,
}

#[allow(unused)]
struct Case {
    regex_str: String,
    regex: Regex,
    capture: bool,
    ctor: fn(&str) -> TokenType,
}

macro_rules! case {
    ($regex:expr, $capture:expr, $ctor:expr) => {
        Case {
            regex_str: $regex.to_string(),
            regex: Regex::new(concat!("^", $regex)).unwrap(),
            capture: $capture,
            ctor: $ctor,
        }
    };
}

static CASES: Lazy<[Case; 25]> = Lazy::new(|| {
    [
        case!(r"(?i)rem.*", false, |_v| TokenType::Comment),
        case!(r"[ \t\n\r]", false, |_v| TokenType::Whitespace),
        case!(r"(?i)print", false, |_v| TokenType::Print),
        case!(r"(?i)if", false, |_v| TokenType::If),
        case!(r"(?i)then", false, |_v| TokenType::Then),
        case!(r"(?i)let", false, |_v| TokenType::Let),
        case!(r"(?i)goto", false, |_v| TokenType::Goto),
        case!(r"(?i)gosub", false, |_v| TokenType::Gosub),
        case!(r"(?i)return", false, |_v| TokenType::Return),
        case!(r",", false, |_v| TokenType::Comma),
        case!(r"=", false, |_v| TokenType::Equal),
        case!(r"<>|><", false, |_v| TokenType::NotEqual),
        case!(r"<=", false, |_v| TokenType::LessEqual),
        case!(r">=", false, |_v| TokenType::GreaterEqual),
        case!(r"<", false, |_v| TokenType::Less),
        case!(r">", false, |_v| TokenType::Greater),
        case!(r"\+", false, |_v| TokenType::Plus),
        case!(r"-", false, |_v| TokenType::Minus),
        case!(r"\*", false, |_v| TokenType::Multiply),
        case!(r"/", false, |_v| TokenType::Divide),
        case!(r"\(", false, |_v| TokenType::OpenParen),
        case!(r"\)", false, |_v| TokenType::CloseParen),
        case!(r"[A-Za-z_]+", true, |v| TokenType::Variable(v.to_string())),
        case!(r"-?[0-9]+", true, |v| TokenType::Number(v.parse().unwrap())),
        case!(r#"".*""#, true, |v| {
            let mut x = v.to_string();
            x.pop();
            x.remove(0);
            TokenType::String(x)
        }),
    ]
});

fn match_token(text: &str, col_start: usize, line_num: usize) -> Result<Token> {
    let token = CASES.iter().find_map(|case| {
        let m = case.regex.find(text)?;
        let content = &text[m.start()..m.end()];

        Some(Token {
            kind: (case.ctor)(content),
            position: Position {
                line_num,
                col_start: m.start() + col_start,
                col_end: m.end() + col_start,
            },
        })
    });

    token.ok_or(ParseError::UnkownToken {
        line_num,
        col_start,
        unkown_code: text.to_string(),
    })
}

fn tokenize_line(line: &str, line_num: usize) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut col = 0;
    while line.len() > col {
        let mut token = match_token(&line[col..], col, line_num)?;
        let offset = token.position.col_end - token.position.col_start;

        if !IGNORE_TOKEN_TYPES.contains(&token.kind) {
            token.position.line_num = line_num;
            tokens.push(token);
        }

        col += offset;
    }
    Ok(tokens)
}

/// Translate input into a Vec of Tokens
pub fn tokenize(lines: &[impl AsRef<str>]) -> Result<Vec<Token>> {
    let tokens = lines
        .iter()
        .enumerate()
        .map(|(i, line)| tokenize_line(line.as_ref(), i))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(tokens)
}

// Outsource Unittests to extra file:
#[cfg(test)]
#[path = "tokenizer_tests.rs"]
mod tests;
