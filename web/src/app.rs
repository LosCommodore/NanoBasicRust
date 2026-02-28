use std::rc::Rc;
use anyhow::Result;
use leptos::prelude::*;
use nanobasic::interpreter::Interpreter;

// keep a few programs around; you can add more paths/names as needed
const FIB_BAS: &str = include_str!(r"../../../Examples/fib.bas");
const FACTORIAL_BAS: &str = include_str!(r"../../../Examples/factorial.bas");

/// run the interpreter on a blob of source text and return the output
fn run_nano(source: &str) -> Result<String> {
    let mut stream = Vec::<u8>::new();
    let mut nano_interpreter = Interpreter::from_str(source, &mut stream)?;
    nano_interpreter.run()?;
    let result = String::from_utf8(stream)?;
    Ok(result)
}

#[component]
pub fn App() -> impl IntoView {
    // state that holds the latest output
    let (output, set_output) = signal(String::new());
    let (active_program, set_active_program) = signal(String::new());

    // list of programs we can execute (name, source text)
    let programs: Rc<Vec<(&str, &str)>> = Rc::new(vec![
        ("Fibonacci", FIB_BAS),
        ("Factorial", FACTORIAL_BAS),
    ]);

    // prepare clones for the closures; Rc::clone is cheap and prevents moving the
    // original value out of scope.
    let programs_for_change = programs.clone();
  
    // index of the currently selected program
    let (selected_idx, set_selected_idx) = signal(0usize);

    view! {
        <div>
            <label for="programs">"Choose program:"</label>
            <select
                id="programs"
                on:change=move |ev| {
                    if let Ok(idx) = event_target_value(&ev).parse::<usize>() {
                        set_selected_idx.set(idx);
                    }
                    // update preview text (I assume you want the source, not name)
                    let src = programs_for_change[selected_idx.get()].1;
                    set_active_program.set(src.to_string());
                }
            >
            {programs_for_change.iter().enumerate().map(|(i,(name,_))|{
                    view! { <option value={i.to_string()} selected={i==selected_idx.get()}>{*name}</option> }
                }).collect_view()}
            </select>
        </div>

        // show the raw program text for the selected item
        <pre class="program-source"> { active_program } </pre>

        <button
            on:click=move |_| {
                let p = programs.clone();
                let code = p[selected_idx.get()].1;
                match run_nano(code) {
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
