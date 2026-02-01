pub mod expressions;
pub mod statement_if;
pub mod statement_let;
pub mod statements;

use crate::parser::statements::Statement;
use crate::tokenizer::{Token, TokenType};
use std::error::Error;
use std::iter::Peekable;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
use serde::Serialize;

/// Represents postion information in the code
#[derive(Serialize, Debug, PartialEq)]
pub struct Node<T> {
    line_num: usize, // line number (in text editor)
    col_start: usize,
    col_end: usize,
    content: T,
}

impl<T> Node<T> {
    fn new(token: &Token, content: T) -> Self {
        Node {
            col_start: token.col_start,
            line_num: token.line_num,
            col_end: token.col_end,
            content,
        }
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Line {
    line_id: usize,
    statement: Statement,
}

impl Line {
    /// Parse a line from tokens
    /// 
    /// Syntax Line:
    /// <line>::= <number> <statement> "\n" | "REM" .* \n
    /// 
    /// - Comments are already excluded by the tokenizer
    pub fn parse<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let line_token = tokens.next().expect("Token not found");

        let TokenType::Number(line_id) = line_token.kind else {
            return Err("Expected line number".into());
        };

        let statement = Statement::parse(tokens)?;
        Ok(Line { statement, line_id })
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, Result};
    use crate::tokenizer::tokenize;
    #[test]
    fn test_somehting() -> Result<()> {
        // -- Read input
        let txt: Vec<String> = vec!["10 LET A = (2 + 3)*5 + B*-10".to_string()];
        let tokens = tokenize(&txt)?;

        println!("{:#?}", tokens);
        let mut iter_token = tokens.iter().peekable();

        let _result = Line::parse(&mut iter_token);
        Ok(())
    }
}
