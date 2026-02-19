use anyhow::Result;
use env_logger::{Builder, Target, WriteStyle};
use log::LevelFilter;
use nanobasic;
use std::fs::File;
use std::path::Path;

fn tokenize_and_parse(file: impl AsRef<Path>) -> Result<()> {
    let tokens = nanobasic::tokenizer::read_file(file)?;
    println!("{:#?}", tokens);

    let lines = nanobasic::parser::parse(&tokens)?;
    println!("{:#?}", lines);

    let file = File::create("output.json").expect("failed to create file");
    serde_json::to_writer_pretty(&file, &lines)?;
    Ok(())
}

fn run_app() -> Result<()> {
    tokenize_and_parse("../Examples/factorial.bas")
}

fn main() -> Result<()> {
    Builder::new()
        .filter_level(LevelFilter::Info) // Setzt das Basis-Level auf Info
        .target(Target::Stdout)
        .write_style(WriteStyle::Always) // Schreibt in stdout statt stderr
        .init();

    log_panics::init(); // Ab jetzt landen Panics im Log

    log::info!("Starting progam");

    let result = run_app();
    if let Err(e) = result {
        log::error!("Progam aborted due to error: {e:?}");
        return Err(e);
    };

    log::info!("Finished program");
    Ok(())
}
