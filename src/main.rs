pub mod parser;
pub mod tokenizer;
use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::path::PathBuf;
use tokenizer::tokenize;

#[allow(dead_code)]
fn read_factorial() -> Result<Vec<String>> {
    let file: &str = r"../Examples/factorial.bas";

    let current_dir: PathBuf = env::current_dir()?;
    let absolute: PathBuf = current_dir.join(file);

    let absolute_str = absolute.to_string_lossy().to_string();
    println!("this is the path: {absolute_str}");

    let file = tokenizer::read_file(file).context("could not read facotrial.bas")?;
    Ok(file)
}

fn tokenize_and_parse(code: &[impl AsRef<str>]) -> Result<()> {
    let tokens = tokenize(code)?;

    println!("{:#?}", tokens);
    let mut iter_token = tokens.iter().peekable();

    let out = parser::parse(&mut iter_token)?;
    println!("{:#?}", out);

    let file = File::create("output.json").expect("failed to create file");
    serde_json::to_writer_pretty(&file, &out)?;
    Ok(())
}

fn main() -> Result<()> {
    // -- Read input
    let code = read_factorial()?;
    //let txt: Vec<String> = vec!["10 LET A = (2 + 3)*5 + B*-10".to_string()];

    // -- main
    tokenize_and_parse(&code)?;

    println!("Finished program");
    Ok(())
}
