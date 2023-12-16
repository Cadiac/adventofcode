use std::collections::HashSet;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day16;

#[derive(Clone)]
enum Tile {
    Empty,
    Mirror(char),
    Splitter(char),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Grid = Vec<Vec<Tile>>;

fn parse(input: &str) -> Result<(Grid, usize, usize), AocError> {
    let grid: Grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| match tile {
                    '.' => Ok(Tile::Empty),
                    '/' | '\\' => Ok(Tile::Mirror(tile)),
                    '|' | '-' => Ok(Tile::Splitter(tile)),
                    _ => Err(AocError::parse(tile, "Unexpected symbol")),
                })
                .try_collect()
        })
        .try_collect()?;

    let height = grid.len();
    let width = grid
        .first()
        .map(|row| row.len())
        .ok_or(AocError::parse(input, "Empty input?"))?;

    Ok((grid, height, width))
}

impl Solution for Day16 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day16.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (grid, height, width) = parse(input)?;
        let count = energize(width, height, &grid, (0, 0, Direction::East))?;

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (grid, height, width) = parse(input)?;

        let mut max_energized = 0;

        for x in 0..width {
            let count = energize(width, height, &grid, (x as isize, 0, Direction::South))?;
            max_energized = usize::max(max_energized, count);

            let count = energize(
                width,
                height,
                &grid,
                (x as isize, height as isize - 1, Direction::North),
            )?;
            max_energized = usize::max(max_energized, count);
        }

        for y in 0..height {
            let count = energize(width, height, &grid, (0, y as isize, Direction::East))?;
            max_energized = usize::max(max_energized, count);

            let count = energize(
                width,
                height,
                &grid,
                (width as isize - 1, y as isize, Direction::West),
            )?;
            max_energized = usize::max(max_energized, count);
        }

        Ok(max_energized)
    }
}

fn energize(
    width: usize,
    height: usize,
    grid: &[Vec<Tile>],
    entry: (isize, isize, Direction),
) -> Result<usize, AocError> {
    let mut beams = vec![entry];
    let mut seen: HashSet<(isize, isize, Direction)> = HashSet::new();

    while let Some((x, y, direction)) = beams.pop() {
        if x >= 0
            && y >= 0
            && (x as usize) < width
            && (y as usize) < height
            && seen.insert((x, y, direction))
        {
            match grid[y as usize][x as usize] {
                Tile::Empty => {
                    match direction {
                        Direction::North => beams.push((x, y - 1, direction)),
                        Direction::East => beams.push((x + 1, y, direction)),
                        Direction::South => beams.push((x, y + 1, direction)),
                        Direction::West => beams.push((x - 1, y, direction)),
                    };
                }
                Tile::Mirror(mirror) => {
                    match (mirror, direction) {
                        ('/', Direction::South) => beams.push((x - 1, y, Direction::West)),
                        ('/', Direction::North) => beams.push((x + 1, y, Direction::East)),
                        ('/', Direction::East) => beams.push((x, y - 1, Direction::North)),
                        ('/', Direction::West) => beams.push((x, y + 1, Direction::South)),

                        ('\\', Direction::South) => beams.push((x + 1, y, Direction::East)),
                        ('\\', Direction::North) => beams.push((x - 1, y, Direction::West)),
                        ('\\', Direction::East) => beams.push((x, y + 1, Direction::South)),
                        ('\\', Direction::West) => beams.push((x, y - 1, Direction::North)),

                        _ => return Err(AocError::parse(mirror, "Unexpected mirror")),
                    };
                }
                Tile::Splitter(splitter) => {
                    match (splitter, direction) {
                        ('|', Direction::South) => beams.push((x, y + 1, direction)),
                        ('|', Direction::North) => beams.push((x, y - 1, direction)),
                        ('|', _) => {
                            beams.push((x, y - 1, Direction::North));
                            beams.push((x, y + 1, Direction::South));
                        }

                        ('-', Direction::East) => beams.push((x + 1, y, direction)),
                        ('-', Direction::West) => beams.push((x - 1, y, direction)),
                        ('-', _) => {
                            beams.push((x + 1, y, Direction::East));
                            beams.push((x - 1, y, Direction::West));
                        }

                        _ => return Err(AocError::parse(splitter, "Unexpected splitter")),
                    };
                }
            }
        }
    }

    let energized: HashSet<(isize, isize)> = seen.into_iter().map(|(x, y, _)| (x, y)).collect();

    Ok(energized.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day16.part_1(
                [
                    r".|...\....",
                    r"|.-.\.....",
                    r".....|-...",
                    r"........|.",
                    r"..........",
                    r".........\",
                    r"..../.\\..",
                    r".-.-/..|..",
                    r".|....-|.\",
                    r"..//.|...."
                ]
                .join("\n")
                .as_str()
            ),
            Ok(46)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day16.part_2(
                [
                    r".|...\....",
                    r"|.-.\.....",
                    r".....|-...",
                    r"........|.",
                    r"..........",
                    r".........\",
                    r"..../.\\..",
                    r".-.-/..|..",
                    r".|....-|.\",
                    r"..//.|...."
                ]
                .join("\n")
                .as_str()
            ),
            Ok(51)
        );
    }
}
