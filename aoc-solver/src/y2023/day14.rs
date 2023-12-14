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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

fn slide_rock(mut tiles: Grid, x: usize, y: usize, dx: isize, dy: isize) -> Grid {
    if tiles[y][x] != Tile::Rounded {
        return tiles;
    }

    let mut target_x = x as isize;
    let mut target_y = y as isize;

    tiles[y][x] = Tile::Empty;

    while can_slide(&tiles, target_x + dx, target_y + dy) {
        target_x += dx;
        target_y += dy;
    }

    tiles[target_y as usize][target_x as usize] = Tile::Rounded;

    tiles
}

fn can_slide(tiles: &Grid, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && y < tiles.len() as isize
        && x < tiles[y as usize].len() as isize
        && tiles[y as usize][x as usize] == Tile::Empty
}

fn tilt(mut tiles: Grid, direction: &Direction) -> Grid {
    match direction {
        Direction::North => {
            for y in 0..tiles.len() {
                for x in 0..tiles[y].len() {
                    tiles = slide_rock(tiles, x, y, 0, -1);
                }
            }
        }
        Direction::South => {
            for y in (0..tiles.len()).rev() {
                for x in 0..tiles[y].len() {
                    tiles = slide_rock(tiles, x, y, 0, 1);
                }
            }
        }
        Direction::West => {
            for y in 0..tiles.len() {
                for x in 0..tiles[y].len() {
                    tiles = slide_rock(tiles, x, y, -1, 0);
                }
            }
        }
        Direction::East => {
            for y in 0..tiles.len() {
                for x in (0..tiles[y].len()).rev() {
                    tiles = slide_rock(tiles, x, y, 1, 0);
                }
            }
        }
    }

    tiles
}

fn spin(mut tiles: Grid, cycles: usize) -> Grid {
    let mut seen: HashMap<(Grid, Direction), usize> = HashMap::new();
    let mut simulated: Vec<Grid> = vec![];

    for cycle in 0..cycles {
        if let Some(first_occurence) = seen.insert((tiles.clone(), Direction::North), cycle) {
            let skips_every = cycle - first_occurence;
            let remaining_cycle_at_end = (cycles - first_occurence) % skips_every - 1;

            return simulated.swap_remove(first_occurence + remaining_cycle_at_end);
        }

        for direction in SPIN_CYCLE.iter() {
            tiles = tilt(tiles, direction);
        }

        simulated.push(tiles.clone());
    }

    tiles
}

fn support_beams_load(tiles: &Grid) -> u32 {
    tiles
        .iter()
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
        let grid = parse(input)?;

        let tilted = tilt(grid, &Direction::North);
        let total_load = support_beams_load(&tilted);

        Ok(total_load)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let grid = parse(input)?;

        let cycled = spin(grid, 1000000000);
        let total_load = support_beams_load(&cycled);

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
    fn it_tilts_north() {
        assert_eq!(
            tilt(
                parse(
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
                )
                .unwrap(),
                &Direction::North
            ),
            parse(
                "OOOO.#.O..\n\
                 OO..#....#\n\
                 OO..O##..O\n\
                 O..#.OO...\n\
                 ........#.\n\
                 ..#....#.#\n\
                 ..O..#.O.O\n\
                 ..O.......\n\
                 #....###..\n\
                 #....#....\n"
            )
            .unwrap()
        );
    }

    #[test]
    fn it_tilts_east() {
        assert_eq!(
            tilt(parse("O.O..#.O..\n").unwrap(), &Direction::East),
            parse("...OO#...O\n").unwrap()
        );
    }

    #[test]
    fn it_tilts_west() {
        assert_eq!(
            tilt(parse("O.O..#.O..\n").unwrap(), &Direction::West),
            parse("OO...#O...\n").unwrap()
        );
    }

    #[test]
    fn it_tilts_south() {
        assert_eq!(
            tilt(
                parse(
                    "O.\n\
                 .#\n\
                 ..\n\
                 OO\n\
                 #O\n\
                 O.\n\
                 .#\n"
                )
                .unwrap(),
                &Direction::South
            ),
            parse(
                "..\n\
                 .#\n\
                 O.\n\
                 O.\n\
                 #O\n\
                 .O\n\
                 O#\n"
            )
            .unwrap()
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
