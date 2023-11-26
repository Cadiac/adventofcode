use crate::solution::{AocError, Solution};

pub struct Day02;

impl Solution for Day02 {
    type F = i32;
    type S = i32;

    fn meta(&self) -> (u32, u32) {
        (2, 2021)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day02.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let mut position: (i32, i32) = (0, 0);

        for command in input.lines() {
            let parts: Vec<&str> = command.split(' ').collect();
            let steps = parts[1].parse::<i32>().unwrap();

            match parts[0] {
                "forward" => position.0 += steps,
                "down" => position.1 += steps,
                "up" => position.1 -= steps,
                _ => unimplemented!(),
            }
        }

        Ok(position.0 * position.1)
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let mut position: (i32, i32) = (0, 0);
        let mut aim = 0;

        for command in input.lines() {
            let parts: Vec<&str> = command.split(' ').collect();
            let steps = parts[1].parse::<i32>().unwrap();

            match parts[0] {
                "forward" => {
                    position.0 += steps;
                    position.1 += steps * aim;
                }
                "down" => aim += steps,
                "up" => aim -= steps,
                _ => unimplemented!(),
            }
        }

        Ok(position.0 * position.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day02.part_1(
                "forward 5\n\
                down 5\n\
                forward 8\n\
                up 3\n\
                down 8\n\
                forward 2"
            ),
            Ok(150)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day02.part_2(
                "forward 5\n\
                down 5\n\
                forward 8\n\
                up 3\n\
                down 8\n\
                forward 2"
            ),
            Ok(900)
        );
    }
}
