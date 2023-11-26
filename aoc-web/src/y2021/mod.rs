use yew::prelude::*;
use yew_router::components::Link;

use crate::{year::Year, Route};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub day: Option<u8>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header>
            <Year current={2021} />
            <nav class="links">
                {
                    for (19..=20).map(|day| {
                        let active = if Some(day) == props.day {
                            "active-link"
                        } else {
                            ""
                        };

                        html! {
                            <Link<Route>
                                classes={classes!(active)}
                                to={Route::Solution { year: 2021, day }}
                            >
                                { format!("[{day}]") }
                            </Link<Route>>
                        }
                    })
                }
            </nav>
        </header>
    }
}
