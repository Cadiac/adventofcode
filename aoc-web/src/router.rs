use yew::prelude::*;
use yew_agent::oneshot::OneshotProvider;
use yew_router::prelude::*;

use crate::{agent::SolutionTask, header::Header, home::Home, solution::Solution, y2022};

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

pub fn switch(route: Route) -> Html {
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
            <main class="fade-in">
                <OneshotProvider<SolutionTask> path="/worker.js">
                    { main }
                </OneshotProvider<SolutionTask>>
            </main>
        </>
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
