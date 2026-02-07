use super::Node;
use super::super::expressions::Expression;
use crate::{parser::expressions::parse_expression, tokenizer::{Position, Token, TokenType}};
use anyhow::Result;
use serde::Serialize;
use std::{iter::Peekable};
use anyhow::anyhow;


#[derive(Serialize, Debug, PartialEq)]
pub enum Printable {
    String(String),
    ExpressionNode(Box<Expression>),
}

pub type Printables = Vec<Node<Printable>>;

fn parse_printable<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Printable>>
where
    I: Iterator<Item = &'a Token>,
{
    let position: Position;
    let token_preview =  tokens.peek().ok_or(anyhow!("Expected Printable"))?;
    let content = match token_preview.kind {
        TokenType::String(ref str) => {
            let token = tokens.next().expect("Token was peeked, now not found");
            position = token.position;
            Printable::String(str.clone())
        }
        _ => {
            let Node{content, position:pos} = parse_expression(tokens)?;
            position = pos;
            Printable::ExpressionNode(Box::new(content))
        }
    };
    let node = Node{content, position};

    Ok(node)
}


pub fn parse_printables<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Printables>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut printables =  Vec::new();
    loop 
    {
        let printable: Node<Printable> = parse_printable(tokens)?;
        printables.push(printable);

        if let Some(Token{kind:TokenType::Comma, ..}) = tokens.peek()
        { 
            tokens.next();
        } else {
            let mut position = printables[0].position;
            position.col_end = printables.last().unwrap().position.col_end;
            let node = Node {content:printables, position};
            return Ok(node) 
        };
    }
}

