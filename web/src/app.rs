use std::rc::Rc;
use anyhow::Result;
use leptos::prelude::*;
use nanobasic::interpreter::Interpreter;

// include every BASIC file in the top‑level Examples directory; the
// tuple elements are (display name, source code).  When you add/remove files
// here, update the list accordingly (a build script could generate this, but
// for simplicity we write them out).
const PROGRAMS: &[(&str, &str)] = &[
    ("Factorial", include_str!(r"../../../Examples/factorial.bas")),
    ("Fibonacci", include_str!(r"../../../Examples/fib.bas")),
    ("GCD", include_str!(r"../../../Examples/gcd.bas")),
    ("Gosub", include_str!(r"../../../Examples/gosub.bas")),
    ("Goto", include_str!(r"../../../Examples/goto.bas")),
    ("If1", include_str!(r"../../../Examples/if1.bas")),
    ("If2", include_str!(r"../../../Examples/if2.bas")),
    ("Print1", include_str!(r"../../../Examples/print1.bas")),
    ("Print2", include_str!(r"../../../Examples/print2.bas")),
    ("Print3", include_str!(r"../../../Examples/print3.bas")),
    ("Variables", include_str!(r"../../../Examples/variables.bas")),
];

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
    let programs: Rc<Vec<(&str, &str)>> = Rc::new(PROGRAMS.iter().copied().collect());

    // pre-populate the active_program with the first entry
    if let Some((_, src)) = programs.get(0) {
        set_active_program.set(src.to_string());
    }

    // prepare clones for the closures; Rc::clone is cheap and prevents moving the
    // original value out of scope.  we need separate clones for each handler
    // because Rust moves captured variables into a `move` closure.
    let programs_for_change = programs.clone();
    let programs_for_click = programs.clone();
  
    // index of the currently selected program
    let (selected_idx, set_selected_idx) = signal(0usize);

    view! {
        <div class="px-8 pt-8 max-w-4xl h-screen flex flex-col overflow-hidden min-h-0">
            <h1 class="text-2xl font-bold mb-4">"Nanobasic Playground"</h1>
            <div class="flex items-center space-x-4 mb-4">
                <label for="programs">"Choose program:"</label>
                <select
                    id="programs"
                    class="w-64 border border-gray-300 rounded px-2 py-1 bg-white focus:outline-none focus:ring-2 focus:ring-blue-400"
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
                <button
                    class="bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded shadow"
                    on:click=move |_| {
                        // use pre-cloned Rc
                        let code = programs_for_click[selected_idx.get()].1;
                        match run_nano(code) {
                            Ok(txt) => set_output.set(txt),
                            Err(e)  => set_output.set(format!("error: {e:?}")),
                        }
                    }
                >
                    "Run interpreter"
                </button>
            </div>

        // program section with title
        <div class="mb-4 flex-1 flex flex-col min-h-0">
            <h2 class="text-xl font-semibold mb-2">"Program source"</h2>
            <pre class="program-source bg-blue-50 border border-blue-300 rounded p-4 text-sm overflow-auto flex-1">
                { active_program }
            </pre>
        </div>

        // output section with title
        <div class="mb-4 flex-1 flex flex-col min-h-0">
            <h2 class="text-xl font-semibold mb-2">"Program output"</h2>
            <pre class="nano-output bg-gray-50 border border-gray-300 rounded p-4 overflow-y-auto text-sm flex-1">
                {output}
            </pre>
        </div>
        </div>
    }
}
