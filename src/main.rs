pub mod tokenizer;
use tokenizer::{tokenize};
use std::hint::black_box;

fn main() {
    println!("Starting program");
    let file = r"C:\Proj\cs_from_scatch\ComputerScienceFromScratch\NanoBASIC\Examples\factorial.bas";
    let txt = tokenizer::read_file(file).expect("could not read file");
    let tokens = tokenize(&txt).expect("Failed to read");
    black_box(&tokens);
    println!("{:#?}", tokens);
}
