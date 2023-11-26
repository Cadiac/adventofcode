use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub year: u32,
}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
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
        format!("            AoC {}           ", props.year).as_str(),
    ].join("\n") }</code></pre> }
}
