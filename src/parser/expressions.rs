use super::{Node, Result};
use crate::tokenizer::{Token, TokenType};
use std::iter::Peekable;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct BooleanExpression {
    pub operator: TokenType,
    pub left_expr: Node<NumericExpression>,
    pub right_expr: Node<NumericExpression>,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum NumericExpression {
    /// A numeric expression with two operands like 2 + 2 or 8 / 4
    BinaryOperation {
        left: Box<Node<NumericExpression>>,
        right: Box<Node<NumericExpression>>,
        operator: char,
    },

    /// A numeric expression with one operand, like -4
    UnaryOperation {
        expression: Box<Node<NumericExpression>>,
        operator: char,
    },

    /// An integer written out in NanoBASIC code
    NumberLiteral(usize),

    /// A variable *name* that will have its value retrieved
    VarRetrieve(String),
}

impl NumericExpression {
    pub fn parse_factor<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokens
            .next()
            .ok_or("Syntax error: unexpected end of line")?;

        let output = match &token.kind {
            TokenType::Variable(var) => NumericExpression::VarRetrieve(var.clone()),
            TokenType::Number(num) => NumericExpression::NumberLiteral(*num),
            TokenType::OpenParen => {
                let expr = NumericExpression::parse_term(tokens)?;

                let TokenType::CloseParen = token.kind else {
                    return Err("Invalid Token. Expected ')'".into());
                };

                expr
            }
            _ => return Err("Invalid Token".into()),
        };

        Ok(output)
    }

    pub fn parse_term<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let factor = NumericExpression::parse_factor(tokens);
        factor
    }
}

#[cfg(test)]
mod tests {
    use super::Result;
    use crate::parser::expressions::NumericExpression;
    use crate::tokenizer::{Token, tokenize};

    #[test]
    pub fn test_create_num_expr() -> Result<()> {
        let lines = vec!["2+3".to_string()];
        let tokens = tokenize(&lines)?;
        // println!("tokens: \n{:#?}",tokens);

        let mut iter_tokens: std::iter::Peekable<std::slice::Iter<'_, Token>> =
            tokens.iter().peekable();

        let x = NumericExpression::parse_term(&mut iter_tokens);
        println!("{:#?}", x);
        assert!(true);
        Ok(())
    }
}
