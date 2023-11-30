use crate::solution::{Solution, Solver};

pub mod day01;

pub const MAX_DAYS: u8 = 1;

pub struct Y2023;

impl Solver for Y2023 {
    fn run_solution(day: u8, input: Option<String>) -> Vec<String> {
        match day {
            1 => day01::Day01.run(input, 1, 2023),
            _ => vec![String::from("Solution not implemented (yet?)")],
        }
    }

    fn run_all() -> Vec<String> {
        let mut output = Vec::new();

        for day in 1..=MAX_DAYS {
            output.append(&mut Y2023::run_solution(day, None));
            output.push("--".to_string())
        }

        output
    }

    fn get_source(day: u8) -> &'static str {
        match day {
            1 => include_str!("./day01.rs"),
            _ => unimplemented!(),
        }
    }
}
