use crate::solution::Solution;

pub mod day19;

pub const MIN_DAYS: u8 = 19;
pub const MAX_DAYS: u8 = 19;

pub fn run_solution(day: u8, input: Option<String>) -> Vec<String> {
    match day {
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
