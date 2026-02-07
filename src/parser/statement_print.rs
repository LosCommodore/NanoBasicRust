use super::Node;
use super::expressions::Expression;
use crate::tokenizer::Token;
use anyhow::Result;
use std::iter::Peekable;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize)]
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum PrintItem {
    String(String),
    ExpressionNode(Box<Node<Expression>>),
}

pub type PrintStatement = Vec<PrintItem>;
pub type PrintNode = Node<PrintStatement>;


#[allow(dead_code)]

pub fn parse_print_node<'a, I>(_tokens: &mut Peekable<I>) -> Result<PrintNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        todo!();
    }


impl PrintItem {
    #[allow(dead_code)]
    fn parse<'a, I>(_tokens: &mut Peekable<I>) -> Result<Node<PrintItem>>
    where
        I: Iterator<Item = &'a Token>,
    {
        todo!();
    }
}
