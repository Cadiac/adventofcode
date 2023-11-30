pub mod header;
pub mod home;
pub mod navlink;
pub mod solution;
pub mod y2020;
pub mod y2021;
pub mod y2022;
pub mod y2023;
pub mod year;

use home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{header::Header, navlink::NavLink, solution::Solution};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/:year")]
    Home { year: u32 },
    #[at("/:year/:day")]
    Solution { year: u32, day: u8 },
    #[at("/2022/9/rope")]
    Rope,
    #[at("/2022/18/lava")]
    Lava,
    #[at("/2022/22/cube")]
    Cube,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn router(route: Route) -> Html {
    let year = match route {
        Route::Index | Route::NotFound => 2023,
        Route::Solution { year, day: _ } | Route::Home { year } => year,
        Route::Lava | Route::Rope | Route::Cube => 2022,
    };

    let main = match route {
        Route::Index => html! { <Home year={2023} /> },
        Route::Home { year } => html! { <Home year={year} /> },
        Route::Solution { year, day } => {
            html! { <Solution year={year} day={day} />}
        }
        Route::Rope => {
            html! { <y2022::Rope/> }
        }
        Route::Lava => {
            html! { <y2022::Lava/> }
        }
        Route::Cube => {
            html! { <y2022::Cube/> }
        }
        Route::NotFound => html! {<h1>{ "Not Found :(" }</h1>},
    };

    html! {
        <>
            <Header year={year} route={route} />
            <main>{ main }</main>
        </>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer>
            <small>
                {"Made by "}
                <a href="https://github.com/Cadiac">{"Cadiac"}</a>
                {". Source code can be be found "}
                <a href="https://github.com/Cadiac/adventofcode">{"here"}</a>
                {"."}
            </small>
        </footer>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={router} />
            </BrowserRouter>
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
