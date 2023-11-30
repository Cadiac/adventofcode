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

pub struct Y2022;

impl Solver for Y2022 {
    fn run_solution(day: u8, input: Option<String>) -> Vec<String> {
        match day {
            1 => day01::Day01.run(input, 1, 2022),
            2 => day02::Day02.run(input, 2, 2022),
            3 => day03::Day03.run(input, 3, 2022),
            4 => day04::Day04.run(input, 4, 2022),
            5 => day05::Day05.run(input, 5, 2022),
            6 => day06::Day06.run(input, 6, 2022),
            7 => day07::Day07.run(input, 7, 2022),
            8 => day08::Day08.run(input, 8, 2022),
            9 => day09::Day09.run(input, 9, 2022),
            10 => day10::Day10.run(input, 10, 2022),
            11 => day11::Day11.run(input, 11, 2022),
            12 => day12::Day12.run(input, 12, 2022),
            13 => day13::Day13.run(input, 13, 2022),
            14 => day14::Day14.run(input, 14, 2022),
            15 => day15::Day15.run(input, 15, 2022),
            16 => day16::Day16.run(input, 16, 2022),
            17 => day17::Day17.run(input, 17, 2022),
            18 => day18::Day18.run(input, 18, 2022),
            19 => day19::Day19.run(input, 19, 2022),
            20 => day20::Day20.run(input, 20, 2022),
            21 => day21::Day21.run(input, 21, 2022),
            22 => day22::Day22.run(input, 22, 2022),
            23 => day23::Day23.run(input, 23, 2022),
            24 => day24::Day24.run(input, 24, 2022),
            25 => day25::Day25.run(input, 25, 2022),
            _ => vec![String::from("Solution not implemented (yet?)")],
        }
    }

    fn run_all() -> Vec<String> {
        let mut output = Vec::new();

        for day in 1..=MAX_DAYS {
            output.append(&mut Y2022::run_solution(day, None));
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
            12 => include_str!("./day12.rs"),
            13 => include_str!("./day13.rs"),
            14 => include_str!("./day14.rs"),
            15 => include_str!("./day15.rs"),
            16 => include_str!("./day16.rs"),
            17 => include_str!("./day17.rs"),
            18 => include_str!("./day18.rs"),
            19 => include_str!("./day19.rs"),
            20 => include_str!("./day20.rs"),
            21 => include_str!("./day21.rs"),
            22 => include_str!("./day22.rs"),
            23 => include_str!("./day23.rs"),
            24 => include_str!("./day24.rs"),
            25 => include_str!("./day25.rs"),
            _ => unimplemented!(),
        }
    }
}
