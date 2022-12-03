use yew::prelude::*;

use aoc::solution::{run_all, run_solution};

#[function_component]
fn App() -> Html {
    let console = use_state(|| vec![]);
    let run_all = {
        let console = console.clone();
        move |_| {
            let output = run_all().unwrap();
            console.set(output);
        }
    };

    let run_day_1 = {
        let console = console.clone();
        move |_| {
            let output = run_solution(aoc::solution::Day::Day01, None).unwrap();
            console.set(output);
        }
    };

    let run_day_2 = {
        let console = console.clone();
        move |_| {
            let output = run_solution(aoc::solution::Day::Day02, None).unwrap();
            console.set(output);
        }
    };

    let run_day_3 = {
        let console = console.clone();
        move |_| {
            let output = run_solution(aoc::solution::Day::Day03, None).unwrap();
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
                        <li><button onclick={run_day_1}>{ "[1]" }</button></li>
                        <li><button onclick={run_day_2}>{ "[2]" }</button></li>
                        <li><button onclick={run_day_3}>{ "[3]" }</button></li>
                    </ul>
                </nav>
            </header>
            <main>
                <pre>{ console.join("\n") }</pre>
            </main>
        </>
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}