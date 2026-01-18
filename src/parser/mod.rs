use crate::tokenizer::{Token, TokenType};
use std::error::Error;
use std::iter::Peekable;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[allow(unused)]
#[derive(Debug, PartialEq)]
struct Statement {
    node: Node,
    line_id: usize,
    kind: StatementEnum,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum StatementEnum {
    Print,
    Return,
    If(Box<IfStatement>),
    GoSub,
    GoTo,
    Let(Box<LetStatement>),
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
struct Node {
    line_num: usize,
    col_start: usize,
    col_end: usize,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
struct BooleanExpression {
    operator: TokenType,
    left_expr: NumericExpression,
    right_expr: NumericExpression,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
struct NumericExpression {
    node: Node,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
struct IfStatement {
    boolean_expr: BooleanExpression,
    then_statement: Statement,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
struct LetStatement {
    name: String,
    expr: NumericExpression,
}

fn parse_let_statement<'a, I>(tokens: &mut Peekable<I>) -> Result<StatementEnum>
where
    I: Iterator<Item = &'a Token>,
{
    let var = tokens
        .next()
        .ok_or("Syntax error: unexpected end of line")?;

    println!("{:?}",&var);
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

    let let_statement: LetStatement = LetStatement {
        name: var_name.clone(),
        expr,
    };
    let statement_enum = StatementEnum::Let(Box::new(let_statement));
    Ok(statement_enum)
}

fn parse_if_statement<'a, I>(_tokens: &mut Peekable<I>) -> Result<StatementEnum>
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
    let if_statement: IfStatement = IfStatement {
        boolean_expr,
        then_statement,
    };

    Ok(StatementEnum::If(Box::new(if_statement)))
}

fn parse_line<'a, I>(tokens: &mut Peekable<I>) -> Result<Statement>
where
    I: Iterator<Item = &'a Token>,
{
    let line_token = tokens.next().expect("Token not found");

    let TokenType::Number(line_id) = line_token.kind else {
        return Err("Parse error".into());
    };

    println!("line line_id: {:?}", line_id);
    println!("line token: {:?}", line_token);

    let token = tokens.next().expect("Token not found");

    let kind: StatementEnum = match token.kind {
        TokenType::Print => StatementEnum::Print,
        TokenType::If => parse_if_statement(tokens)?,
        TokenType::Let => parse_let_statement(tokens)?,
        TokenType::Goto => StatementEnum::GoTo,
        TokenType::Gosub => StatementEnum::GoSub,
        TokenType::Return => StatementEnum::Return,
        _ => return Err("error".into()),
    };

    let node = Node {
        line_num: 1,
        col_start: 1,
        col_end: 1,
    };
    let statement = Statement {
        node,
        line_id,
        kind,
    };
    Ok(statement)
}

pub fn parse(tokens: &Vec<Token>) {
    let mut iter_token = tokens.iter().peekable();

    while let Some(_t) = iter_token.peek() {
        parse_line(&mut iter_token).expect("error while parsing line");
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_line};
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
    fn test_line_let_statement() {
        let tokens = vec![
            dummy_token(TokenType::Number(10)),
            dummy_token(TokenType::Let),
            dummy_token(TokenType::Variable("ABC".to_string())),
            dummy_token(TokenType::Equal),
            dummy_token(TokenType::Number(42)),
        ];

        let mut iter_tokens = tokens.iter().peekable();
        let result = parse_line(&mut iter_tokens);
        println!("{:#?}", &result)
    }
}
