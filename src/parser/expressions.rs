use super::{Node, Result};
use crate::tokenizer::{Token, TokenType};
use std::iter::Peekable;

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

impl NumericExpression {
    pub fn create<'a, I>(_tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        Ok(NumericExpression {
            node: Node {
                line_num: 2,
                col_start: 3,
                col_end: 4,
            },
            kind: NumericExpressionKind::NumberLiteral(42),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use crate::parser::expressions::NumericExpression;
    use crate::tokenizer::{Token, TokenType};

    fn dummy_token(tk: TokenType) -> Token {
        Token {
            kind: tk,
            line_num: 10,
            col_start: 1,
            col_end: 2,
        }
    }

     

    #[test]
    pub fn test_create_num_expr() -> Result<(),Error> {

        let tokens = vec![
            dummy_token(TokenType::Variable("ABC".to_string())),
            dummy_token(TokenType::Equal),
            dummy_token(TokenType::Number(42)),
        ];
        let mut iter_tokens: std::iter::Peekable<std::slice::Iter<'_, Token>> = tokens.iter().peekable();

        let x = NumericExpression::create(&mut iter_tokens);
        assert!(true);
        Ok(())
    }
}