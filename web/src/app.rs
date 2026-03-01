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

const MAX_EXE_LINES: usize = 10000;

/// run the interpreter on a blob of source text and return the output
fn run_nano(source: &str) -> Result<(String, String)> {
    let mut stream = Vec::<u8>::new();
    let mut interpreter = Interpreter::from_str(source, &mut stream)?;
    let mut count_lines = 0usize;

    while !interpreter.finished() & (count_lines < MAX_EXE_LINES) {
        interpreter.step_line()?;
        count_lines += 1;
    }
    let ast = interpreter.ast_json_pretty()?;
    let result = String::from_utf8(stream)?;
    Ok((result, ast))
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="flex items-center justify-between px-4 py-3 mb-2 bg-blue-900 text-white shadow-md">
            <h1 class="text-lg sm:text-xl font-bold tracking-tight">"Nanobasic"</h1>

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
fn SelectProgram(set_active_program: WriteSignal<String>) -> impl IntoView {
    let programs: Rc<Vec<(&str, &str)>> = Rc::new(PROGRAMS.iter().copied().collect());

    let (selected_idx, set_selected_idx) = signal(0usize);

    view! {
        <div class="flex items-center space-x-4">
            <label for="programs">"LOAD program:"</label>
            <select
                id="programs"
                class="w-full sm:w-64 border border-gray-300 rounded px-2 py-2 bg-white focus:ring-2 focus:ring-blue-400"
                on:change=move |ev| {
                    if let Ok(idx) = event_target_value(&ev).parse::<usize>() {
                        set_selected_idx.set(idx);
                    }
                    let src = programs[selected_idx.get()].1;
                    set_active_program.set(src.to_string());
                }
            >
                {programs
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
        </div>
    }
}

#[component]
fn ButtonRun(
    active_program: ReadSignal<String>,
    set_output: WriteSignal<String>,
    set_ast: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div>
            <button
                class="bg-blue-500 hover:bg-blue-600 text-white font-semibold p-2 rounded shadow w-full"
                on:click=move |_| {
                    let code = active_program.get();
                    match run_nano(&code) {
                        Ok((output, ast)) => {
                            set_output.set(output);
                            set_ast.set(ast);
                        }
                        Err(e) => set_output.set(format!("error: {e:?}")),
                    }
                }
            >
                "Run interpreter"
            </button>
        </div>
    }
}

#[component]
pub fn ProgramSource(
    active_program: ReadSignal<String>,
    set_active_program: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-[250px] sm:min-h-[400px] w-full">
            <h2 class="text-lg font-semibold mb-1">"Program source"</h2>
            <textarea
                class="w-full flex-1 min-h-0 resize-none border border-blue-300 rounded p-3 text-sm sm:text-base focus:ring-2 focus:ring-blue-400 outline-none"
                autocapitalize="off"
                // flex-1 und min-h-0 sind hier ESSENZIELL, damit die Textarea
                // innerhalb der 200px scrollt, statt das Div zu dehnen
                prop:value=active_program
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    set_active_program.set(val);
                }
            />
        </div>
    }
}

#[component]
pub fn DisplayAST(ast: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="w-full px-1">
            <details
                class="group border border-blue-200 rounded-lg bg-white overflow-hidden shadow-sm"
                open=false
            >
                <summary class="flex justify-between items-center p-3 cursor-pointer bg-blue-50 list-none font-bold text-sm text-blue-900 uppercase">
                    "Abstract Syntax Tree (AST)"
                    <span class="transition-transform group-open:rotate-180">"▼"</span>
                </summary>
                <pre class="p-4 font-mono overflow-auto max-h-[900px] bg-white border-t border-blue-100 whitespace-pre-wrap break-all">
                    {ast}
                </pre>
            </details>
        </div>
    }
}

#[component]
pub fn ProgramOutput(output: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col min-h-h-800 min-w-0 ">
            <h2 class="text-xl font-semibold mb-2">"Program output"</h2>
            <pre
                spellcheck="false"
                autocapitalize="off"
                class="bg-white w-full border border-blue-300 rounded p-4 overflow-y-auto flex-1"
            >
                {output}
            </pre>
        </div>
    }
}

#[component]
pub fn Hint(hint: String) -> impl IntoView {
    view! {
        <div
            class="w-full bg-yellow-50 border-yellow-400 p-2 mb-1 flex items-center shadow-sm"
            role="alert"
        >
            <div class="flex-shrink-0">
                <svg class="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
                    <path
                        fill-rule="evenodd"
                        d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                        clip-rule="evenodd"
                    />
                </svg>
            </div>
            <div class="ml-3">
                <p class="text-sm text-yellow-700">
                    <span class="font-bold">"Hint "</span>
                    {hint}
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // state: output of program
    let (output, set_output) = signal(String::new());
    let (ast, set_ast) = signal(String::new());

    // list of demo programs
    let programs: Rc<Vec<(&str, &str)>> = Rc::new(PROGRAMS.iter().copied().collect());
    let (active_program, set_active_program) = signal(String::new());

    // pre-populate the active_program with the first entry
    if let Some((_, src)) = programs.get(0) {
        set_active_program.set(src.to_string());
    }

    view! {
        <div class="px-4 pb-4 w-full min-h-screen flex flex-col gap-4 overflow-x-hidden">
            <Header />
            <SelectProgram set_active_program />
            <ProgramSource active_program set_active_program />
            <ButtonRun active_program set_output set_ast />

            <ProgramOutput output />
            <DisplayAST ast />
            <div class="flex flex-row">
                <Hint hint=format!(
                    "Cu rrently a maximum of {MAX_EXE_LINES} are executed to prevent freezing the browser in these cases.",
                ) />
            </div>

        </div>
    }
}
