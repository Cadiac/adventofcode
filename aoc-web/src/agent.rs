use chrono::Local;
use yew_agent::prelude::*;

use aoc_solver::{solution::Solver, y2020::Y2020, y2021::Y2021, y2022::Y2022, y2023::Y2023};

#[oneshot]
pub async fn SolutionTask(input: (u32, u8)) -> (String, i64) {
    let (year, day) = input;

    let start = Local::now();

    let output = match year {
        2020 => Y2020::run_solution(day, None),
        2021 => Y2021::run_solution(day, None),
        2022 => Y2022::run_solution(day, None),
        2023 => Y2023::run_solution(day, None),
        _ => vec!["Missing year".to_string()],
    };

    let duration = (Local::now() - start).num_milliseconds();

    (output.join("\n"), duration)
}
