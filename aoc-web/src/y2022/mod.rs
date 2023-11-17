mod cube;
mod lava;
mod rope;

pub use self::cube::Cube;
pub use self::lava::Lava;
pub use self::rope::Rope;

use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let navigator = use_navigator().unwrap();

    let day_9_extra = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Rope))
    };

    let day_18_extra = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Lava))
    };

    let day_22_extra = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Cube))
    };

    html! {
        <header>
            <h1>{"AoC 2022"}</h1>
            <nav>
                <ul>
                    {
                        for (1..=9).map(|day| {
                            let navigator = navigator.clone();
                            let onclick = Callback::from(move |_| navigator.push(&Route::Solution { year: 2022, day }));

                            html! {
                                <li><button onclick={onclick}>{format!("[{day}]")}</button></li>
                            }
                        })
                    }
                    <li><button onclick={day_9_extra}>{ "[9+]" }</button></li>
                    {
                        for (10..=25).map(|day| {
                            let navigator = navigator.clone();
                            let onclick = Callback::from(move |_| navigator.push(&Route::Solution { year: 2022, day }));

                            html! {
                                <li><button onclick={onclick}>{format!("[{day}]")}</button></li>
                            }
                        })
                    }
                    <li><button onclick={day_18_extra}>{ "[18+]" }</button></li>
                    <li><button onclick={day_22_extra}>{ "[22+]" }</button></li>
                </ul>
            </nav>
        </header>
    }
}
