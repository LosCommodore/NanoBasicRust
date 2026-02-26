use anyhow::Context;
use anyhow::Result;
use glob::glob;
use nanobasic::interpreter::Interpreter;
use nanobasic::parser;
use nanobasic::tokenizer::{Token, tokenize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const TEST_DIR: &str = "Examples";

fn tokenize_from_file(path: impl AsRef<Path>) -> Result<Vec<Token>> {
    let path = path.as_ref();
    log::info!(r#"Parsing tokens from "{path:#?}""#);

    let file = File::open(path).context(format!("Could not open file: {}", path.display()))?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let tokens = tokenize(&lines)?;
    Ok(tokens)
}

fn tokenize_and_parse(file: impl AsRef<Path>) -> Result<()> {
    let tokens = tokenize_from_file(file)?;
    //println!("{:#?}", tokens);

    let _lines = nanobasic::parser::parse_tokens(&tokens)?;
    //println!("{:#?}", lines);

    //let file = File::create("output.json").expect("failed to create file");
    //serde_json::to_writer_pretty(&file, &lines)?;
    Ok(())
}

#[test]
fn test_tokenize_and_parse_all_examples() {
    let mut p = TEST_DIR.to_string();
    p.push_str("/*.bas");

    let pattern = glob(&p).expect("invalid pattern");

    for path_result in pattern {
        let path = path_result.unwrap();
        print!("---- Executing: {path:#?}");
        tokenize_and_parse(&path).unwrap();
        println!(" ✅");
    }
}

#[test]
pub fn test_interpret_all_examples() {
    let mut p = TEST_DIR.to_string();
    p.push_str("/*.bas");

    let pattern = glob(&p).expect("invalid pattern");

    for path_result in pattern {
        let path = path_result.unwrap();
        println!("---- Executing: {path:#?}");
        let lines = parser::parse_file(&path).unwrap();
        let mut nano_interpreter = Interpreter::new(lines, None);
        nano_interpreter.run().unwrap();
        println!("✅ -------------------------------------------------------------");
        println!("")
    }
}