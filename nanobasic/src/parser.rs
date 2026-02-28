pub mod expressions;
pub mod statements;
pub mod tokenizer;

use self::tokenizer::{Position, Token, TokenType, tokenize};
use crate::parser::statements::Statement;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::path::Path;
use std::rc::Rc;
use std::io;
use std::path::PathBuf;
use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("File '{path}' could be read")]
    FileOpen { source: io::Error, path: PathBuf },

    #[error(
        "Unknown token '{unkown_code}' at line: {line_num:?}, starting at column: {col_start:?})"
    )]
    UnkownToken {
        line_num: usize,
        col_start: usize,
        unkown_code: String,
    },

    #[error("Unexpected end of file")]
    UnexpectedEOF,

    #[error("Wrong Token, expected: {expected}, actual: {actual}")]
    WrongToken { expected: String, actual: String },
}

pub type Result<T> = result::Result<T, ParseError>;

/// Represents postion information in the code
#[derive(Serialize, Debug, PartialEq)]
pub struct Node<T> {
    pub position: Position,
    pub content: T,
}

impl<T> Node<T> {
    fn new(token: &Token, content: T) -> Self {
        Node {
            content,
            position: token.position,
        }
    }
}

/// Syntax Line:
/// <line>::= <number> <statement> "\n" | "REM" .* \n
///
/// - Comments are already excluded by the tokenizer
#[derive(Serialize, Debug, PartialEq)]
pub struct Line {
    pub line_id: usize,
    pub statement: Rc<Node<Statement>>,
}

/// Parse a line from tokens
impl Line {
    pub fn parse<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let line_token = tokens.next().ok_or(ParseError::UnexpectedEOF)?;

        let TokenType::Number(line_id) = line_token.kind else {
            return Err(ParseError::WrongToken {
                expected: "Line numer".to_string(),
                actual: format!("{:?}", line_token.kind),
            });
        };

        let statement = Statement::parse(tokens)?;
        Ok(Line {
            statement: Rc::new(statement),
            line_id,
        })
    }
}

/// Parse Tokens into an Abstract Syntax Tree (List of Line)
pub fn parse_tokens<'a>(tokens: &[Token]) -> Result<Vec<Line>> {
    let mut iter_token = tokens.iter().peekable();

    let mut lines = Vec::new();
    loop {
        if iter_token.peek().is_none() {
            break;
        }
        let line = Line::parse(&mut iter_token)?;
        lines.push(line);
    }
    Ok(lines)
}

pub fn parse_file(path: impl AsRef<Path>) -> Result<Vec<Line>> {
    let path = path.as_ref();

    log::info!(r#"Opening file"{path:#?}""#);
    let file = File::open(path).map_err(|e| ParseError::FileOpen {
        source: e,
        path: path.into(),
    })?;
    let reader = BufReader::new(file);

    log::info!(r"Tokenizing");
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let tokens = tokenize(&lines)?;

    log::info!(r"Parsing");
    parse_tokens(&tokens)
}

// Outsource Unittests to extra file:
#[cfg(test)]
#[path = "parser_tests.rs"]
mod tests;
