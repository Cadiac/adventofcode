use crate::solution::{AocError, Solution};

pub struct Day06;

#[derive(Debug, Clone)]
struct Race {
    duration: u64,
    record: u64,
}

fn parse_part_1(input: &str) -> Result<Vec<Race>, AocError> {
    let mut iter = input.trim().lines();

    let durations = iter
        .next()
        .ok_or_else(|| AocError::parse(input, "Missing times"))?
        .strip_prefix("Time:")
        .ok_or_else(|| AocError::parse(input, "Missing time prefix"))?;

    let durations = parse_numbers(durations)?;

    let records = iter
        .next()
        .ok_or_else(|| AocError::parse(input, "Missing distances"))?
        .strip_prefix("Distance:")
        .ok_or_else(|| AocError::parse(input, "Missing distance prefix"))?;

    let records = parse_numbers(records)?;

    let races = durations
        .into_iter()
        .enumerate()
        .map(|(i, duration)| Race {
            duration,
            record: records[i],
        })
        .collect();

    Ok(races)
}

fn parse_part_2(input: &str) -> Result<Race, AocError> {
    let mut iter = input.trim().lines();

    let duration = iter
        .next()
        .ok_or_else(|| AocError::parse(input, "Missing times"))?
        .strip_prefix("Time:")
        .ok_or_else(|| AocError::parse(input, "Missing time prefix"))?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .map_err(|_| AocError::parse(input, "Error parsing duration"))?;

    let record = iter
        .next()
        .ok_or_else(|| AocError::parse(input, "Missing distances"))?
        .strip_prefix("Distance:")
        .ok_or_else(|| AocError::parse(input, "Missing distance prefix"))?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .map_err(|_| AocError::parse(input, "Error parsing record"))?;

    Ok(Race { duration, record })
}

fn parse_numbers(numbers: &str) -> Result<Vec<u64>, AocError> {
    numbers
        .split_whitespace()
        .map(|number| {
            number
                .parse::<u64>()
                .map_err(|_| AocError::parse(number, "Error parsing number"))
        })
        .collect()
}

impl Solution for Day06 {
    type F = usize;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day06.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let ways_to_beat = parse_part_1(input)?
            .into_iter()
            .map(|race| {
                (1..race.duration)
                    .filter(|button_held| {
                        let distance = button_held * (race.duration - button_held);
                        distance > race.record
                    })
                    .count()
            })
            .product();

        Ok(ways_to_beat)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let race = parse_part_2(input)?;

        let ways_to_beat = (1..race.duration)
            .filter(|button_held| {
                let distance = button_held * (race.duration - button_held);
                distance > race.record
            })
            .count();

        Ok(ways_to_beat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day06.part_1(
                "Time:      7  15   30\n\
                 Distance:  9  40  200\n"
            ),
            Ok(288)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day06.part_2(
                "Time:      7  15   30\n\
                 Distance:  9  40  200\n"
            ),
            Ok(71503)
        );
    }
}
