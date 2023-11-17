use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! { <pre><code>{ vec![
        "                               ",
        "               *               ",
        "               ^^              ",
        "              ^^o              ",
        "              o^^              ",
        "              ^^o^             ",
        "             o^^^^o            ",
        "             ^^o^^^^           ",
        "        _______||_______       ",
        "            AoC 2023           ",
    ].join("\n") }</code></pre> }
}
