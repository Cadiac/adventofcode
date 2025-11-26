use yew::prelude::*;
use yew_agent::oneshot::OneshotProvider;
use yew_router::prelude::*;

use crate::{
    header::Header, home::Home, runner::SolutionTask, solution::Solution,
    syntax::SyntaxHighlightTask, y2022, y2024,
};

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
    #[at("/2024/14/visual")]
    EasterEgg,
    #[at("/2024/15/warehouse")]
    WarehouseRobot,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    let year = match route {
        Route::Index | Route::NotFound => 2024,
        Route::Solution { year, day: _ } | Route::Home { year } => year,
        Route::Lava | Route::Rope | Route::Cube => 2022,
        Route::EasterEgg | Route::WarehouseRobot => 2024,
    };

    let main = match route {
        Route::Index => html! { <Home year={2025} /> },
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
        Route::EasterEgg => {
            html! { <y2024::EasterEgg/> }
        }
        Route::WarehouseRobot => {
            html! { <y2024::WarehouseRobot/> }
        }
        Route::NotFound => html! {<h1>{ "Not Found :(" }</h1>},
    };

    html! {
        <>
            <Header year={year} route={route} />
            <main class="fade-in">
                <OneshotProvider<SolutionTask> path="/solution-worker.js">
                    <OneshotProvider<SyntaxHighlightTask> path="/syntax-worker.js">
                        { main }
                    </OneshotProvider<SyntaxHighlightTask>>
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
