pub mod home;
pub mod solution;
pub mod y2021;
pub mod y2022;
pub mod y2023;
pub mod year;

use home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::solution::Solution;

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
    let header = match route {
        Route::Index | Route::Home { year: 2023 } => {
            html! { <y2023::Header/> }
        }
        Route::Home { year: 2022 } => {
            html! { <y2022::Header/> }
        }
        Route::Home { year: 2021 } => {
            html! { <y2021::Header/> }
        }
        Route::Solution { year: 2021, day } => {
            html! { <y2021::Header day={Some(day)}/> }
        }
        Route::Solution { year: 2022, day } => {
            html! { <y2022::Header day={Some(day)}/> }
        }
        Route::Solution { year: 2023, day } => {
            html! { <y2023::Header day={Some(day)}/> }
        }
        Route::Lava | Route::Rope | Route::Cube => {
            html! { <y2022::Header/> }
        }
        _ => html! { <y2023::Header/> },
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
            { header }
            { main }
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
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}
