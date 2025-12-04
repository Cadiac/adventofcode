use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub type Coords = (isize, isize);

#[rustfmt::skip]
const NEIGHBOURS: [Coords; 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub struct Day04;

fn parse(input: &str) -> Result<HashSet<Coords>, AocError> {
    let mut result = HashSet::new();

    for (y, row) in input.trim().lines().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            if tile == '@' {
                result.insert((x as isize, y as isize));
            }
        }
    }

    Ok(result)
}

fn count_neighbours((x, y): &Coords, occupied: &HashSet<Coords>) -> usize {
    NEIGHBOURS
        .iter()
        .filter(|(dx, dy)| occupied.contains(&(x + dx, y + dy)))
        .count()
}

impl Solution for Day04 {
    type Part1 = usize;
    type Part2 = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day04.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let occupied = parse(input)?;

        let result = occupied
            .iter()
            .filter(|roll| count_neighbours(roll, &occupied) < 4)
            .count();

        Ok(result)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut occupied = parse(input)?;
        let mut removed = 0;

        loop {
            let before = occupied.len();
            let current = occupied.clone();
            occupied.retain(|roll| count_neighbours(roll, &current) >= 4);
            let after = occupied.len();

            removed += before - after;

            if before == after {
                break;
            }
        }

        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day04.part_1(
                "..@@.@@@@.\n\
                 @@@.@.@.@@\n\
                 @@@@@.@.@@\n\
                 @.@@@@..@.\n\
                 @@.@@@@.@@\n\
                 .@@@@@@@.@\n\
                 .@.@.@.@@@\n\
                 @.@@@.@@@@\n\
                 .@@@@@@@@.\n\
                 @.@.@@@.@."
            ),
            Ok(13)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day04.part_2(
                "..@@.@@@@.\n\
                 @@@.@.@.@@\n\
                 @@@@@.@.@@\n\
                 @.@@@@..@.\n\
                 @@.@@@@.@@\n\
                 .@@@@@@@.@\n\
                 .@.@.@.@@@\n\
                 @.@@@.@@@@\n\
                 .@@@@@@@@.\n\
                 @.@.@@@.@."
            ),
            Ok(43)
        );
    }
}
