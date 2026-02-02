use super::expressions::Expression;
use super::expressions::parse_expression;
use super::{Node};
use crate::tokenizer::{Token, TokenType};
use serde::Serialize;
use std::iter::Peekable;
use anyhow::{Result,anyhow};


#[derive(Serialize)]
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct LetStatement {
    name: String,
    expression: Node<Expression>,
}

impl LetStatement {
    pub fn create<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Self>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // - Variable
        let mut token = tokens
            .next()
            .ok_or(anyhow!("Syntax error: unexpected end of line"))?;

        let col_start = token.col_start;

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
        let col_end = expression.col_end;
        let content = LetStatement {
            name: var_name.clone(),
            expression,
        };
        let node = Node {
            content,
            line_num: token.line_num,
            col_start,
            col_end,
        };
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::statement_let::LetStatement;
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
    fn test_let_statement() {
        let tokens = vec![
            dummy_token(TokenType::Variable("ABC".to_string())),
            dummy_token(TokenType::Equal),
            dummy_token(TokenType::Number(42)),
        ];

        let mut iter_tokens: std::iter::Peekable<std::slice::Iter<'_, Token>> =
            tokens.iter().peekable();
        let result = LetStatement::create(&mut iter_tokens);
        println!("{:#?}", &result)
    }
}
