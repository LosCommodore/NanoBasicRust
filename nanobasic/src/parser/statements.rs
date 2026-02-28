pub mod if_statement;
pub mod let_statment;
pub mod print_statment;
use super::Node;
use super::expressions::Expression;
use super::expressions::parse_expression;
use super::tokenizer::Position;
use super::tokenizer::Token;
use super::tokenizer::TokenType;
use super::{ParseError, Result};
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
        use TokenType as TT;
        let token: &Token = tokens.next().ok_or(ParseError::UnexpectedEOF)?;

        let statement = match token.kind {
            TT::Print => {
                let Node { content, position } = parse_printables(tokens)?;
                let content = Print(Box::new(content));
                wrap_statement_in_node(content, token, position)
            }
            TT::If => {
                let Node { content, position } = IfStatement::parse_node(tokens)?;
                let content = If(Box::new(content));
                wrap_statement_in_node(content, token, position)
            }
            TT::Let => {
                let Node { content, position } = LetStatement::parse(tokens)?;
                let content = Let(Box::new(content));
                wrap_statement_in_node(content, token, position)
            }
            TT::Goto => {
                let Node { content, position } = parse_expression(tokens)?;
                let content = GoTo(Box::new(content));
                wrap_statement_in_node(content, token, position)
            }
            TT::Gosub => {
                let Node { content, position } = parse_expression(tokens)?;
                let content = GoSub(Box::new(content));
                wrap_statement_in_node(content, token, position)
            }
            TT::Return => Node {
                position: token.position,
                content: Return,
            },
            _ => {
                return Err(ParseError::WrongToken {
                    expected: "Statement".to_string(),
                    actual: format!("{:?}", token.kind),
                });
            }
        };
        Ok(statement)
    }
}

fn wrap_statement_in_node(content: Statement, token: &Token, postion: Position) -> Node<Statement> {
    let position = Position {
        col_end: postion.col_end,
        ..token.position
    };
    Node { content, position }
}
