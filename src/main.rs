use regex::Regex;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[allow(unused)]
#[derive(Debug)]
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

#[allow(unused)]
#[derive(Debug)]
struct Token{
    kind: TokenType,
    line_num: usize,
    col_start: usize,
    col_end: usize,
}

struct Case {
    regex: Regex,
    capture: bool,
    ctor: fn(&str) -> TokenType,
}

lazy_static::lazy_static! {
    static ref CASES: [Case; 2] = [
        Case {
            regex: Regex::new(r"rem.*").unwrap(),
            capture: false,
            ctor: |_v: &str|TokenType::Comment,
        },
        Case {
            regex: Regex::new(r"[A-Za-z_]+").unwrap(),
            capture: true,
            ctor: |v: &str|TokenType::Variable(v.to_string()),
        },
    ];
}


fn find_rem(line: &str) -> Result<Token> {
    fn std_token(case: &Case, text: &str) -> Option<Token> {
        let m = case.regex.find(text)?;
        Some(Token{
            kind: (case.ctor)(""),
            line_num: 0,
            col_start: m.start(),
            col_end: m.end()
        })
    }

    fn capture_token(case: &Case, text: &str) -> Option<Token> {
            let captures= case.regex.captures(text)?;
            let col_start = captures.get(0)?.start();
            let col_end = captures.get(0)?.end();
            
            let cap1 = captures.get(1)?;
            let cap1_str = cap1.as_str();
            Some(Token{
                kind: (case.ctor)(cap1_str),
                line_num: 0,
                col_start,
                col_end,  
            })
            }

    let token = CASES
        .iter()
        .find_map(|case| {
            match case.capture {
                true  => capture_token(case, line),
                false => std_token(case, line),
            }
        });
    
    let token = token.expect("Syntax Error");
    Ok(token)
}


fn main() {
    println!("Hallo welt");
    let my_token = find_rem("rem hallo welt");
    println!("{:#?}", my_token);
}
