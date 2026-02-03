use super::Node;
use super::expressions::Expression;
use super::statement_if::IfStatement;
use super::statement_let::LetStatement;
use crate::parser::expressions::parse_expression;
use crate::tokenizer::{Token, TokenType};
use anyhow::{Result, anyhow};
use serde::Serialize;
use std::iter::Peekable;

/// <statement> ::=
///    'PRINT' <expr-list>
///  | 'IF'    <boolean-expr> 'THEN' <statement>
///  | 'GOTO'  <expression>
///  | 'LET'   <var> = <expression>
///  | 'GOSUB' <expression>
///  | 'RETURN'
///
#[derive(Serialize)]
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Box<Vec<Expression>>),
    If(Box<Node<IfStatement>>),
    GoSub(Box<Node<Expression>>),
    GoTo(Box<Node<Expression>>),
    Let(Box<Node<LetStatement>>),
    Return,
}

use Statement::*;

impl Statement {
    /// Parse statement from tokens
    pub fn parse<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token: &Token = tokens.next().expect("Token not found");

        let statement = match token.kind {
            //TokenType::Print => Statement::Print,
            //TokenType::If => parse_if_statement(tokens)?,
            TokenType::Let => Let(Box::new(LetStatement::create(tokens)?)),
            TokenType::Goto => Statement::GoTo(Box::new(parse_expression(tokens)?)),
            TokenType::Gosub => Statement::GoSub(Box::new(parse_expression(tokens)?)),
            TokenType::Return => Statement::Return,
            _ => return Err(anyhow!("error")),
        };
        Ok(statement)
    }
}
