const INPUT_FILE: &str = include_str!("../../inputs/day02.txt");

use std::num::ParseIntError;
use std::str::FromStr;

use aoc::parse_from_str;

struct CommandPart1 {
    horizontal: i32,
    vertical: i32,
}

impl FromStr for CommandPart1 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let movement = parts[1].parse::<i32>()?;

        match parts[0] {
            "forward" => Ok(CommandPart1 {
                horizontal: movement,
                vertical: 0,
            }),
            "down" => Ok(CommandPart1 {
                horizontal: 0,
                vertical: movement,
            }),
            "up" => Ok(CommandPart1 {
                horizontal: 0,
                vertical: -movement,
            }),
            _ => unimplemented!(),
        }
    }
}

fn part_1(input: &str) -> i32 {
    let commands = parse_from_str::<CommandPart1>(input);
    // horizontal, depth
    let mut position: (i32, i32) = (0, 0);

    for command in commands {
        position.0 += command.horizontal;
        position.1 += command.vertical;
    }

    position.0 * position.1
}

struct CommandPart2 {
    aim: i32,
    movement: i32,
}

impl FromStr for CommandPart2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let steps = parts[1].parse::<i32>()?;

        match parts[0] {
            "forward" => Ok(CommandPart2 {
                aim: 0,
                movement: steps,
            }),
            "down" => Ok(CommandPart2 {
                aim: steps,
                movement: 0,
            }),
            "up" => Ok(CommandPart2 {
                aim: -steps,
                movement: 0,
            }),
            _ => unimplemented!(),
        }
    }
}

fn part_2(input: &str) -> i32 {
    let commands = parse_from_str::<CommandPart2>(input);

    // horizontal, depth
    let mut position: (i32, i32) = (0, 0);
    let mut aim = 0;

    for command in commands {
        aim += command.aim;

        position.0 += command.movement;
        position.1 += command.movement * aim;
    }

    position.0 * position.1
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "forward 5\n\
                down 5\n\
                forward 8\n\
                up 3\n\
                down 8\n\
                forward 2"
            ),
            150
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "forward 5\n\
                down 5\n\
                forward 8\n\
                up 3\n\
                down 8\n\
                forward 2"
            ),
            900
        );
    }
}
