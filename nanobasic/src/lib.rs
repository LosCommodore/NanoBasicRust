use thiserror::Error;
use std::result;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown token '{unkown_code}' at line: {line_num:?}, starting at column: {col_start:?})")]
    UnkownToken {
        line_num: usize,
        col_start: usize,
        unkown_code: String,
    },
    
    #[error("Unexpected end of file")]
    UnexpectedEOF,

    #[error("Wrong Token, expected: {expected}, actual: {actual}")]
    WrongToken
        {
        expected: String,
        actual: String,
        },
}

pub type Result<T> = result::Result<T, ParseError>;

pub mod parser;
pub mod tokenizer;
