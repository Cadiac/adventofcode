use std::collections::HashSet;

use crate::solution::{AocError, Solution};

type Coords = (isize, isize);
type Obstructions = HashSet<Coords>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn as_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn parse(input: &str) -> Result<(Coords, Obstructions, usize, usize), AocError> {
    let mut obstructions = HashSet::new();
    let mut start = None;

    for (y, row) in input.trim().lines().enumerate() {
        for (x, tile) in row.trim().chars().enumerate() {
            match tile {
                '#' => {
                    obstructions.insert((x as isize, y as isize));
                }
                '^' => start = Some((x as isize, y as isize)),
                _ => (),
            }
        }
    }

    let height = input.trim().lines().count();
    let width = input.trim().lines().next().unwrap().chars().count();

    match start {
        Some(start) => Ok((start, obstructions, width, height)),
        None => Err(AocError::parse(input, "Missing start position")),
    }
}

fn is_within_bounds(pos: Coords, width: usize, height: usize) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < (width as isize) && pos.1 < (height as isize)
}

fn patrol(
    start: Coords,
    width: usize,
    height: usize,
    obstructions: HashSet<(isize, isize)>,
) -> (usize, bool) {
    let mut direction = Direction::Up;
    let mut pos = start;
    let mut visited = HashSet::new();
    let mut hit_obstructions_from: HashSet<(Coords, Direction)> = HashSet::new();

    loop {
        visited.insert(pos);

        let (dx, dy) = direction.as_delta();
        let next = (pos.0 + dx, pos.1 + dy);

        if !is_within_bounds(next, width, height) {
            return (visited.len(), false);
        }

        if obstructions.contains(&next) {
            if !hit_obstructions_from.insert((next, direction)) {
                return (visited.len(), true);
            }
            direction = direction.rotate_right()
        } else {
            pos = next
        }
    }
}

pub struct Day06;
impl Solution for Day06 {
    type Part1 = usize;
    type Part2 = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day06.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (start, obstructions, width, height) = parse(input)?;

        let (visited, _) = patrol(start, width, height, obstructions);

        Ok(visited)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (start, obstructions, width, height) = parse(input)?;

        let mut count = 0;

        for y in 0..height {
            for x in 0..width {
                let current = (x as isize, y as isize);

                if current == start {
                    continue;
                }

                let mut possible = obstructions.clone();
                if possible.insert(current) {
                    let (_, is_stuck_in_loop) = patrol(start, width, height, possible);

                    if is_stuck_in_loop {
                        count += 1
                    }
                }
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day06.part_1(
                "....#.....\n\
                 .........#\n\
                 ..........\n\
                 ..#.......\n\
                 .......#..\n\
                 ..........\n\
                 .#..^.....\n\
                 ........#.\n\
                 #.........\n\
                 ......#..."
            ),
            Ok(41)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day06.part_2(
                "....#.....\n\
                 .........#\n\
                 ..........\n\
                 ..#.......\n\
                 .......#..\n\
                 ..........\n\
                 .#..^.....\n\
                 ........#.\n\
                 #.........\n\
                 ......#..."
            ),
            Ok(6)
        );
    }

    #[test]
    fn it_should_not_get_stuck_in_loop() {
        let input = "....#.....\n\
                           .........#\n\
                           ..........\n\
                           ..#.......\n\
                           .......#..\n\
                           ..........\n\
                           .#..^.....\n\
                           ........#.\n\
                           #.........\n\
                           ......#...";

        let (start, obstructions, width, height) = parse(input).unwrap();

        assert_eq!(patrol(start, width, height, obstructions), (41, false))
    }

    #[test]
    fn it_should_get_stuck_in_loop_1() {
        let input = "....#.....\n\
                           .........#\n\
                           ..........\n\
                           ..#.......\n\
                           .......#..\n\
                           ..........\n\
                           .#.#^.....\n\
                           ........#.\n\
                           #.........\n\
                           ......#...";

        let (start, obstructions, width, height) = parse(input).unwrap();

        assert_eq!(patrol(start, width, height, obstructions), (18, true))
    }

    #[test]
    fn it_should_get_stuck_in_loop_2() {
        let input = "....#.....\n\
                           .........#\n\
                           ..........\n\
                           ..#.......\n\
                           .......#..\n\
                           ..........\n\
                           .#..^.....\n\
                           ......#.#.\n\
                           #.........\n\
                           ......#...";

        let (start, obstructions, width, height) = parse(input).unwrap();

        assert_eq!(patrol(start, width, height, obstructions), (26, true))
    }
}
