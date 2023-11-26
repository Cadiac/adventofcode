use crate::solution::Solution;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day19;

pub const MIN_DAYS: u8 = 19;
pub const MAX_DAYS: u8 = 19;

pub fn run_solution(day: u8, input: Option<String>) -> Vec<String> {
    match day {
        1 => day01::Day01.run(input),
        2 => day02::Day02.run(input),
        3 => day03::Day03.run(input),
        19 => day19::Day19.run(input),
        _ => vec![String::from("Solution not implemented (yet?)")],
    }
}

pub fn run_all() -> Vec<String> {
    let mut output = Vec::new();

    for day in MIN_DAYS..=MAX_DAYS {
        output.append(&mut run_solution(day, None));
        output.push("--".to_string())
    }

    output
}
