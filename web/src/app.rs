use anyhow::Result;
use leptos::prelude::*;
use nanobasic::interpreter::Interpreter;
use std::rc::Rc;

// include every BASIC file in the top‑level Examples directory; the
// tuple elements are (display name, source code).  When you add/remove files
// here, update the list accordingly (a build script could generate this, but
// for simplicity we write them out).
const PROGRAMS: &[(&str, &str)] = &[
    (
        "Factorial",
        include_str!(r"../../nanobasic/Examples/factorial.bas"),
    ),
    (
        "Fibonacci",
        include_str!(r"../../nanobasic/Examples/fib.bas"),
    ),
    ("GCD", include_str!(r"../../nanobasic/Examples/gcd.bas")),
    ("Gosub", include_str!(r"../../nanobasic/Examples/gosub.bas")),
    ("Goto", include_str!(r"../../nanobasic/Examples/goto.bas")),
    ("If1", include_str!(r"../../nanobasic/Examples/if1.bas")),
    ("If2", include_str!(r"../../nanobasic/Examples/if2.bas")),
    (
        "Print1",
        include_str!(r"../../nanobasic/Examples/print1.bas"),
    ),
    (
        "Print2",
        include_str!(r"../../nanobasic/Examples/print2.bas"),
    ),
    (
        "Print3",
        include_str!(r"../../nanobasic/Examples/print3.bas"),
    ),
    (
        "Variables",
        include_str!(r"../../nanobasic/Examples/variables.bas"),
    ),
];

/// run the interpreter on a blob of source text and return the output
fn run_nano(source: &str) -> Result<String> {
    let mut stream = Vec::<u8>::new();
    let mut interpreter = Interpreter::from_str(source, &mut stream)?;

    let mut count_lines = 0usize;
    let maximum_lines = 10000usize;
    while !interpreter.finished() & (count_lines < maximum_lines) {
        interpreter.step_line()?;
        count_lines += 1;
    }

    let result = String::from_utf8(stream)?;
    Ok(result)
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="flex items-center justify-between px-6 py-4 mb-4 bg-blue-900 text-white shadow-md">
            <h1 class="text-xl font-bold tracking-tight">"Nanobasic Playground"</h1>

            <a
                href="https://github.com/LosCommodore/NanoBasicRust"
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center gap-2 hover:text-gray-400 transition-colors duration-200"
                aria-label="GitHub Repository"
            >
                <span class="hidden sm:inline text-sm font-medium">"Source Code"</span>
                <svg
                    height="28"
                    width="28"
                    viewBox="0 0 16 16"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
                </svg>
            </a>
        </header>
    }
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

    // index of the currently selected program
    let (selected_idx, set_selected_idx) = signal(0usize);

    view! {
        <div class="px-8 w-full h-screen flex flex-col overflow-hidden min-h-0 min-w-0">
            <Header />
            <div class="flex items-center space-x-4 mb-4">
                <label for="programs">"Choose program:"</label>
                <select
                    id="programs"
                    class="w-64 border border-gray-300 rounded px-2 py-1 bg-white focus:outline-none focus:ring-2 focus:ring-blue-400"
                    on:change=move |ev| {
                        if let Ok(idx) = event_target_value(&ev).parse::<usize>() {
                            set_selected_idx.set(idx);
                        }
                        let src = programs_for_change[selected_idx.get()].1;
                        set_active_program.set(src.to_string());
                    }
                >
                    {programs_for_change
                        .iter()
                        .enumerate()
                        .map(|(i, (name, _))| {
                            view! {
                                <option value=i.to_string() selected=i == selected_idx.get()>
                                    {*name}
                                </option>
                            }
                        })
                        .collect_view()}
                </select>
                <button
                    class="bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded shadow"
                    on:click=move |_| {
                        let code = active_program.get();
                        match run_nano(&code) {
                            Ok(txt) => set_output.set(txt),
                            Err(e) => set_output.set(format!("error: {e:?}")),
                        }
                    }
                >
                    "Run interpreter"
                </button>
            </div>
            <div class="mb-4 flex-1 flex flex-col min-h-0 min-w-0">
                <h2 class="text-xl font-semibold mb-2">"Program source"</h2>
                <textarea
                    class="w-full bg-blue-50 border border-blue-300 rounded p-4 text-sm flex-1 resize-none focus:outline-none"
                    prop:value=active_program
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        set_active_program.set(val);
                    }
                />
            </div>

            <div class="mb-4 flex-1 flex flex-col min-h-0 min-w-0">
                <h2 class="text-xl font-semibold mb-2">"Program output"</h2>
                <pre class="nano-output w-full bg-gray-50 border border-gray-300 rounded p-4 overflow-y-auto text-sm flex-1">
                    {output}
                </pre>
            </div>
        </div>
    }
}
