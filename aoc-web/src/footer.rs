use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
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
