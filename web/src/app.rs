use anyhow::Result;
use leptos::logging;
use leptos::prelude::*;
use nanobasic::interpreter::Interpreter;


const FACTORIAL_BAS: &str = include_str!(r"../../../Examples/fib.bas");

fn run_nano() -> Result<()> {
    let mut stream = Vec::<u8>::new();
    let mut nano_interpreter = Interpreter::from_str(FACTORIAL_BAS, &mut stream)?;
    
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
