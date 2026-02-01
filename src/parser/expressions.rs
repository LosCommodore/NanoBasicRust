use super::{Node, Result};
use crate::tokenizer::{Token, TokenType};
use serde::Serialize;
use std::iter::Peekable;

#[derive(Serialize)]
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct BooleanExpression {
    pub operator: TokenType,
    pub left_expr: Node<Expression>,
    pub right_expr: Node<Expression>,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum UnaryOperationType {
    Minus,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum BinaryOperationType {
    Plus,
    Minus,
    Multiply,
    Devide,
}


/// Expression: evaluates to a single numericic value (=> NumericExpression in Pyhton code)
#[derive(Serialize, Debug, PartialEq)]
pub enum Expression {
    /// A numeric expression with two operands like 2 + 2 or 8 / 4
    BinaryOperation {
        left: Box<Node<Expression>>,
        right: Box<Node<Expression>>,
        operator: BinaryOperationType,
    },

    /// A numeric expression with one operand, like -4
    UnaryOperation {
        expression: Box<Node<Expression>>,
        operator: UnaryOperationType,
    },

    /// An integer written out in NanoBASIC code
    NumberLiteral(usize),

    /// A variable *name* that will have its value retrieved
    VarRetrieve(String),
}

/// FACTOR :=
/// Variable | Number | (Expression) | -FACTOR
pub fn parse_factor<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Expression>>
where
    I: Iterator<Item = &'a Token>,
{
    let first_token = tokens
        .next()
        .ok_or("Syntax error: unexpected end of line")?;

    let token = first_token;
    let this_node: Node<Expression> = match &token.kind {
        TokenType::Variable(var) => {
            let content = Expression::VarRetrieve(var.clone());
            Node::new(first_token, content)
        }

        TokenType::Number(num) => {
            let content = Expression::NumberLiteral(*num);
            Node::new(first_token, content)
        }

        TokenType::OpenParen => {
            let inner_node: Node<Expression> = parse_expression(tokens)?;

            let token = tokens
                .next()
                .ok_or("Syntax error: unexpected end of line. Expected ')'")?;

            let TokenType::CloseParen = token.kind else {
                return Err("Invalid Token. Expected ')'".into());
            };

            Node {
                content: inner_node.content,
                line_num: first_token.line_num,
                col_start: first_token.col_start,
                col_end: inner_node.col_end,
            }
        }

        TokenType::Minus => {
            let factor = parse_factor(tokens)?;
            let col_end = factor.col_end;
            let content = Expression::UnaryOperation {
                expression: Box::new(factor),
                operator: UnaryOperationType::Minus,
            };

            Node {
                content,
                line_num: token.line_num,
                col_start: token.col_start,
                col_end,
            }
        }
        _ => return Err("Unexpected token in numeric expression.".into()),
    };

    Ok(this_node)
}

/// Term :=
/// FACTOR *|/ FACTOR *|/ FACTOR ...
pub fn parse_term<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Expression>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_factor(tokens)?;
    loop {
        let Some(&token) = tokens.peek() else {
            return Ok(left);
        };

        left = match &token.kind {
            t @ (TokenType::Multiply | TokenType::Divide) => {
                _ = tokens.next().expect("Unexpected Error");
                let col_start = left.col_start;
                let operator = if *t == TokenType::Multiply {
                    BinaryOperationType::Multiply
                } else {
                    BinaryOperationType::Devide
                };

                let right = parse_factor(tokens)?;
                let col_end = right.col_end;
                Node {
                    content: Expression::BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator,
                    },
                    col_start,
                    col_end: col_end,
                    line_num: token.line_num,
                }
            }
            _ => {
                let ret = Ok(left);
                return ret;
            }
        }
    }
}

/// Expression :=
/// TERM +|- TERM +|- TERM ...
/// a term in this context is a chain of one or more factors
pub fn parse_expression<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Expression>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_term(tokens)?;
    loop {
        let Some(&token) = tokens.peek() else {
            return Ok(left);
        };

        println!("{:?}", token.kind);
        left = match &token.kind {
            t @ (TokenType::Plus | TokenType::Minus) => {
                _ = tokens.next().expect("Unexpected Error");
                let col_start = left.col_start;
                let operator = if *t == TokenType::Plus {
                    BinaryOperationType::Plus
                } else {
                    BinaryOperationType::Minus
                };

                let right = parse_term(tokens)?;
                let col_end = right.col_end;
                Node {
                    content: Expression::BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator,
                    },
                    col_start,
                    col_end: col_end,
                    line_num: token.line_num,
                }
            }
            _ => {
                let ret = Ok(left);
                return ret;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Result;
    use crate::parser::expressions::{parse_expression, parse_term};
    use crate::tokenizer::{Token, tokenize};

    #[test]
    pub fn test_parse_term_multiply() -> Result<()> {
        let lines = vec!["2 * 3".to_string()];
        let tokens = tokenize(&lines)?;
        let mut iter_tokens: std::iter::Peekable<std::slice::Iter<'_, Token>> =
            tokens.iter().peekable();
        let x = parse_term(&mut iter_tokens);
        println!("{:#?}", x);
        Ok(())
    }

    #[test]
    pub fn test_create_num_expr() -> Result<()> {
        let lines = vec!["2 + 3".to_string()];
        let tokens = tokenize(&lines)?;
        println!("tokens: \n{:#?}", tokens);

        let mut iter_tokens: std::iter::Peekable<std::slice::Iter<'_, Token>> =
            tokens.iter().peekable();

        let x = parse_expression(&mut iter_tokens)?;
        println!("{:#?}", x);
        assert!(true);
        Ok(())
    }
}
