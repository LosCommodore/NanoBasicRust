pub mod expressions;
pub mod statements;

use crate::parser::statements::Statement;
use crate::tokenizer::{Position, Token, TokenType};
use anyhow::{Result, anyhow, bail};
use serde::Serialize;
use std::iter::Peekable;

/*
trait Parse: Sized {
    fn into_node<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Self>> where I: Iterator<Item = &'a Token>;
}
*/

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
        I: Iterator<Item=&'a Token>
    {
        let line_token = tokens.next().ok_or(anyhow!("Token not found"))?;

        let TokenType::Number(line_id) = line_token.kind else {
            bail!("Expected line number")
        };

        let statement = Statement::parse(tokens)?;
        Ok(Line { statement, line_id })
    }
}

pub fn parse<'a>(tokens: &[Token]) -> Result<Vec<Line>>

{
    let mut iter_token= tokens.iter().peekable();

    let mut lines = Vec::new();
    while let Some(_) = iter_token.peek() {
        let line = Line::parse(&mut iter_token)?;
        lines.push(line);
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::{Line, Result};
    use crate::tokenizer::tokenize;
    #[test]
    fn test_lines() -> Result<()> {
        // -- Read input
        let lines = [
            vec!["10 LET A = (2 + 3)*5 + B*-10".to_string()],
            vec!["20 GOTO 20+B".to_string()],
            vec!["30 GOTOSUB 40".to_string()],
            vec!["40 IF B<>33 THEN GOTO 42".to_string()],
        ];

        for line in &lines {
            println!("{:#?}", line);

            println!("* Tokenizing");
            let tokens = tokenize(&line)?;

            println!("* Parsing");
            let mut iter_token = tokens.iter().peekable();
            let result = Line::parse(&mut iter_token);
            println!("{:?}", result);
        }
        Ok(())
    }
}
