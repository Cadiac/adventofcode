use yew::prelude::*;
use yew_router::components::Link;

use crate::Route;

const YEARS: &[u32] = &[2023, 2022, 2021, 2020];

#[derive(Properties, PartialEq)]
pub struct YearProps {
    pub current: u32,
}

#[function_component(Year)]
pub fn year(props: &YearProps) -> Html {
    html! {
        <div class="row">
            <h1>{"AoC"}</h1>
            <nav class="links">
                {
                    for YEARS.iter().map(|year| {
                        let active = if *year == props.current {
                            "active-link"
                        } else {
                            ""
                        };

                        html! {
                            <Link<Route> classes={classes!(active)} to={Route::Home { year: *year }}>{ year.to_string() }</Link<Route>>
                        }
                    })
                }
            </nav>
        </div>
    }
}
