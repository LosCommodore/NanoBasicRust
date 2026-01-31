use super::Result;
use super::statement_if::IfStatement;
use super::statement_let::LetStatement;
use crate::tokenizer::{Token, TokenType};
use std::iter::Peekable;
use super::Node;
use serde::Serialize;

#[derive(Serialize)]
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum Statement {
    Print,
    Return,
    If(Box<Node<IfStatement>>),
    GoSub,
    GoTo,
    Let(Box<Node<LetStatement>>),
}

use Statement::*;

impl Statement {
    pub fn new<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token: &Token = tokens.next().expect("Token not found");

        let statement = match token.kind {
            //TokenType::Print => Statement::Print,
            //TokenType::If => parse_if_statement(tokens)?,
            TokenType::Let => Let(Box::new(LetStatement::create(tokens)?)),
            //TokenType::Goto => Statement::GoTo,
            //TokenType::Gosub => Statement::GoSub,
            //TokenType::Return => Statement::Return,
            _ => return Err("error".into()),
        };
        Ok(statement)
    }
}