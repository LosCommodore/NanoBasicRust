use anyhow::Result;
use leptos::prelude::*;
use nanobasic::interpreter::Interpreter;

const FACTORIAL_BAS: &str = include_str!(r"../../../Examples/fib.bas");

/// run the interpreter and return the text it wrote to the provided stream
fn run_nano() -> Result<String> {
    let mut stream = Vec::<u8>::new();
    let mut nano_interpreter = Interpreter::from_str(FACTORIAL_BAS, &mut stream)?;
    nano_interpreter.run()?;
    let result = String::from_utf8(stream)?;
    Ok(result)
}

#[component]
pub fn App() -> impl IntoView {
    // state that holds the latest output
    let (output, set_output) = signal(String::new());

    view! {
        <button
            on:click=move |_| {
                // synchronous call is fine for a small program; if you prefer
                // non‑blocking, wrap this in `spawn_local`
                match run_nano() {
                    Ok(txt) => set_output.set(txt),
                    Err(e)  => set_output.set(format!("error: {e:?}")),
                }
            }
        >
            "Run interpreter"
        </button>

        // render the stored text, <pre> keeps the line breaks
        <pre class="nano-output">{output}</pre>
    }
}
