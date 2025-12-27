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
    Variable,
    Number,
    String,
}

#[allow(unused)]
#[derive(Debug)]
struct Token{
    kind: TokenType,
    line_num: usize,
    col_start: usize,
    col_end: usize,
}


/*

fn find_token(text: &str) -> Token {
    fn rem_token(txt: &str) -> Token {
        Token(kind=rem_token(txt))

    }

    fn hallo(x: &str) -> &str  { x}

    let reg_rem = Regex::new(r"rem.*").unwrap();

    let ac = vec![("dbc", hallo)];
    let ac2: Vec<(Regex, fn(&str) -> Token)>;
    ac2.push((reg_rem), rem_token )


    match text {
        _ if reg_rem.is_match(text) => Token::Comment,
        _ => Token::Divide
    }
}
*/

fn find_rem(line: &str) -> Result<Option<Token>> {
    let reg_rem = Regex::new(r"rem.*").unwrap();
    let m = reg_rem.find(line);

    let Some(mm) = m else {return Ok(None) };

    println!("{:?}",mm);
    Ok(Some(
        Token{
            kind:TokenType::Comment, 
             line_num:1,
              col_start:2,
               col_end:3}))
}

fn main() {
    println!("Hallo welt");
    let my_token = find_rem("rem hallo welt");
    println!("{:#?}", my_token);
}
