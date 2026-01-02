pub mod tokenizer;
use tokenizer::Token;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

use std::fs::{File};
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let out: Vec<String>  = reader
        .lines()
        .map(|l| l.unwrap())
        .collect();
    Ok(out)
}

fn execute_code(lines: &Vec<String>) -> Result<Vec<Token>> {
    let tokens = lines
    .iter()
    .map(|line| tokenizer::match_line(line)) 
    .collect::<Result<Vec<_>>>()?           
    .into_iter()
    .flatten()
    .collect();

    Ok(tokens)
}

fn main() {
    println!("Starting program");
    let file = r"C:\Proj\cs_from_scatch\ComputerScienceFromScratch\NanoBASIC\Examples\factorial.bas";
    let txt = read_file(file).expect("could not read file");
    let tokens = execute_code(&txt);

    println!("{:#?}", tokens);

    let my_token = tokenizer::match_line("a = 12");
    println!("{:#?}", my_token);
}
