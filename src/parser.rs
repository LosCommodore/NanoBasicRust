pub mod expressions;
pub mod statement_if;
pub mod statement_let;
pub mod statements;

use crate::parser::statements::StatementEnum;
use crate::tokenizer::{Token, TokenType};
use std::error::Error;
use std::iter::Peekable;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Represents command like "if", "let, "goto" ...
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct Statement {
    line_id: usize,
    node: Node,
    kind: StatementEnum,
}

/// Represents postion information in the code
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct Node {
    line_num: usize, // line number (in text editor)
    col_start: usize,
    col_end: usize,
}

fn parse_line<'a, I>(tokens: &mut Peekable<I>) -> Result<Statement>
where
    I: Iterator<Item = &'a Token>,
{
    let line_token = tokens.next().expect("Token not found");

    let TokenType::Number(line_id) = line_token.kind else {
        return Err("Parse error".into());
    };

    println!("line line_id: {:?}", line_id);
    println!("line token: {:?}", line_token);

    let kind = StatementEnum::from_token(tokens)?;

    let node = Node {
        line_num: line_token.line_num,
        col_start: 1,
        col_end: 1,
    };
    let statement = Statement {
        node,
        line_id,
        kind,
    };
    Ok(statement)
}

pub fn parse(tokens: &Vec<Token>) {
    let mut iter_token = tokens.iter().peekable();

    while let Some(_t) = iter_token.peek() {
        parse_line(&mut iter_token).expect("error while parsing line");
    }
}
