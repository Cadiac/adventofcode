use yew::prelude::*;

use aoc_solver::solution::Solver;
use aoc_solver::y2020::Y2020;
use aoc_solver::y2021::Y2021;
use aoc_solver::y2022::Y2022;
use aoc_solver::y2023::Y2023;

#[derive(Properties, PartialEq)]
pub struct SolutionProps {
    pub day: u8,
    pub year: u32,
}

#[function_component]
pub fn Solution(props: &SolutionProps) -> Html {
    let (output, source) = match props.year {
        2020 => (
            Y2020::run_solution(props.day, None),
            Y2020::get_source(props.day),
        ),
        2021 => (
            Y2021::run_solution(props.day, None),
            Y2021::get_source(props.day),
        ),
        2022 => (
            Y2022::run_solution(props.day, None),
            Y2022::get_source(props.day),
        ),
        2023 => (
            Y2023::run_solution(props.day, None),
            Y2023::get_source(props.day),
        ),
        _ => (vec!["Unknown year".to_string()], ""),
    };

    html! {
        <div>
            <pre>
                <code>{ output.join("\n") }</code>
            </pre>
            <pre>
                <code class="source">{ source }</code>
            </pre>
        </div>
    }
}
