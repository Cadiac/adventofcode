use crate::solution::{Solution, Solver};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

pub const MAX_DAYS: u8 = 12;

pub struct Y2023;

impl Solver for Y2023 {
    fn run_solution(day: u8, input: Option<String>) -> Vec<String> {
        match day {
            1 => day01::Day01.run(input, 1, 2023),
            2 => day02::Day02.run(input, 2, 2023),
            3 => day03::Day03.run(input, 3, 2023),
            4 => day04::Day04.run(input, 4, 2023),
            5 => day05::Day05.run(input, 5, 2023),
            6 => day06::Day06.run(input, 6, 2023),
            7 => day07::Day07.run(input, 7, 2023),
            8 => day08::Day08.run(input, 8, 2023),
            9 => day09::Day09.run(input, 9, 2023),
            10 => day10::Day10.run(input, 10, 2023),
            11 => day11::Day11.run(input, 11, 2023),
            12 => day12::Day12.run(input, 12, 2023),
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
            2 => include_str!("./day02.rs"),
            3 => include_str!("./day03.rs"),
            4 => include_str!("./day04.rs"),
            5 => include_str!("./day05.rs"),
            6 => include_str!("./day06.rs"),
            7 => include_str!("./day07.rs"),
            8 => include_str!("./day08.rs"),
            9 => include_str!("./day09.rs"),
            10 => include_str!("./day10.rs"),
            11 => include_str!("./day11.rs"),
            _ => unimplemented!(),
        }
    }
}
