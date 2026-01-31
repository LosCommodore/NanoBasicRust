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
#[derive(Serialize)]
#[allow(unused)]
#[derive(Debug, PartialEq)]
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

pub fn parse_line<'a, I>(tokens: &mut Peekable<I>) -> Result<Statement>
where
    I: Iterator<Item = &'a Token>,
{
    let line_token = tokens.next().expect("Token not found");

    let TokenType::Number(line_id) = line_token.kind else {
        return Err("Parse error".into());
    };

    println!("line line_id: {:?}", line_id);
    println!("line token: {:?}", line_token);

    Statement::new(tokens)
}

pub fn parse(tokens: &Vec<Token>) {
    let mut iter_token = tokens.iter().peekable();

    while let Some(_t) = iter_token.peek() {
        parse_line(&mut iter_token).expect("error while parsing line");
    }
}
