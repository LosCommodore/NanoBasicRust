//use std::iter::Peekable;
use super::{Node, Statement};
use crate::parser::expressions::{Expression, parse_expression};
use crate::tokenizer::{Token, TokenType};
use anyhow::{Result, anyhow, bail};
use serde::Serialize;
use std::iter::Peekable;

/// Relationaloparator ::= <relop>
#[derive(Serialize, Debug, PartialEq)]
pub enum RelationalOperator {
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct BooleanExpression {
    pub operator: RelationalOperator,
    pub left_expr: Node<Expression>,
    pub right_expr: Node<Expression>,
}

/// 'IF' <boolean-expr> 'THEN' <statement>
#[derive(Serialize)]
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct IfStatement {
    boolean_expr: Node<BooleanExpression>,
    then_statement: Statement,
}

fn parse_relational_operator<'a, I>(tokens: &mut Peekable<I>) -> Result<RelationalOperator>
where
    I: Iterator<Item = &'a Token>,
{
    use RelationalOperator::*;
    let token = tokens.next().ok_or(anyhow!("Unexpeted end of line"))?;

    let operator = match token.kind {
        TokenType::Equal => Equal,
        TokenType::NotEqual => NotEqual,
        TokenType::Greater => Greater,
        TokenType::GreaterEqual => GreaterEqual,
        TokenType::Less => Less,
        TokenType::LessEqual => LessEqual,
        _ => bail!("Expected Relational Operator"),
    };

    Ok(operator)
}

fn parse_boolean_expression<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<BooleanExpression>>
where
    I: Iterator<Item = &'a Token>,
{
    let left_expr = parse_expression(tokens)?;
    let col_start = left_expr.col_start;
    let line_num = left_expr.line_num;
    let operator = parse_relational_operator(tokens)?;
    let right_expr = parse_expression(tokens)?;
    let col_end = right_expr.col_end;
    let content = BooleanExpression {
        operator,
        left_expr,
        right_expr,
    };
    let node = Node {
        content,
        line_num,
        col_start,
        col_end,
    };
    Ok(node)
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

#[cfg(test)]
mod tests {
    use super::{Result, parse_boolean_expression};
    use crate::tokenizer::tokenize;

    #[test]
    fn test_boolean_expression() -> Result<()> {
        // -- Read input
        let lines = [
            vec!["A=3".to_string()],
            //vec!["42>34".to_string()],
        ];

        for line in &lines {
            println!("{:#?}", line);

            println!("* Tokenizing");
            let tokens = tokenize(&line)?;

            println!("* Parsing");
            let mut iter_token = tokens.iter().peekable();
            let result = parse_boolean_expression(&mut iter_token);
            println!("{result:#?}");
        }
        Ok(())
    }
}
