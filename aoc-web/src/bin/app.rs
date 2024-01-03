use yew::prelude::*;

use aoc_web::{footer::Footer, router::Router};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Router/>
            <Footer />
        </>
    }
}

fn main() {
    let document = gloo::utils::document();
    let container = document.query_selector("#aoc").unwrap().unwrap();

    yew::Renderer::<App>::with_root(container).render();
    wasm_logger::init(wasm_logger::Config::default());
}
