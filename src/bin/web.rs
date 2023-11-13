use yew::prelude::*;

use aoc::y2022::{run_solution, MAX_DAYS};

use aoc::web::{cube::Cube, lava::Lava, rope::Rope};

enum Scene {
    Rope,
    Lava,
    Cube,
    Day,
}

#[function_component]
fn App() -> Html {
    let console = use_state(|| {
        vec![
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
        ]
    });

    let scene = use_state(|| Scene::Day);

    let activate_rope = {
        let scene = scene.clone();
        move |_| {
            scene.set(Scene::Rope);
        }
    };

    let activate_lava = {
        let scene = scene.clone();
        move |_| {
            scene.set(Scene::Lava);
        }
    };

    let activate_cube = {
        let scene = scene.clone();
        move |_| {
            scene.set(Scene::Cube);
        }
    };

    // TODO: Disabled for now, make this run all of the solutions
    // simultaneously using web workers or something
    // let run_all = {
    //     let console = console.clone();
    //     let view_rope = view_rope.clone();
    //     move |_| {
    //         view_rope.set(false);
    //         let output = run_all();
    //         console.set(output);
    //     }
    // };

    let run_day = |day: u8| {
        let console = console.clone();
        let scene = scene.clone();
        move |_| {
            scene.set(Scene::Day);
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
                        // <li><button onclick={run_all}>{ "[All]" }</button></li>
                        {
                            for (1..=9).map(|day| {
                                html! {
                                    <li><button onclick={run_day(day)}>{format!("[{}]", day)}</button></li>
                                }
                            })
                        }
                        <li><button onclick={activate_rope}>{ "[9+]" }</button></li>
                        {
                            for (10..=MAX_DAYS).map(|day| {
                                html! {
                                    <li><button onclick={run_day(day)}>{format!("[{}]", day)}</button></li>
                                }
                            })
                        }
                        <li><button onclick={activate_lava}>{ "[18+]" }</button></li>
                        <li><button onclick={activate_cube}>{ "[22+]" }</button></li>
                    </ul>
                </nav>
            </header>
            <main>
                {match *scene {
                    Scene::Day => {
                        html! { <pre><code>{ console.join("\n") }</code></pre> }
                    },
                    Scene::Rope => {
                        html! { <Rope/> }
                    },
                    Scene::Lava => {
                        html! { <Lava/> }
                    }
                    Scene::Cube => {
                        html! { <Cube/> }
                    }
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
