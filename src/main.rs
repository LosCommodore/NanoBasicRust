use regex::Regex;
use std::error::Error;
use once_cell::sync::Lazy;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[allow(unused)]
#[derive(Debug)]
#[derive(PartialEq)]
enum TokenType {
    Comment,
    Whitespace,
    Print,
    If,
    Then,
    Let,
    Goto,
    Gosub,
    Return,
    Comma,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParen,
    CloseParen,
    Variable(String),
    Number(usize),
    String(String),
}

const IGNORE_TOKEN_TYPES: &[TokenType] = &[TokenType::Whitespace, TokenType::Comment];

#[allow(unused)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Token{
    kind: TokenType,
    line_num: usize,
    col_start: usize,
    col_end: usize,
}


#[allow(unused)]
struct Case {
    regex_str: String,
    regex: Regex,
    capture: bool,
    ctor: fn(&str) -> TokenType,
}

macro_rules! case {
    ($regex:expr, $capture:expr, $ctor:expr) => {
        Case {
            regex_str: $regex.to_string(),
            regex: Regex::new(concat!("^",$regex)).unwrap(),
            capture: $capture,
            ctor: $ctor,
        }
    };
}

static CASES: Lazy<[Case; 25]> = Lazy::new(|| [
    case!(r"(?i)rem.*", false, |_v| TokenType::Comment),
    case!(r"[ \t\n\r]", false, |_v| TokenType::Whitespace),
    case!(r"(?i)print", false, |_v| TokenType::Print),
    case!(r"(?i)if", false, |_v| TokenType::If),
    case!(r"(?i)then", false, |_v| TokenType::Then),
    case!(r"(?i)let", false, |_v| TokenType::Let),
    case!(r"(?i)goto", false, |_v| TokenType::Goto),
    case!(r"(?i)gosub", false, |_v| TokenType::Gosub),
    case!(r"(?i)return", false, |_v| TokenType::Return),
    case!(r",", false, |_v| TokenType::Comma),
    case!(r"=", false, |_v| TokenType::Equal),
    case!(r"<>|><", false, |_v| TokenType::NotEqual),
    case!(r"<=", false, |_v| TokenType::LessEqual),
    case!(r">=", false, |_v| TokenType::GreaterEqual),
    case!(r"<", false, |_v| TokenType::Less),
    case!(r">", false, |_v| TokenType::Greater),
    case!(r"\+", false, |_v| TokenType::Plus),
    case!(r"-", false, |_v| TokenType::Minus),
    case!(r"\*", false, |_v| TokenType::Multiply),
    case!(r"/", false, |_v| TokenType::Divide),
    case!(r"\(", false, |_v| TokenType::OpenParen),
    case!(r"\)", false, |_v| TokenType::CloseParen),
    case!(r"[A-Za-z_]+", true, |v| TokenType::Variable(v.to_string())),
    case!(r"-?[0-9]+", true, |v| TokenType::Number(v.parse().unwrap())),
    case!(r".*", true, |v| TokenType::String(v.to_string())),
]);


fn match_token(text: &str) -> Result<Token> {
    fn std_token(case: &Case, text: &str) -> Option<Token> {
        let m = case.regex.find(text)?;

        Some(Token{
            kind: (case.ctor)(text),
            line_num: 0,
            col_start: m.start(),
            col_end: m.end() -1
        })
    }

    
    let token = CASES
        .iter()
        .find_map(|case| std_token(case, text));
        
    let token = token.ok_or("Syntax Error".into());    
    token
}

fn match_line(line: &str) -> Result<Vec<Token>> {
    let mut parse_line = line;
    let mut tokens = Vec::<Token>::new();
    let mut col = 0;
    while parse_line.len() > col {
        parse_line = &line[col..];

        let mut token = match_token(parse_line)?;
        let offset = token.col_end - token.col_start+1;
      
        if !IGNORE_TOKEN_TYPES.contains(&token.kind) {
            token.col_start += col;
            token.col_end += col;
            tokens.push(token);
        }
        
        col+=offset;
    }
    Ok(tokens)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_token() {
        let params = [
            (r"rem hallo", Token{kind: TokenType::Comment, line_num:0, col_start:0, col_end:8}),
            (r"REM HaLLo", Token{kind: TokenType::Comment, line_num:0, col_start:0, col_end:8}),
            (r"goto", Token{kind: TokenType::Goto, line_num:0, col_start:0, col_end:3}),
            (r")", Token{kind: TokenType::CloseParen, line_num:0, col_start:0, col_end:0}),
            (r"ABC", Token{kind: TokenType::Variable("ABC".to_string()), line_num:0, col_start:0, col_end:2}),
            ];

        for (text, result) in &params {
            println!("Using regex: {}", *text);
            assert_eq!(match_token(*text).unwrap(), *result);
        }
    }
}


fn main() {
    println!("Starting program");
    let my_token = match_line("a = 12");
    println!("{:#?}", my_token);
}
