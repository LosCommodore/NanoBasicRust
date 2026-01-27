//use std::iter::Peekable;
use super::Statement;
use super::expressions::BooleanExpression;
//use crate::tokenizer::{Token, TokenType};

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct IfStatement {
    boolean_expr: BooleanExpression,
    then_statement: Statement,
}

/*
impl IfStatement {
    pub fn create<'a, I>(_tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let boolean_expr = BooleanExpression {
            operator: TokenType::Equal,
            left_expr: NumericExpression {
                node: Node {
                    line_num: 2,
                    col_start: 3,
                    col_end: 4,
                },
                kind:
            },
            right_expr: NumericExpression {
                node: Node {
                    line_num: 2,
                    col_start: 3,
                    col_end: 4,
                },
            },
        };

        let then_statement: Statement = Statement {
            line_id: 42,
            node: Node {
                line_num: 2,
                col_start: 3,
                col_end: 4,
            },
            kind: StatementEnum::Print,
        };

        Ok(IfStatement {
            boolean_expr,
            then_statement,
        })
    }
}
*/
