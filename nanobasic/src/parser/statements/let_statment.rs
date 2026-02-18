use super::Node;
use crate::parser::expressions::{Expression, parse_expression};
use crate::tokenizer::{Token, TokenType};
use anyhow::{Result, anyhow};
use serde::Serialize;
use std::iter::Peekable;

#[derive(Serialize, Debug, PartialEq)]
pub struct LetStatement {
    name: String,
    expression: Node<Expression>,
}

impl LetStatement {
    pub fn parse<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Self>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // - Variable
        let mut token = tokens
            .next()
            .ok_or(anyhow!("Syntax error: unexpected end of line"))?;

        let mut position = token.position;

        println!("{:?}", &token);
        let TokenType::Variable(var_name) = &token.kind else {
            return Err(anyhow!("Syntax Error: expected variable "));
        };

        // Token Equal
        token = tokens
            .next()
            .ok_or(anyhow!("Syntax error: unexpected end of line"))?;

        let TokenType::Equal = &token.kind else {
            return Err(anyhow!("Syntax Error: expected variable"));
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
    use crate::tokenizer::{Position, Token, TokenType};

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
