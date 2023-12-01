use wasm_bindgen::prelude::*;
use yew::prelude::*;

use aoc_solver::solution::Solver;
use aoc_solver::y2020::Y2020;
use aoc_solver::y2021::Y2021;
use aoc_solver::y2022::Y2022;
use aoc_solver::y2023::Y2023;

#[wasm_bindgen(inline_js = "export function highlight() { hljs.highlightAll(); }")]
extern "C" {
    fn highlight();
}

#[derive(Properties, PartialEq)]
pub struct SolutionProps {
    pub day: u8,
    pub year: u32,
}

pub enum Msg {}

pub struct Solution;

impl Component for Solution {
    type Message = Msg;
    type Properties = SolutionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        highlight();
    }

    fn destroy(&mut self, _: &Context<Self>) {}

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

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
}
