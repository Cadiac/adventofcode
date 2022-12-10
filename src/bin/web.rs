use yew::prelude::*;

use aoc::solution::{run_all, run_solution, MAX_DAYS};

use aoc::web::{rope::Rope};

#[function_component]
fn App() -> Html {
    let console = use_state(|| vec![
        "                               ".to_string(),
        "               *               ".to_string(),
        "               ^^              ".to_string(),
        "              ^^o              ".to_string(),
        "              o^^              ".to_string(),
        "              ^^o^             ".to_string(),
        "             o^^^^o            ".to_string(),
        "             ^^o^^^^           ".to_string(),
        "        _______||_______       ".to_string(),
        "            AoC 2022           ".to_string(),
    ]);

    let view_rope = use_state(|| false);

    let toggle_rope = {
        let view_rope = view_rope.clone();
        move |_| {
            view_rope.set(!(*view_rope));
        }
    };

    let run_all = {
        let console = console.clone();
        let view_rope = view_rope.clone();
        move |_| {
            view_rope.set(false);
            let output = run_all();
            console.set(output);
        }
    };

    let run_day = |day: u8| {
        let console = console.clone();
        let view_rope = view_rope.clone();
        move |_| {
            view_rope.set(false);
            let output = run_solution(day, None);
            console.set(output);
        }
    };

    html! {
        <>
            <header>
                <h1>{"AoC 2022"}</h1>
                <nav>
                    <ul>
                        <li><button onclick={run_all}>{ "[All]" }</button></li>
                        {
                            for (1..=9).map(|day| {
                                html! {
                                    <li><button onclick={run_day(day)}>{format!("[{}]", day)}</button></li>
                                }
                            })
                        }
                        <li><button onclick={toggle_rope}>{ "[9+]" }</button></li>
                        {
                            for (9..=MAX_DAYS).map(|day| {
                                html! {
                                    <li><button onclick={run_day(day)}>{format!("[{}]", day)}</button></li>
                                }
                            })
                        }
                    </ul>
                </nav>
            </header>
            <main>
                {if *view_rope {
                    html! { <Rope/> }
                } else {
                    html! { <pre><code>{ console.join("\n") }</code></pre> }
                }}
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
