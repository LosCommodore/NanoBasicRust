use super::Node;
use crate::tokenizer::TokenType;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct BooleanExpression {
    pub operator: TokenType,
    pub left_expr: NumericExpression,
    pub right_expr: NumericExpression,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct NumericExpression {
    pub node: Node,
}
