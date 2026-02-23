//use std::iter::Peekable;
use super::{Node, Statement};
use crate::parser::expressions::{Expression, parse_expression};
use crate::tokenizer::{Token, TokenType};
use crate::{ParseError, Result};
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
    then_statement: Node<Statement>,
}

fn parse_relational_operator<'a, I>(tokens: &mut Peekable<I>) -> Result<RelationalOperator>
where
    I: Iterator<Item = &'a Token>,
{
    use RelationalOperator::*;
    let token = tokens.next().ok_or(ParseError::UnexpectedEOF)?;

    let operator = match token.kind {
        TokenType::Equal => Equal,
        TokenType::NotEqual => NotEqual,
        TokenType::Greater => Greater,
        TokenType::GreaterEqual => GreaterEqual,
        TokenType::Less => Less,
        TokenType::LessEqual => LessEqual,
        _ => {
            return Err(ParseError::WrongToken {
                expected: "relational operator".to_string(),
                actual: format!("{:?}", token.kind),
            });
        }
    };

    Ok(operator)
}

fn parse_boolean_expression<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<BooleanExpression>>
where
    I: Iterator<Item = &'a Token>,
{
    let left_expr = parse_expression(tokens)?;
    let mut position = left_expr.position;
    let operator = parse_relational_operator(tokens)?;
    let right_expr = parse_expression(tokens)?;
    position.col_end = right_expr.position.col_end;

    let content = BooleanExpression {
        operator,
        left_expr,
        right_expr,
    };
    let node = Node { content, position };
    Ok(node)
}

impl IfStatement {
    pub fn parse_node<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Self>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let boolean_expr = parse_boolean_expression(tokens)?;
        let mut position = boolean_expr.position;
        let then_token = tokens.next().ok_or(ParseError::UnexpectedEOF)?;
        if then_token.kind != TokenType::Then {
            return Err(ParseError::WrongToken {
                expected: "THEN".to_string(),
                actual: format!("{:?}", then_token.kind),
            });
        };
        let then_statement = Statement::parse(tokens)?;
        position.col_end = 0; // TODO: fix

        let content = IfStatement {
            boolean_expr,
            then_statement,
        };
        let node = Node {
            content,
            position: position,
        };
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::IfStatement;
    use super::{Result, parse_boolean_expression};
    use crate::tokenizer::tokenize;

    #[test]
    fn test_boolean_expression() -> Result<()> {
        // -- Read input
        let lines = [vec!["A=3".to_string()], vec!["42>34".to_string()]];

        for line in &lines {
            println!("{:#?}", line);

            println!("* Tokenizing");
            let tokens = tokenize(&line)?;

            println!("* Parsing");
            let mut iter_token = tokens.iter().peekable();
            let result = parse_boolean_expression(&mut iter_token)?;
            println!("{result:#?}");
        }
        Ok(())
    }

    #[test]
    fn test_if() -> Result<()> {
        // -- Read input
        let lines = [vec!["IF A=3 THEN GOTO 42".to_string()]];

        for line in &lines {
            println!("{:#?}", line);

            println!("* Tokenizing");
            let tokens = tokenize(&line)?;

            println!("* Parsing");
            let mut iter_token = tokens.iter().peekable();
            let _if_token: Option<&crate::tokenizer::Token> = iter_token.next();
            let result = IfStatement::parse_node(&mut iter_token)?;
            println!("{result:#?}");
        }
        Ok(())
    }
}
