pub mod expressions;
pub mod statements;

use crate::{Result, ParseError};
use crate::parser::statements::Statement;
use crate::tokenizer::{Position, Token, TokenType};
use serde::Serialize;
use std::iter::Peekable;

/// Represents postion information in the code
#[derive(Serialize, Debug, PartialEq)]
pub struct Node<T> {
    position: Position,
    content: T,
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
    line_id: usize,
    statement: Node<Statement>,
}

/// Parse a line from tokens
impl Line {
    pub fn parse<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let line_token = tokens.next().ok_or(ParseError::UnexpectedEOF)?;

        let TokenType::Number(line_id) = line_token.kind else {
            return Err(ParseError::WrongToken { expected: "Line numer".to_string(), actual: format!("{:?}", line_token.kind)})
        };

        let statement = Statement::parse(tokens)?;
        Ok(Line { statement, line_id })
    }
}

/// Parse Tokens into an Abstract Syntax Tree (List of Line)
pub fn parse<'a>(tokens: &[Token]) -> Result<Vec<Line>> {
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

// Outsource Unittests to extra file:
#[cfg(test)]
#[path = "parser_tests.rs"]
mod tests;
