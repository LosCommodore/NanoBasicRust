pub mod parser;
pub mod tokenizer;
use tokenizer::tokenize;
use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
use parser::expressions::parse_expression;
use std::fs::File;

#[allow(dead_code)]
fn read_factorial() -> Result<Vec<String>> {
    let file =
        r"C:\Proj\cs_from_scatch\ComputerScienceFromScratch\NanoBASIC\Examples\factorial.bas";

    tokenizer::read_file(file)
}

fn main() {
    println!("Starting program");
    //read_factorial()
    let txt = vec!["2 + 3".to_string()];
    let tokens = tokenize(&txt).expect("Failed to read");
    
    println!("{:#?}", tokens);
    let mut iter_token = tokens.iter().peekable();
    let out = parse_expression(&mut iter_token).expect("parsing sucessful");
    println!("{:#?}", out);
    
    let file = File::create("output.json").expect("failed to create file");
    serde_json::to_writer_pretty(&file,&out).unwrap();
}
