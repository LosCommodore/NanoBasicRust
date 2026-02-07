//use std::iter::Peekable;
use super::expressions::{BooleanExpression, RelationalOperator};
use super::{Node, Statement};
use crate::parser::expressions::Expression;
use crate::tokenizer::Token;
use anyhow::Result;
use serde::Serialize;
use std::iter::Peekable;

/// 'IF' <boolean-expr> 'THEN' <statement>
#[derive(Serialize)]
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct IfStatement {
    boolean_expr: Node<BooleanExpression>,
    then_statement: Statement,
}

fn parse_boolean_expression<'a, I>(_tokens: &mut Peekable<I>) -> Result<Node<BooleanExpression>>
where
    I: Iterator<Item = &'a Token>,
{
    let left = Expression::NumberLiteral(32);
    let right = Expression::NumberLiteral(32);
    let left_node = Node {
        col_end: 1,
        col_start: 1,
        line_num: 1,
        content: left,
    };
    let right_node = Node {
        content: right,
        ..left_node
    };
    let _dummy_boolean_expression = BooleanExpression {
        operator: RelationalOperator::Equal,
        left_expr: left_node,
        right_expr: right_node,
    };

    todo!();
    // let node = Node {content: dummy_boolean_expression, .. left_node};
    // Ok(dummy_boolean_expression)
}

impl IfStatement {
    pub fn parse_node<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Self>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let _boolean_expr_ = parse_boolean_expression(tokens)?;

        /*
        let statement = IfStatement {
            boolean_expr,
            then_statement: todo!(),
        };
        */
        todo!();
        //Ok(statement)
    }
}
