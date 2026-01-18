use std::error::Error;
use std::iter::Peekable;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
use super::Node;
use super::expressions::NumericExpression;
use crate::tokenizer::{Token, TokenType};

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct LetStatement {
    name: String,
    expr: NumericExpression,
}

impl LetStatement {
    pub fn create<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = &'a Token>,
    {
        let var = tokens
            .next()
            .ok_or("Syntax error: unexpected end of line")?;

        println!("{:?}", &var);
        let TokenType::Variable(var_name) = &var.kind else {
            return Err("Syntax Error: expected variable ".into());
        };

        let expr = NumericExpression {
            node: Node {
                line_num: 2,
                col_start: 3,
                col_end: 4,
            },
        };

        Ok(LetStatement {
            name: var_name.clone(),
            expr,
        })
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

        let mut iter_tokens = tokens.iter().peekable();
        let result = LetStatement::create(&mut iter_tokens);
        println!("{:#?}", &result)
    }
}
