use super::super::expressions::Expression;
use super::Node;
use crate::{ParseError, Result};
use crate::{
    parser::expressions::parse_expression,
    tokenizer::{Position, Token, TokenType},
};
use serde::Serialize;
use std::iter::Peekable;

#[derive(Serialize, Debug, PartialEq)]
pub enum Printable {
    String(String),
    ExpressionNode(Box<Expression>),
}

pub type Printables = Vec<Node<Printable>>;

fn parse_one_printable<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Printable>>
where
    I: Iterator<Item = &'a Token>,
{
    let position: Position;
    let token_preview = tokens.peek().ok_or(ParseError::UnexpectedEOF)?;
    let content = match token_preview.kind {
        TokenType::String(ref str) => {
            let token = tokens.next().expect("Token was peeked, now not found ??");
            position = token.position;
            Printable::String(str.clone())
        }
        _ => {
            let Node {
                content,
                position: pos,
            } = parse_expression(tokens)?;
            position = pos;
            Printable::ExpressionNode(Box::new(content))
        }
    };
    let node = Node { content, position };

    Ok(node)
}

pub fn parse_printables<'a, I>(tokens: &mut Peekable<I>) -> Result<Node<Printables>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut printables = Vec::new();
    loop {
        let printable: Node<Printable> = parse_one_printable(tokens)?;
        printables.push(printable);

        if let Some(Token {
            kind: TokenType::Comma,
            ..
        }) = tokens.peek()
        {
            tokens.next();
        } else {
            let mut position = printables[0].position;
            position.col_end = printables.last().unwrap().position.col_end;
            let node = Node {
                content: printables,
                position,
            };
            return Ok(node);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Result;
    use crate::tokenizer::tokenize;

    #[test]
    fn test_parse_printable() -> Result<()> {
        let printables = vec![r#""Hallo""#.to_string(), r#""2+3*4""#.to_string()];

        for printable in &printables {
            println!("{:#?}", printable);

            println!("* Tokenizing");
            let tokens = tokenize(&vec![printable.clone()])?;

            println!("* Parsing");
            let mut iter_token = tokens.iter().peekable();
            let result = super::parse_printables(&mut iter_token)?;
            println!("{:#?}", result);
        }
        Ok(())
    }

    #[test]
    fn test_parse_printables() -> Result<()> {
        let printables = vec![r#""Hallo", 2+3*4"#.to_string()];

        for printable in &printables {
            println!("{:#?}", printable);

            println!("* Tokenizing");
            let tokens = tokenize(&vec![printable.clone()])?;
            println!("tokens = {tokens:#?}");

            println!("* Parsing");
            let mut iter_token = tokens.iter().peekable();
            let result = super::parse_printables(&mut iter_token)?;
            println!("{:#?}", result);
        }
        Ok(())
    }
}
