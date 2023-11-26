mod cube;
mod lava;
mod rope;

use yew::prelude::*;
use yew_router::components::Link;

pub use self::cube::Cube;
pub use self::lava::Lava;
pub use self::rope::Rope;

use crate::year::Year;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub day: Option<u8>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header>
            <Year current={2022} />
            <nav class="links">
                {
                    for (1..=9).map(|day| {
                        let active = if Some(day) == props.day {
                            "active-link"
                        } else {
                            ""
                        };

                        html! {
                            <Link<Route>
                                classes={classes!(active)}
                                to={Route::Solution { year: 2022, day }}
                            >
                                { format!("[{day}]") }
                            </Link<Route>>
                        }
                    })
                }
                <Link<Route> classes={classes!("")} to={Route::Rope}>
                    { format!("[9+]") }
                </Link<Route>>
                {

                    for (10..=25).map(|day| {
                        let active = if Some(day) == props.day {
                            "active-link"
                        } else {
                            ""
                        };

                        html! {
                            <Link<Route>
                                classes={classes!(active)}
                                to={Route::Solution { year: 2022, day }}
                            >
                                { format!("[{day}]") }
                            </Link<Route>>
                        }
                    })
                }
                <Link<Route> classes={classes!("")} to={Route::Lava}>
                    { format!("[18+]") }
                </Link<Route>>
                <Link<Route> classes={classes!("")} to={Route::Cube}>
                    { format!("[22+]") }
                </Link<Route>>
            </nav>
        </header>
    }
}
