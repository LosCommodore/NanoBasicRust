pub mod parser;
pub mod tokenizer;
use tokenizer::tokenize;
use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
use std::fs::File;


#[allow(dead_code)]
fn read_factorial() -> Result<Vec<String>> {
    let file =
        r"C:\Proj\cs_from_scatch\ComputerScienceFromScratch\NanoBASIC\Examples\factorial.bas";

    tokenizer::read_file(file)
}

fn tokenize_and_parse(txt: &Vec<String>) -> Result<()> {
   
    let tokens = tokenize(&txt)?;
    
    println!("{:#?}", tokens);
    let mut iter_token = tokens.iter().peekable();

    let out = parser::parse_line(&mut iter_token)?;
    println!("{:#?}", out);
    
    let file = File::create("output.json").expect("failed to create file");
    serde_json::to_writer_pretty(&file,&out)?;
    Ok(())
}

fn main() {
    println!("Starting program");

    // -- Read input
    let txt: Vec<String> = vec!["10 LET A = (2 + 3)*5 + B*-10".to_string()];
    
    // -- main
    let result = tokenize_and_parse(&txt);

    // -- Error handling
    if let Err(error) = result {
        println!("{:?}", error);
        std::process::exit(1);
    }
    
}
