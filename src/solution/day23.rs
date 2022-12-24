use std::collections::{HashMap, HashSet};

use itertools::{Itertools, MinMaxResult::MinMax};

use crate::solution::{AocError, Solution};

type Coords = (i64, i64);

#[rustfmt::skip]
const RULES: [([Coords; 3], Coords); 4] = [
    ([( 0, -1), ( 1, -1), (-1, -1)], ( 0, -1)), // N
    ([( 0,  1), ( 1,  1), (-1,  1)], ( 0,  1)), // S
    ([(-1,  0), (-1, -1), (-1,  1)], (-1,  0)), // W
    ([( 1,  0), ( 1, -1), ( 1,  1)], ( 1,  0)), // E
];

#[rustfmt::skip]
const NEIGHBOURS: [Coords; 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub struct Day23;

impl Day23 {
    fn parse(input: &str) -> Result<HashSet<Coords>, AocError> {
        let mut elves = HashSet::new();

        for (y, row) in input.lines().enumerate() {
            for (x, tile) in row.chars().enumerate() {
                if tile == '#' {
                    elves.insert((x as i64, y as i64));
                }
            }
        }

        Ok(elves)
    }

    fn simulate_round(elves: &mut HashSet<(i64, i64)>, round: usize) -> bool {
        let mut proposals: HashMap<Coords, Vec<Coords>> = HashMap::new();

        for elf in elves.iter() {
            if NEIGHBOURS
                .iter()
                .all(|neighbour| !elves.contains(&(elf.0 + neighbour.0, elf.1 + neighbour.1)))
            {
                continue;
            }

            if let Some((_, movement)) =
                RULES
                    .iter()
                    .cycle()
                    .skip(round % 4)
                    .take(4)
                    .find(|(checks, _)| {
                        checks.iter().all(|neighbour| {
                            !elves.contains(&(elf.0 + neighbour.0, elf.1 + neighbour.1))
                        })
                    })
            {
                proposals
                    .entry((elf.0 + movement.0, elf.1 + movement.1))
                    .or_default()
                    .push(*elf);
            }
        }

        proposals.retain(|_, proposals| proposals.len() == 1);

        for (_, proposals) in proposals.iter() {
            elves.remove(&proposals[0]);
        }

        for (target, _) in proposals.iter() {
            elves.insert(*target);
        }

        !proposals.is_empty()
    }

    fn count_ground(elves: HashSet<(i64, i64)>) -> Result<i64, AocError> {
        let x = elves.iter().map(|(x, _)| x).minmax();
        let y = elves.iter().map(|(_, y)| y).minmax();
        match (x, y) {
            (MinMax(min_x, max_x), MinMax(min_y, max_y)) => {
                Ok((max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i64)
            }
            _ => Err(AocError::logic("failed to get rectangle")),
        }
    }
}

impl Solution for Day23 {
    type F = i64;
    type S = usize;

    fn name(&self) -> &'static str {
        "Day 23"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day23.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let mut elves = Day23::parse(input)?;

        for round in 0..10 {
            Day23::simulate_round(&mut elves, round);
        }

        Day23::count_ground(elves)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut elves = Day23::parse(input)?;

        let mut round = 0;
        while Day23::simulate_round(&mut elves, round) {
            round += 1;
        }

        Ok(round + 1)
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str =
        ".....\n\
         ..##.\n\
         ..#..\n\
         .....\n\
         ..##.\n\
         .....";

    const INPUT_LARGE: &str =
        "..............\n\
         ..............\n\
         .......#......\n\
         .....###.#....\n\
         ...#...#.#....\n\
         ....#...##....\n\
         ...#.###......\n\
         ...##.#.##....\n\
         ....#..#......\n\
         ..............\n\
         ..............\n\
         ..............";

    #[test]
    fn it_solves_part1_small() {
        assert_eq!(Day23.part_1(INPUT_SMALL), Ok(25));
    }

    #[test]
    fn it_solves_part1_large() {
        assert_eq!(Day23.part_1(INPUT_LARGE), Ok(110));
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(Day23.part_2(INPUT_LARGE), Ok(20));
    }
}
