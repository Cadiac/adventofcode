use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day14;

#[derive(Clone)]
enum Tile {
    Rounded,
    Cube,
    Empty,
}

enum Direction {
    North,
    East,
    South,
    West,
}

const SPIN_CYCLE: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

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

fn tilt(tiles: &mut Grid, direction: &Direction) {
    match direction {
        Direction::North => slide_north(tiles),
        Direction::East => slide_east(tiles),
        Direction::South => slide_south(tiles),
        Direction::West => slide_west(tiles),
    }
}

fn slide_north(tiles: &mut Vec<Vec<Tile>>) {
    for y in 0..tiles.len() {
        for x in 0..tiles[y].len() {
            if let Tile::Rounded = tiles[y][x] {
                let mut target_y = y;

                tiles[y][x] = Tile::Empty;

                while target_y > 0 && matches!(tiles[target_y - 1][x], Tile::Empty) {
                    target_y -= 1
                }

                tiles[target_y][x] = Tile::Rounded;
            }
        }
    }
}

fn slide_east(tiles: &mut Vec<Vec<Tile>>) {
    for row in tiles {
        for x in (0..row.len()).rev() {
            if let Tile::Rounded = row[x] {
                let mut target_x = x;

                row[x] = Tile::Empty;

                while target_x < row.len() - 1 && matches!(row[target_x + 1], Tile::Empty) {
                    target_x += 1
                }

                row[target_x] = Tile::Rounded;
            }
        }
    }
}

fn slide_south(tiles: &mut Vec<Vec<Tile>>) {
    for y in (0..tiles.len()).rev() {
        for x in 0..tiles[y].len() {
            if let Tile::Rounded = tiles[y][x] {
                let mut target_y = y;

                tiles[y][x] = Tile::Empty;

                while target_y < tiles.len() - 1 && matches!(tiles[target_y + 1][x], Tile::Empty) {
                    target_y += 1
                }

                tiles[target_y][x] = Tile::Rounded;
            }
        }
    }
}

fn slide_west(tiles: &mut Vec<Vec<Tile>>) {
    for row in tiles {
        for x in 0..row.len() {
            if let Tile::Rounded = row[x] {
                let mut target_x = x;

                row[x] = Tile::Empty;

                while target_x > 0 && matches!(row[target_x - 1], Tile::Empty) {
                    target_x -= 1
                }

                row[target_x] = Tile::Rounded;
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
