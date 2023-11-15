use crate::solution::{AocError, Solution};

pub struct Day01;

fn parse(input: &str) -> Result<Vec<u64>, AocError> {
    let mut values = Vec::new();

    for chunk in input.split("\n\n") {
        let mut sum = 0;

        for line in chunk.lines() {
            sum += line
                .parse::<u64>()
                .map_err(|err| AocError::parse(line, err))?;
        }

        values.push(sum);
    }

    Ok(values)
}

impl Solution for Day01 {
    type F = u64;
    type S = u64;

    fn meta(&self) -> (u32, u32) {
        (1, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/2022/day01.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        parse(input)?
            .into_iter()
            .max()
            .ok_or_else(|| AocError::logic("no distinct max"))
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        parse(input).map(|mut elves| {
            elves.sort();
            elves.iter().rev().take(3).sum()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day01.part_1(
                "1000\n\
                2000\n\
                3000\n\
                \n\
                4000\n\
                \n\
                5000\n\
                6000\n\
                \n\
                7000\n\
                8000\n\
                9000\n\
                \n\
                10000"
            ),
            Ok(24000)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day01.part_2(
                "1000\n\
                2000\n\
                3000\n\
                \n\
                4000\n\
                \n\
                5000\n\
                6000\n\
                \n\
                7000\n\
                8000\n\
                9000\n\
                \n\
                10000"
            ),
            Ok(45000)
        );
    }

    #[test]
    fn it_handles_broken_input() {
        assert_eq!(
            Day01.part_2(
                "10a00\n\
                2000\n\
                3000
                \n\
                10000"
            ),
            Err(AocError::parse("10a00", "invalid digit found in string"))
        );
    }
}
