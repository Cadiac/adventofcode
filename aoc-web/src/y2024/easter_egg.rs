use std::collections::HashSet;

use yew::prelude::*;

use aoc_solver::solution::Solution;
use aoc_solver::y2024::day14::{parse, predict, Day14, Robot};

pub struct EasterEgg {
    seconds: i64,
    width: u64,
    height: u64,
    robots: Vec<Robot>,
    predicted: HashSet<(i64, i64)>,
}

pub enum Msg {
    TimeChanged(i64),
}

impl Component for EasterEgg {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let robots = parse(Day14.default_input()).unwrap();
        let width = 101;
        let height = 103;
        let seconds = 7892;

        let predicted: HashSet<(i64, i64)> = robots
            .iter()
            .map(|robot| predict(robot, width, height, seconds))
            .collect();

        Self {
            seconds,
            width,
            height,
            robots,
            predicted,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _: &Context<Self>) {}

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TimeChanged(new_time) => {
                self.seconds = new_time;
                self.predicted = self
                    .robots
                    .iter()
                    .map(|robot| predict(robot, self.width, self.height, self.seconds))
                    .collect();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let output = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| {
                        if self.predicted.contains(&(x as i64, y as i64)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        html! {
            <>
                <h2>{"-- Day 14 --"}</h2>

                <input
                    class="full-width"
                    type="range"
                    min="0"
                    max="10403"
                    value={self.seconds.to_string()}
                    step="1"
                    oninput={ctx.link().callback(|event: InputEvent| {
                        let input = event.target_unchecked_into::<web_sys::HtmlInputElement>();
                        let seconds = input.value().parse::<i64>().unwrap_or_default();
                        Msg::TimeChanged(seconds)
                    })}
                />
                <p class="success">
                    {format!("t={}", self.seconds)}
                </p>
                <pre class="small">
                    <code>{ output }</code>
                </pre>
            </>
        }
    }
}
