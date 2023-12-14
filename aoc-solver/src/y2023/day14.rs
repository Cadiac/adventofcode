use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

const SPIN_CYCLE: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

pub struct Day14;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Rounded,
    Cube,
    Empty,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Grid = Vec<Vec<Tile>>;

fn parse(input: &str) -> Result<Grid, AocError> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| match tile {
                    '.' => Ok(Tile::Empty),
                    '#' => Ok(Tile::Cube),
                    'O' => Ok(Tile::Rounded),
                    _ => Err(AocError::parse(tile, "Unexpected symbol")),
                })
                .try_collect()
        })
        .try_collect()
}

fn slide_rock(grid: &mut Grid, x: usize, y: usize, dx: isize, dy: isize) {
    if grid[y][x] != Tile::Rounded {
        return;
    }

    let mut target_x = x as isize;
    let mut target_y = y as isize;

    grid[y][x] = Tile::Empty;

    while can_slide(grid, target_x + dx, target_y + dy) {
        target_x += dx;
        target_y += dy;
    }

    grid[target_y as usize][target_x as usize] = Tile::Rounded;
}

fn can_slide(grid: &Grid, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && y < grid.len() as isize
        && x < grid[y as usize].len() as isize
        && grid[y as usize][x as usize] == Tile::Empty
}

fn tilt(grid: &mut Grid, direction: &Direction) {
    let (height, width) = (grid.len(), grid[0].len());

    match direction {
        Direction::North => {
            for y in 0..height {
                for x in 0..width {
                    slide_rock(grid, x, y, 0, -1);
                }
            }
        }
        Direction::South => {
            for y in (0..height).rev() {
                for x in 0..width {
                    slide_rock(grid, x, y, 0, 1);
                }
            }
        }
        Direction::West => {
            for y in 0..height {
                for x in 0..width {
                    slide_rock(grid, x, y, -1, 0);
                }
            }
        }
        Direction::East => {
            for y in 0..height {
                for x in (0..width).rev() {
                    slide_rock(grid, x, y, 1, 0);
                }
            }
        }
    }
}

fn collect_round_rocks(grid: &Grid) -> Vec<Vec<usize>> {
    let round_rocks = grid
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter_map(|(x, tile)| match tile {
                    Tile::Rounded => Some(x),
                    _ => None,
                })
                .collect()
        })
        .collect();

    round_rocks
}

fn spin(grid: &mut Grid, cycles: usize) {
    let mut seen: HashMap<Vec<Vec<usize>>, usize> = HashMap::new();
    let mut simulated: Vec<Grid> = vec![];

    for cycle in 0..cycles {
        let round_rocks = collect_round_rocks(grid);

        if let Some(first_occurence) = seen.insert(round_rocks, cycle) {
            let repeats_every = cycle - first_occurence;
            let remaining_cycle_at_end = (cycles - first_occurence) % repeats_every - 1;

            *grid = simulated.swap_remove(first_occurence + remaining_cycle_at_end);
            return;
        }

        for direction in SPIN_CYCLE.iter() {
            tilt(grid, direction);
        }

        simulated.push(grid.clone());
    }
}

fn support_beams_load(grid: &Grid) -> u32 {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .map(|tile| match tile {
                    Tile::Rounded => y as u32 + 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum()
}

impl Solution for Day14 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day14.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let mut grid = parse(input)?;

        tilt(&mut grid, &Direction::North);
        let total_load = support_beams_load(&grid);

        Ok(total_load)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let mut grid = parse(input)?;

        spin(&mut grid, 1000000000);
        let total_load = support_beams_load(&grid);

        Ok(total_load)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day14.part_1(
                "O....#....\n\
                 O.OO#....#\n\
                 .....##...\n\
                 OO.#O....O\n\
                 .O.....O#.\n\
                 O.#..O.#.#\n\
                 ..O..#O..O\n\
                 .......O..\n\
                 #....###..\n\
                 #OO..#....\n"
            ),
            Ok(136)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day14.part_2(
                "O....#....\n\
                 O.OO#....#\n\
                 .....##...\n\
                 OO.#O....O\n\
                 .O.....O#.\n\
                 O.#..O.#.#\n\
                 ..O..#O..O\n\
                 .......O..\n\
                 #....###..\n\
                 #OO..#....\n"
            ),
            Ok(64)
        );
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(Day14.part_2(Day14.default_input()), Ok(87700));
    }
}
