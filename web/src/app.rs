use leptos::prelude::*;
use nanobasic::interpreter::Interpreter;
use nanobasic::parser::parse_tokens;
use leptos::logging;
use nanobasic::tokenizer::tokenize;
use anyhow::Result;

const FACTORIAL_BAS: &str = include_str!(r"../../../Examples/fib.bas");



fn run_nano() -> Result<()> {
    let program = FACTORIAL_BAS
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let tokens = tokenize(&program)?;
    let ast = parse_tokens(&tokens)?;

    let mut stream = Vec::<u8>::new();    
    let mut nano_interpreter = Interpreter::new(ast, &mut stream);
    nano_interpreter.run()?;
    logging::log!("Executing Program\n hallo welt");

    let result: String = String::from_utf8(stream)?;

    logging::log!("{}", result);
    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| set_count.set(count.get() +2)
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>

        <button
            on:click= |_| {run_nano().unwrap();}
        >
            Run interpreter
        </button>
    }
}