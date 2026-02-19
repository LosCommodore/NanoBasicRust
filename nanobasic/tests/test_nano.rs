use glob::glob;
use std::path::Path;
use anyhow::{Result};

const TEST_DIR: &str = "Examples";

fn tokenize_and_parse(file: impl AsRef<Path>) -> Result<()> {
    let tokens = nanobasic::tokenizer::read_file(file)?;
    //println!("{:#?}", tokens);

    let _lines = nanobasic::parser::parse(&tokens)?;
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

    for path_result  in pattern {
        let path = path_result.unwrap();
        print!("---- Executing: {path:#?}");
        tokenize_and_parse(&path).unwrap();
        println!(" ✅");
    }
}
