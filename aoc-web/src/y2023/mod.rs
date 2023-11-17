use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let navigator = use_navigator();

    html! {
        <header>
            <h1>{"AoC 2023"}</h1>
            <nav>
                <ul>
                    {
                        for (1..=1).map(|day| {
                            // TODO: why is navigator no available? is it a feature flag?
                            let onclick = if let Some(nav) = navigator.clone() {
                                Callback::from(move |_| nav.push(&Route::Solution { year: 2023, day }))
                            } else {
                                Callback::from(move |_| {})
                            };

                            html! {
                                <li><button onclick={onclick}>{format!("[{day}]")}</button></li>
                            }
                        })
                    }
                </ul>
            </nav>
        </header>
    }
}
