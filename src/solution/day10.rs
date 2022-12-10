use crate::solution::{AocError, Solution};

pub struct Day10;

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Day10 {
    fn parse(input: &str) -> Result<Vec<Instruction>, AocError> {
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_ascii_whitespace();

                match iter.next() {
                    Some("addx") => {
                        let arg = iter
                            .next()
                            .ok_or_else(|| AocError::parse(line, "missing arg for addx"))?
                            .parse::<i32>()
                            .map_err(|err| AocError::parse(line, err))?;

                        Ok(Instruction::Addx(arg))
                    }
                    Some("noop") => Ok(Instruction::Noop),
                    _ => Err(AocError::parse(line, "unknown instruction")),
                }
            })
            .collect()
    }
}

impl Solution for Day10 {
    type F = i32;
    type S = String;

    fn name(&self) -> &'static str {
        "Day 09"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day10.txt")
    }

    fn part_1(&self, input: &str) -> Result<i32, AocError> {
        let mut instructions = Self::parse(input)?.into_iter();
        let mut register = 1;

        let mut task = Instruction::Noop;
        let mut remaining_cycles = 0;

        let mut cycle = 1;
        let mut signals_sum = 0;

        loop {
            if remaining_cycles == 0 {
                if let Instruction::Addx(value) = task {
                    register += value;
                }

                match instructions.next() {
                    Some(addx @ Instruction::Addx(_)) => {
                        task = addx;
                        remaining_cycles = 2;
                    }
                    Some(noop @ Instruction::Noop) => {
                        task = noop;
                        remaining_cycles = 1;
                    }
                    None => break,
                }
            }

            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                signals_sum += register * cycle
            }

            remaining_cycles -= 1;
            cycle += 1;
        }

        Ok(signals_sum)
    }

    fn part_2(&self, input: &str) -> Result<String, AocError> {
        let mut instructions = Self::parse(input)?.into_iter();

        let mut cycle = 1;
        let mut register = 1;

        let mut task = Instruction::Noop;
        let mut remaining_cycles = 0;

        let mut output: Vec<char> = vec!['\n'];

        loop {
            if remaining_cycles == 0 {
                if let Instruction::Addx(value) = task {
                    register += value;
                }

                match instructions.next() {
                    Some(addx @ Instruction::Addx(_)) => {
                        task = addx;
                        remaining_cycles = 2;
                    }
                    Some(noop @ Instruction::Noop) => {
                        task = noop;
                        remaining_cycles = 1;
                    }
                    None => break,
                }
            }

            if i32::abs(register - (cycle - 1) % 40) <= 1 {
                output.push('#');
            } else {
                output.push('.');
            }
            if cycle % 40 == 0 {
                output.push('\n');
            }

            remaining_cycles -= 1;
            cycle += 1;
        }

        Ok(output.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LONG_EXAMPLE: &str = include_str!("../../inputs/day10-example.txt");

    #[test]
    fn it_solves_part1_example_small() {
        assert_eq!(
            Day10.part_1(
                "noop\n\
                 addx 3\n\
                 addx -5"
            ),
            Ok(0)
        );
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day10.part_1(LONG_EXAMPLE), Ok(13140));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day10.part_2(LONG_EXAMPLE),
            Ok(String::from(
                "\n\
                ##..##..##..##..##..##..##..##..##..##..\n\
                ###...###...###...###...###...###...###.\n\
                ####....####....####....####....####....\n\
                #####.....#####.....#####.....#####.....\n\
                ######......######......######......####\n\
                #######.......#######.......#######.....\n\
            "
            ))
        );
    }
}
