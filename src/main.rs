pub mod parser;
pub mod tokenizer;
use anyhow::Result;
use std::env;
use std::fs::File;
use tokenizer::tokenize;

#[allow(dead_code)]
fn read_factorial() -> Result<Vec<String>> {
    let file =
        r"C:\Proj\cs_from_scatch\ComputerScienceFromScratch\NanoBASIC\Examples\factorial.bas";

    let file = tokenizer::read_file(file)?;
    Ok(file)
}

fn tokenize_and_parse(txt: &Vec<String>) -> Result<()> {
    let tokens = tokenize(&txt)?;

    println!("{:#?}", tokens);
    let mut iter_token = tokens.iter().peekable();

    let out = parser::Line::parse(&mut iter_token)?;
    println!("{:#?}", out);

    let file = File::create("output.json").expect("failed to create file");
    serde_json::to_writer_pretty(&file, &out)?;
    Ok(())
}

fn main() -> Result<()> {
    //color_backtrace::install();

    println!("Starting program");
    let env_name = "RUST_BACKTRACE".to_string();
    if let Ok(value) = env::var(env_name) {
        println!("env_name = {}", value);
    } else {
        println!("env_name not found!");
    }

    // -- Read input
    let txt: Vec<String> = vec!["10 LET A = (2 + 3)*5 + B*-10".to_string()];

    // -- main
    tokenize_and_parse(&txt)?;

    Ok(())
}
