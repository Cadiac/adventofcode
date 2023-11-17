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
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub const MAX_DAYS: u8 = 25;

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
        13 => day13::Day13.run(input),
        14 => day14::Day14.run(input),
        15 => day15::Day15.run(input),
        16 => day16::Day16.run(input),
        17 => day17::Day17.run(input),
        18 => day18::Day18.run(input),
        19 => day19::Day19.run(input),
        20 => day20::Day20.run(input),
        21 => day21::Day21.run(input),
        22 => day22::Day22.run(input),
        23 => day23::Day23.run(input),
        24 => day24::Day24.run(input),
        25 => day25::Day25.run(input),
        _ => vec![String::from("Solution not implemented (yet?)")],
    }
}

pub fn run_all() -> Vec<String> {
    let mut output = Vec::new();

    for day in 1..=MAX_DAYS {
        output.append(&mut run_solution(day, None));
        output.push("--".to_string())
    }

    output
}
