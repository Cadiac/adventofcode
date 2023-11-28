use crate::solution::Solution;

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

pub fn run_solution(day: u8, input: Option<String>) -> Vec<String> {
    match day {
        1 => day01::Day01.run(input),
        2 => day02::Day02.run(input),
        3 => day03::Day03.run(input),
        4 => day04::Day04.run(input),
        5 => day05::Day05.run(input),
        6 => day06::Day06.run(input),
        7 => day07::Day07.run(input),
        8 => day08::Day08.run(input),
        9 => day09::Day09.run(input),
        10 => day10::Day10.run(input),
        11 => day11::Day11.run(input),
        12 => day12::Day12.run(input),
        _ => vec![String::from("Solution not implemented (yet?)")],
    }
}

pub fn run_all() -> Vec<String> {
    let mut output = Vec::new();

    for day in 1..=25 {
        output.append(&mut run_solution(day, None));
        output.push("--".to_string())
    }

    output
}
