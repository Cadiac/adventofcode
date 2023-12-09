use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use crate::solution::{AocError, Solution};

const ACTIONS: [(i32, i32); 5] = [(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)];

type Coords = (i32, i32);
type Blizzards = Vec<(Coords, Direction)>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Search {
    coords: Coords,
    minute: i32,
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.minute.cmp(&self.minute)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub struct Day24;

impl Day24 {
    fn parse(input: &str) -> Result<(Blizzards, Coords, Coords, Coords), AocError> {
        let mut blizzards = Vec::new();
        let start = input
            .lines()
            .next()
            .and_then(|line| line.chars().position(|c| c == '.'))
            .ok_or_else(|| AocError::parse(input, "no start"))?;
        let end = input
            .lines()
            .last()
            .and_then(|line| line.chars().position(|c| c == '.'))
            .ok_or_else(|| AocError::parse(input, "no end"))?;

        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;

        for (y, row) in input.lines().enumerate() {
            for (x, tile) in row.chars().enumerate() {
                if let Some(blizzard) = match tile {
                    '>' => Some(Direction::Right),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    '^' => Some(Direction::Up),
                    _ => None,
                } {
                    blizzards.push(((x as i32, y as i32), blizzard));
                }
            }
        }

        Ok((
            blizzards,
            (start as i32, 0),
            (end as i32, height - 1),
            (width, height),
        ))
    }

    fn blizzard_position(
        (x, y): Coords,
        direction: Direction,
        minute: i32,
        width: i32,
        height: i32,
    ) -> Coords {
        match direction {
            Direction::Right => ((minute + x - 1) % (width - 2) + 1, y),
            Direction::Left => ((-minute + x - 1).rem_euclid(width - 2) + 1, y),

            Direction::Down => (x, (minute + y - 1) % (height - 2) + 1),
            Direction::Up => (x, ((-minute + y - 1).rem_euclid(height - 2)) + 1),
        }
    }

    fn find_quickest(
        blizzards: &Blizzards,
        start_time: i32,
        start: Coords,
        end: Coords,
        (width, height): Coords,
    ) -> Option<i32> {
        let mut dist: HashMap<(Coords, (i32, i32)), i32> = HashMap::new();
        let mut queue: VecDeque<Search> = VecDeque::new();
        let mut blizzards_by_minute: HashMap<i32, Vec<Coords>> = HashMap::new();

        dist.insert(
            (start, (start_time % width, start_time % height)),
            start_time,
        );
        queue.push_back(Search {
            minute: start_time,
            coords: start,
        });

        while let Some(Search { coords, minute }) = queue.pop_front() {
            let current = (coords, (minute % width, minute % height));

            if coords == end {
                return Some(*dist.get(&current).unwrap_or(&i32::MAX));
            }

            if minute > *dist.entry(current).or_insert(i32::MAX) {
                continue;
            }

            for delta in ACTIONS {
                let next: (i32, i32) = (coords.0 + delta.0, coords.1 + delta.1);
                let is_within_bounds =
                    next.0 >= 1 && next.1 >= 1 && next.0 < width - 1 && next.1 < height - 1;
                let is_end = next.0 == end.0 && next.1 == end.1;
                let is_start = next.0 == start.0 && next.1 == start.1;

                if is_within_bounds || is_start || is_end {
                    let next_minute = minute + 1;
                    let is_blizzard_next_minute = blizzards_by_minute
                        .entry(next_minute)
                        .or_insert_with_key(|minute| {
                            // Calculate the blizzard locations only once for each minute
                            blizzards
                                .iter()
                                .map(|(initial_pos, direction)| {
                                    Day24::blizzard_position(
                                        *initial_pos,
                                        *direction,
                                        *minute,
                                        width,
                                        height,
                                    )
                                })
                                .collect()
                        })
                        .iter()
                        .any(|blizzard| next.0 == blizzard.0 && next.1 == blizzard.1);

                    if is_blizzard_next_minute {
                        continue;
                    }

                    let next_key = (next, (next_minute % width, next_minute % height));
                    let known_fastest = dist.entry(next_key).or_insert(i32::MAX);

                    if next_minute < *known_fastest {
                        queue.push_back(Search {
                            minute: next_minute,
                            coords: next,
                        });
                        *known_fastest = next_minute;
                    }
                }
            }
        }

        None
    }
}

impl Solution for Day24 {
    type A = i32;
    type B = i32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day24.txt")
    }

    fn part_1(&self, input: &str) -> Result<i32, AocError> {
        let (blizzards, start, end, dimensions) = Day24::parse(input)?;

        match Day24::find_quickest(&blizzards, 0, start, end, dimensions) {
            Some(duration) => Ok(duration),
            None => Err(AocError::logic("can't reach the end")),
        }
    }

    fn part_2(&self, input: &str) -> Result<i32, AocError> {
        let (blizzards, start, end, dimensions) = Day24::parse(input)?;

        let start_to_end = Day24::find_quickest(&blizzards, 0, start, end, dimensions)
            .ok_or_else(|| AocError::logic("can't reach the end"))?;

        let end_to_snack = Day24::find_quickest(&blizzards, start_to_end, end, start, dimensions)
            .ok_or_else(|| AocError::logic("can't reach the snacks"))?;

        let snack_to_end = Day24::find_quickest(&blizzards, end_to_snack, start, end, dimensions)
            .ok_or_else(|| AocError::logic("can't reach the end again"))?;

        Ok(snack_to_end)
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SIMPLE: &str =
        "#.#####\n\
         #.....#\n\
         #>....#\n\
         #.....#\n\
         #...v.#\n\
         #.....#\n\
         #####.#";

    const INPUT_COMPLEX: &str =
        "#.######\n\
         #>>.<^<#\n\
         #.<..<<#\n\
         #>v.><>#\n\
         #<^v^^>#\n\
         ######.#";

    #[test]
    fn it_solves_part1_simple() {
        assert_eq!(Day24.part_1(INPUT_SIMPLE), Ok(10));
    }

    #[test]
    fn it_solves_part1_complex() {
        assert_eq!(Day24.part_1(INPUT_COMPLEX), Ok(18));
    }

    #[test]
    fn it_solves_part2_complex() {
        assert_eq!(Day24.part_2(INPUT_COMPLEX), Ok(54));
    }
}
