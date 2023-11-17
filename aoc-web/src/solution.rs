use yew::prelude::*;

use aoc_solver::y2021;
use aoc_solver::y2022;
use aoc_solver::y2023;

#[derive(Properties, PartialEq)]
pub struct SolutionProps {
    pub day: u8,
    pub year: u32,
}

#[function_component]
pub fn Solution(props: &SolutionProps) -> Html {
    let output = match props.year {
        2021 => y2021::run_solution(props.day, None),
        2022 => y2022::run_solution(props.day, None),
        2023 => y2023::run_solution(props.day, None),
        _ => vec!["Unknown year".to_string()],
    };

    html! { <pre><code>{ output.join("\n") }</code></pre> }
}
