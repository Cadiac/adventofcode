use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day11;

#[rustfmt::skip]
const NEIGHBOUR_OFFSETS: [(i8, i8); 8] = [
    (-1, 1), (0, 1), (1, 1),
    (-1, 0),         (1, 0),
    (-1,-1), (0,-1), (1,-1),
];

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|energy_level| energy_level.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn simulate_step(mut octopuses: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, usize) {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    for row in &mut octopuses {
        for octopus in row {
            // First, the energy level of each octopus increases by 1.
            *octopus += 1;
        }
    }

    loop {
        let mut some_octopus_flashed = false;

        for y in 0..octopuses.len() {
            for x in 0..octopuses[y].len() {
                let width = octopuses[y].len() as i8;
                let height = octopuses.len() as i8;

                // Then, any octopus with an energy level greater than 9 flashes.
                if octopuses[y][x] > 9 {
                    // An octopus can only flash at most once per step.
                    if flashed.insert((x, y)) {
                        some_octopus_flashed = true;

                        // This increases the energy level of all adjacent octopuses by 1,
                        // including octopuses that are diagonally adjacent.
                        // If this causes an octopus to have an energy level greater than 9, it also flashes.
                        let neighbours = NEIGHBOUR_OFFSETS
                            .iter()
                            .map(|(offset_x, offset_y)| (offset_x + x as i8, offset_y + y as i8))
                            .filter(|(neighbour_x, neighbour_y)| {
                                *neighbour_x >= 0
                                    && *neighbour_x < width
                                    && *neighbour_y >= 0
                                    && *neighbour_y < height
                            });

                        for (neighbour_x, neighbour_y) in neighbours {
                            octopuses[neighbour_y as usize][neighbour_x as usize] += 1;
                        }
                    }
                }
            }
        }

        // This process continues as long as new octopuses keep having their energy level increased beyond 9.
        if !some_octopus_flashed {
            break;
        }
    }

    // Finally, any octopus that flashed during this step has its energy level set to 0, as
    // it used all of its energy to flash.
    let count = flashed.len();

    for (x, y) in flashed.into_iter() {
        octopuses[y][x] = 0;
    }

    (octopuses, count)
}

impl Solution for Day11 {
    type F = usize;
    type S = usize;

    fn meta(&self) -> (u32, u32) {
        (11, 2021)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day11.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let mut octopuses = parse(input);

        let mut total_flashes = 0;
        for _ in 0..100 {
            let (new_octopuses, flashes) = simulate_step(std::mem::take(&mut octopuses));
            octopuses = new_octopuses;
            total_flashes += flashes;
        }

        Ok(total_flashes)
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let mut octopuses = parse(input);

        let total_octopuses_count: usize = octopuses.iter().map(|row| row.len()).sum();

        let mut steps = 1;

        loop {
            let (new_octopuses, flashes) = simulate_step(std::mem::take(&mut octopuses));
            if flashes == total_octopuses_count {
                return Ok(steps);
            }

            octopuses = new_octopuses;
            steps += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_large_example() {
        assert_eq!(
            Day11.part_1(
                "5483143223\n\
                 2745854711\n\
                 5264556173\n\
                 6141336146\n\
                 6357385478\n\
                 4167524645\n\
                 2176841721\n\
                 6882881134\n\
                 4846848554\n\
                 5283751526",
            ),
            Ok(1656)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day11.part_2(
                "5483143223\n\
                 2745854711\n\
                 5264556173\n\
                 6141336146\n\
                 6357385478\n\
                 4167524645\n\
                 2176841721\n\
                 6882881134\n\
                 4846848554\n\
                 5283751526"
            ),
            Ok(195)
        );
    }
}
