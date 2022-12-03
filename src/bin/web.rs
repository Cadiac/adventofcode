use yew::prelude::*;

use aoc::solution::{run_all, run_solution};

#[function_component]
fn App() -> Html {
    let console = use_state(|| vec![
        "             *            ".to_string(),
        "             ^            ".to_string(),
        "            ^^o           ".to_string(),
        "            o^^           ".to_string(),
        "            ^^o^          ".to_string(),
        "           o^^^^o         ".to_string(),
        "           ^^o^^^^        ".to_string(),
        "       ______||______     ".to_string(),
        "          AoC 2022        ".to_string(),
    ]);
    let run_all = {
        let console = console.clone();
        move |_| {
            let output = run_all().unwrap();
            console.set(output);
        }
    };

    let run_day = |day: aoc::solution::Day| {
        let console = console.clone();
        move |_| {
            let output = run_solution(day.clone(), None).unwrap();
            console.set(output);
        }
    };

    html! {
        <>
            <header>
                <h1>{"AoC 2022"}</h1>
                <nav>
                    <ul>
                        <li><button onclick={run_all}>{ "[Run All]" }</button></li>
                        {
                            for (1..=3).map(|day| {
                                html! {
                                    <li><button onclick={run_day(day.into())}>{format!("[{}]", day)}</button></li>
                                }
                            })
                        }
                    </ul>
                </nav>
            </header>
            <main>
                <pre>{ console.join("\n") }</pre>
            </main>
            <footer>
                <small>
                    {"Made by "}
                    <a href="https://github.com/Cadiac">{"Cadiac"}</a>
                    {". Source code can be be found "}
                    <a href="https://github.com/Cadiac/adventofcode">{"here"}</a>
                    {"."}
                </small>
            </footer>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}
