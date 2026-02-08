pub mod if_statement;
pub mod let_statment;
pub mod print_statment;
use super::Node;
use super::expressions::Expression;
use crate::parser::expressions::parse_expression;
use crate::tokenizer::Token;
use crate::tokenizer::TokenType as TT;
use anyhow::{Result, anyhow};
use if_statement::IfStatement;
use let_statment::LetStatement;
use print_statment::{Printables, parse_printables};
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
    Print(Box<Printables>),
    If(Box<IfStatement>),
    GoSub(Box<Expression>),
    GoTo(Box<Expression>),
    Let(Box<LetStatement>),
    Return,
}

use Statement::*;

impl Statement {
    /// Parse statement from tokens
    pub fn parse<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Self>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token: &Token = tokens.next().ok_or(anyhow!("Token not found"))?;

        let statement = match token.kind {
            TT::Print => {
                let Node {content, position} =    parse_printables(tokens)?;
                Node{position, content: Print(Box::new(content)) }
            }
            TT::If =>  {
                let Node {content, position} =    IfStatement::parse_node(tokens)?;
                Node{position, content: If(Box::new(content)) }

            },
            TT::Let => {
                let Node {content, position} =  LetStatement::parse(tokens)?;
                Node{position, content: Let(Box::new(content)) }
            },
            TT::Goto => {
                let Node {content, position} = parse_expression(tokens)?;
                Node{position, content: GoTo(Box::new(content)) }
            },
            TT::Gosub => {
                let Node {content, position} = parse_expression(tokens)?;
                Node{position, content: GoSub(Box::new(content)) }
            },
            TT::Return => {
                Node{position: token.position, content: Return} 
            }
            _ => return Err(anyhow!("Unknown Statement: '{:?}'", token.kind)),
        };
        Ok(statement)
    }
}
