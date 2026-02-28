use super::Node;
use crate::parser::expressions::{Expression, parse_expression};
use crate::parser::tokenizer::{Token, TokenType};
use super::{Result, ParseError};
use serde::Serialize;
use std::iter::Peekable;

#[derive(Serialize, Debug, PartialEq)]
pub struct LetStatement {
    pub name: String,
    pub expression: Node<Expression>,
}

impl LetStatement {
    pub fn parse<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Self>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // - Variable
        let mut token = tokens.next().ok_or(ParseError::UnexpectedEOF)?;

        let mut position = token.position;

        let TokenType::Variable(var_name) = &token.kind else {
            return Err(ParseError::WrongToken {
                expected: "Variable".to_string(),
                actual: format!("{:?}", token.kind),
            });
        };

        // Token Equal
        token = tokens.next().ok_or(ParseError::UnexpectedEOF)?;

        let TokenType::Equal = &token.kind else {
            return Err(ParseError::WrongToken {
                expected: "Variable".to_string(),
                actual: format!("{:?}", token.kind),
            });
        };

        // create numeric Expression here
        let expression = parse_expression(tokens)?;
        position.col_end = expression.position.col_end;

        let content = LetStatement {
            name: var_name.clone(),
            expression,
        };
        Ok(Node { content, position })
    }
}

#[cfg(test)]
mod tests {
    use super::LetStatement;
    use crate::parser::tokenizer::{Position, Token, TokenType};

    fn dummy_token(tk: TokenType) -> Token {
        Token {
            kind: tk,
            position: Position {
                line_num: 10,
                col_start: 1,
                col_end: 2,
            },
        }
    }

    #[test]
    fn test_let_statement() {
        let tokens = vec![
            dummy_token(TokenType::Variable("ABC".to_string())),
            dummy_token(TokenType::Equal),
            dummy_token(TokenType::Number(42)),
        ];

        let mut iter_tokens: std::iter::Peekable<std::slice::Iter<'_, Token>> =
            tokens.iter().peekable();
        let result = LetStatement::parse(&mut iter_tokens);
        println!("{:#?}", &result)
    }
}
