pub mod expressions;
pub mod statements;

use crate::parser::statements::Statement;
use crate::tokenizer::{Position, Token, TokenType, tokenize};
use crate::{ParseError, Result};
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::path::Path;
use std::sync::Arc;

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
    pub statement: Arc<Node<Statement>>,
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
            statement: Arc::new(statement),
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
