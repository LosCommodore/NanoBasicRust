use leptos::prelude::*;
use nanobasic::interpreter::Interpreter;
use nanobasic::parser;

fn run_nano() {
    let path = "../nanobasic/Examples/factorial.bas";
    let lines = parser::parse_file(&path).unwrap();
    let mut nano_interpreter = Interpreter::new(lines, None);
    nano_interpreter.run().unwrap();
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
            on:click= |_| run_nano()
        >
            Run interpreter
        </button>
    }
}