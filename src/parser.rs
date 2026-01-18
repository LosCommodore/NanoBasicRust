pub mod expressions;
pub mod statement_if;
pub mod statement_let;

use crate::tokenizer::{Token, TokenType};
use statement_if::IfStatement;
use statement_let::LetStatement;
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
    line_num: usize,
    col_start: usize,
    col_end: usize,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum StatementEnum {
    Print,
    Return,
    If(Box<IfStatement>),
    GoSub,
    GoTo,
    Let(Box<LetStatement>),
}

fn parse_let_statement<'a, I>(tokens: &mut Peekable<I>) -> Result<StatementEnum>
where
    I: Iterator<Item = &'a Token>,
{
    let let_statement = LetStatement::create(tokens)?;
    let statement_enum = StatementEnum::Let(Box::new(let_statement));
    Ok(statement_enum)
}

/*
fn parse_if_statement<'a, I>(tokens: &mut Peekable<I>) -> Result<StatementEnum>
where
    I: Iterator<Item = &'a Token>,
{
    let if_statement = IfStatement::create(tokens)?;
    Ok(StatementEnum::If(Box::new(if_statement)))
}
*/

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

    let token = tokens.next().expect("Token not found");

    let kind: StatementEnum = match token.kind {
        TokenType::Print => StatementEnum::Print,
        //TokenType::If => parse_if_statement(tokens)?,
        TokenType::Let => parse_let_statement(tokens)?,
        TokenType::Goto => StatementEnum::GoTo,
        TokenType::Gosub => StatementEnum::GoSub,
        TokenType::Return => StatementEnum::Return,
        _ => return Err("error".into()),
    };

    let node = Node {
        line_num: 1,
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
