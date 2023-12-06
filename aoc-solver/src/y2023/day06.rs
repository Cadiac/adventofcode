use crate::solution::{AocError, Solution};

pub struct Day06;

#[derive(Debug, Clone)]
struct Race {
    distance: u64,
    time: u64,
}

fn parse_part_1(input: &str) -> Result<Vec<Race>, AocError> {
    let (time_line, distance_line) = input
        .trim()
        .split_once('\n')
        .ok_or(AocError::parse(input, "Invalid input"))?;

    let times = time_line
        .strip_prefix("Time:")
        .ok_or_else(|| AocError::parse(input, "Missing time prefix"))?;

    let times = parse_numbers(times)?;

    let distances = distance_line
        .strip_prefix("Distance:")
        .ok_or_else(|| AocError::parse(input, "Missing distance prefix"))?;

    let distances = parse_numbers(distances)?;

    let races = distances
        .into_iter()
        .zip(times)
        .map(|(distance, time)| Race { distance, time })
        .collect();

    Ok(races)
}

fn parse_numbers(numbers: &str) -> Result<Vec<u64>, AocError> {
    numbers
        .split_whitespace()
        .map(|number| {
            number
                .parse()
                .map_err(|_| AocError::parse(number, "Error parsing number"))
        })
        .collect()
}

fn parse_part_2(input: &str) -> Result<Race, AocError> {
    let (time_line, distance_line) = input
        .trim()
        .split_once('\n')
        .ok_or(AocError::parse(input, "Invalid input"))?;

    let time = time_line
        .strip_prefix("Time:")
        .ok_or(AocError::parse(time_line, "Missing time prefix"))?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .map_err(|_| AocError::parse(time_line, "Error parsing time"))?;

    let distance = distance_line
        .strip_prefix("Distance:")
        .ok_or_else(|| AocError::parse(distance_line, "Missing distance prefix"))?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .map_err(|_| AocError::parse(distance_line, "Error parsing distance"))?;

    Ok(Race { distance, time })
}

impl Solution for Day06 {
    type F = usize;
    type S = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day06.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let ways_to_beat = parse_part_1(input)?
            .into_iter()
            .map(|race| {
                (1..race.time)
                    .filter(|button_held| {
                        let distance = button_held * (race.time - button_held);
                        distance > race.distance
                    })
                    .count()
            })
            .product();

        Ok(ways_to_beat)
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let race = parse_part_2(input)?;

        // My original solution was to just filter the durations.
        // This runs in ~12,6ms on Macbook Air M2, not too bad.

        // let ways_to_beat = (1..race.duration)
        //     .filter(|button_held| {
        //         let distance = button_held * (race.duration - button_held);
        //         distance > race.record
        //     })
        //     .count();

        // But lets try to be a little bit smarter. This runs in ~1,2µs on the same machine.
        // Solving quadratic formula from ax^2 + bx + c = 0 from the valid range edges:

        // x * (time - x) = distance
        // −x^2 + time * x − distance = 0
        // x = (time ± sqrt(time^2 - 4 * time * distance)) / 2

        // We get two bounds for the range of possible values. We're interested in finding
        // where > 0, and the formula is -x^2 parabola, so the valid range of times to hold
        // the button for should be found between these bounds.

        let sqrt = ((race.time.pow(2) - 4 * race.distance) as f64).sqrt();
        let start = (race.time as f64 - sqrt) / 2.0;
        let end = (race.time as f64 + sqrt) / 2.0;
        let ways_to_beat = end as i64 - start as i64;

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
