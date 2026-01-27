use super::expressions::NumericExpression;
use super::{Node, Result};
use crate::tokenizer::{Token, TokenType};
use std::iter::Peekable;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct LetStatement {
    name: Node<String>,
    expression: Node<NumericExpression>,
}

impl LetStatement {
    pub fn create<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut token = tokens
            .next()
            .ok_or("Syntax error: unexpected end of line")?;

        println!("{:?}", &token);
        let TokenType::Variable(var_name) = &token.kind else {
            return Err("Syntax Error: expected variable ".into());
        };

        let name = Node {
            col_start: token.col_start,
            line_num: token.line_num,
            col_end: token.col_end,
            content: var_name.clone(),
        };

        token = tokens
            .next()
            .ok_or("Syntax error: unexpected end of line")?;

        let TokenType::Equal = &token.kind else {
            return Err("Syntax Error: expected variable ".into());
        };

        // create numeric Expression here
        let expression = Node {
            line_num: token.line_num,
            col_start: token.col_start,
            col_end: 4,
            content: NumericExpression::NumberLiteral(42),
        };

        Ok(LetStatement { name, expression })
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
