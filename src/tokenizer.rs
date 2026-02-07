use anyhow::{Result, anyhow};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        case!(r".*", true, |v| TokenType::String(v.to_string())),
    ]
});

fn match_token(text: &str, col_start: usize) -> Result<Token> {
    fn find_token(case: &Case, text: &str, col_start: usize) -> Option<Token> {
        let m = case.regex.find(text)?;
        let content = &text[m.start()..m.end()];

        let position = Position {
            line_num: 0,
            col_start: m.start() + col_start,
            col_end: m.end() + col_start,
        };

        Some(Token {
            kind: (case.ctor)(content),
            position,
        })
    }

    let token = CASES
        .iter()
        .find_map(|case| find_token(case, text, col_start));

    let token = token.ok_or(anyhow!("Syntax Error"));
    token
}

fn match_line(line: &str, line_num: usize) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut col = 0;
    while line.len() > col {
        let mut token = match_token(&line[col..], col)?;
        //println!("found token: {:?}", token);
        let offset = token.position.col_end - token.position.col_start;

        if !IGNORE_TOKEN_TYPES.contains(&token.kind) {
            token.position.line_num = line_num;
            tokens.push(token);
        }

        col += offset;
    }
    Ok(tokens)
}

pub fn read_file(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let out: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    Ok(out)
}

pub fn tokenize(lines: &Vec<String>) -> Result<Vec<Token>> {
    let tokens = lines
        .iter()
        .enumerate()
        .map(|(i, line)| match_line(line, i))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_token() {
        let params = [
            (
                r"rem hallo",
                Token {
                    kind: TokenType::Comment,
                    position: Position {
                        line_num: 0,
                        col_start: 0,
                        col_end: 9,
                    },
                },
            ),
            (
                r"REM HaLLo",
                Token {
                    kind: TokenType::Comment,
                    position: Position {
                        line_num: 0,
                        col_start: 0,
                        col_end: 9,
                    },
                },
            ),
            (
                r"goto",
                Token {
                    kind: TokenType::Goto,
                    position: Position {
                        line_num: 0,
                        col_start: 0,
                        col_end: 4,
                    },
                },
            ),
            (
                r")",
                Token {
                    kind: TokenType::CloseParen,
                    position: Position {
                        line_num: 0,
                        col_start: 0,
                        col_end: 1,
                    },
                },
            ),
            (
                r"ABC",
                Token {
                    kind: TokenType::Variable("ABC".to_string()),
                    position: Position {
                        line_num: 0,
                        col_start: 0,
                        col_end: 3,
                    },
                },
            ),
        ];

        for (text, result) in &params {
            println!("Using regex: {}", *text);
            assert_eq!(match_token(*text, 0).unwrap(), *result);
        }
    }

    #[test]
    fn test_match_line() {
        let param = r"a = 3";

        let expected = [
            Token {
                kind: TokenType::Variable("a".to_string()),
                position: Position {
                    line_num: 0,
                    col_start: 0,
                    col_end: 1,
                },
            },
            Token {
                kind: TokenType::Equal,
                position: Position {
                    line_num: 0,
                    col_start: 2,
                    col_end: 3,
                },
            },
            Token {
                kind: TokenType::Number(3),
                position: Position {
                    line_num: 0,
                    col_start: 4,
                    col_end: 5,
                },
            },
        ];

        println!("Using regex: {}", param);
        let m = match_line(param, 0).unwrap();
        print!("{:#?}", m);
        assert_eq!(expected, *m);
    }
}
