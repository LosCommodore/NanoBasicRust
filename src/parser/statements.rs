use super::Result;
use super::statement_if::IfStatement;
use super::statement_let::LetStatement;
use crate::tokenizer::{Token, TokenType};
use std::iter::Peekable;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum StatementEnum {
    Print,
    Return,
    If(Box<IfStatement>),
    GoSub,
    GoTo,
    Let(Box<LetStatement>),
}

use StatementEnum::*;


impl StatementEnum {
    pub fn from_token<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token: &Token = tokens.next().expect("Token not found");

        let statement = match token.kind {
            TokenType::Print => StatementEnum::Print,
            //TokenType::If => parse_if_statement(tokens)?,
            TokenType::Let => Let(Box::new(LetStatement::create(tokens)?)),
            TokenType::Goto => StatementEnum::GoTo,
            TokenType::Gosub => StatementEnum::GoSub,
            TokenType::Return => StatementEnum::Return,
            _ => return Err("error".into()),
        };
        Ok(statement)
    }
}
