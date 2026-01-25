use super::{Node, Result};
use crate::tokenizer::{Token, TokenType};
use std::{iter::Peekable};

// # A numeric expression is something that can be computed into a number.
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct NumericExpression {
    pub node: Node,
    pub kind: NumericExpressionKind,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct BooleanExpression {
    pub operator: TokenType,
    pub left_expr: NumericExpression,
    pub right_expr: NumericExpression,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum NumericExpressionKind {
    /// A numeric expression with two operands like 2 + 2 or 8 / 4
    BinaryOperation {
        left: Box<NumericExpression>,
        right: Box<NumericExpression>,
        operator: char,
    },

    /// A numeric expression with one operand, like -4
    UnaryOperation {
        expression: Box<NumericExpression>,
        operator: char,
    },

    /// An integer written out in NanoBASIC code
    NumberLiteral(usize),

    /// A variable *name* that will have its value retrieved
    VarRetrieve(String),
}

impl NumericExpressionKind {
    pub fn create<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokens
            .next()
            .ok_or("Syntax error: unexpected end of line")?;

        let output = match &token.kind {
            TokenType::Variable(num) => NumericExpressionKind::VarRetrieve(num.clone()),
            _ => return Err("Invalid Token".into())
        };

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::Result;
    use crate::parser::expressions::{NumericExpressionKind};
    use crate::tokenizer::{Token, tokenize};

    #[test]
    pub fn test_create_num_expr() -> Result<()> {
        let lines = vec!["2+3".to_string()];
        let tokens = tokenize(&lines)?;
        // println!("tokens: \n{:#?}",tokens);

        let mut iter_tokens: std::iter::Peekable<std::slice::Iter<'_, Token>> =
            tokens.iter().peekable();

        let x = NumericExpressionKind::create(&mut iter_tokens);
        println!("{:#?}", x);
        assert!(true);
        Ok(())
    }
}
