pub mod if_statement;
pub mod let_statment;
pub mod print_statment;
use super::Node;
use super::expressions::Expression;
use if_statement::IfStatement;
use let_statment::LetStatement;
use print_statment::{parse_printables, Printables};
use crate::parser::expressions::parse_expression;
use crate::tokenizer::Token;
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
#[derive(Serialize, Debug, PartialEq)]
pub enum Statement {
    Print(Box<Node<Printables>>),
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
            TT::Print => Print(Box::new(parse_printables(tokens)?)),
            TT::If => If(Box::new(IfStatement::parse_node(tokens)?)),
            TT::Let => Let(Box::new(LetStatement::parse(tokens)?)),
            TT::Goto => GoTo(Box::new(parse_expression(tokens)?)),
            TT::Gosub => GoSub(Box::new(parse_expression(tokens)?)),
            TT::Return => Return,
            _ => return Err(anyhow!("Unknown Statement: '{:?}'", token.kind)),
        };
        Ok(statement)
    }
}
