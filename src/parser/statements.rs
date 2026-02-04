use super::Node;
use super::expressions::Expression;
use super::statement_if::{IfStatement};
use super::statement_let::LetStatement;
use crate::parser::expressions::parse_expression;
use crate::tokenizer::{Token};
use crate::tokenizer::TokenType as TT;
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
#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Box<Vec<Node<Expression>>>),
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
        let token: &Token = tokens.next().ok_or(anyhow!("Token not found"))?;
       
        let statement = match token.kind {
            TT::Print => todo!("implement print"),
            TT::If => If(Box::new(IfStatement::parse_node(tokens)?)),
            TT::Let => Let(Box::new(LetStatement::parse(tokens)?)),
            TT::Goto => GoTo(Box::new(parse_expression(tokens)?)),
            TT::Gosub => GoSub(Box::new(parse_expression(tokens)?)),
            TT::Return => Return,
            _ => return Err(anyhow!("error")),
        };
        Ok(statement)
    }
}
