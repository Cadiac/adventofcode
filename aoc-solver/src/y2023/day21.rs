use std::collections::HashSet;

use itertools::Itertools;

use crate::solution::{AocError, Solution};

const DIRECTIONS: [(i8, i8); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub struct Day21;

type Coords = (isize, isize);

#[derive(PartialEq)]
enum Tile {
    GardenPlot,
    Rock,
    Start,
}

fn parse(input: &str) -> Result<(Vec<Vec<Tile>>, Coords), AocError> {
    let mut start = None;

    let instructions = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, tile)| match tile {
                    '.' => Ok(Tile::GardenPlot),
                    '#' => Ok(Tile::Rock),
                    'S' => {
                        start = Some((x as isize, y as isize));
                        Ok(Tile::Start)
                    }
                    _ => Err(AocError::parse(tile, "Unexpected tile")),
                })
                .try_collect()
        })
        .try_collect()?;

    let start = start.ok_or(AocError::parse(input, "Missing starting position"))?;

    Ok((instructions, start))
}

impl Solution for Day21 {
    type A = u64;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day21.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let (tiles, start) = parse(input)?;
        let reachable = find_reachable(&tiles, start, 64, false);

        Ok(reachable.len() as u64)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let (tiles, start) = parse(input)?;

        // This works on many assumptions of what the input looks like.
        // At the input there are no rocks on the X=0 and Y=0 axis, so the
        // Elf can just travel to that direction for the whole distance.

        // When gardens are being visited it appears that in every other garden
        // the Elf only visits "odd" tiles and in every other "even" tiles, like on a checkboard.
        // As long as as there's enough distance left to fully explore the garden
        // these seem to be the only possible reachable shapes, lets call them "A" and "B".

        // Assume that the garden is square shaped.
        let size = tiles.len();

        // Explore gardens exactly enough to fill an area shaped like
        //
        //     , ^ .
        //   , / # \ .
        //   < # # # >
        //   ` \ # / ´
        //     ` v ´

        let max_steps = 2 * size + (size - 1) / 2;

        let reachable = find_reachable(&tiles, start, max_steps, true);

        // The gardens copies the Elf visits contain various kinds of shapes of
        // visited tiles.
        //
        //                           N
        //                          NNN
        //                        K NNN H
        //                       GG BBB CC
        //                      GGG BBB CCC
        //                    K GGG BBB CCC H
        //                   WW BBB AAA BBB EE
        //                  WWW BBB AAA BBB EEE
        //  zoomed out:      WW BBB AAA BBB EE
        //     K N H          J FFF BBB DDD I
        //   K G B C H          FFF BBB DDD
        //   W B A B E           FF BBB DD
        //   J F B D I            J SSS I
        //     J S I                SSS
        //                           S
        // The shapes are:
        // N, E, S, W: These are the points furthest away from start
        // A, B:       Fully filled shapes that repeat and fill the middle part,
        //             every second being "odd" and every second being "even"
        // C, D, F, G: These are missing a corner
        // H, I, J, K: These are the small corner pieces

        let size = size as isize;

        // Figure out the area of each of these garden copies
        let a_shape = visited_area(&reachable, (0, 0), size);
        let b_shape = visited_area(&reachable, (1, 0), size);

        // Pointy pieces
        let north = visited_area(&reachable, (0, -2), size);
        let east = visited_area(&reachable, (2, 0), size);
        let south = visited_area(&reachable, (0, 2), size);
        let west = visited_area(&reachable, (-2, 0), size);

        // Missing corner pieces
        let ne_missing_corner = visited_area(&reachable, (1, -1), size);
        let se_missing_corner = visited_area(&reachable, (1, 1), size);
        let sw_missing_corner = visited_area(&reachable, (-1, 1), size);
        let nw_missing_corner = visited_area(&reachable, (-1, -1), size);

        // Small corner pieces
        let ne_small_corner = visited_area(&reachable, (2, -1), size);
        let se_small_corner = visited_area(&reachable, (2, 1), size);
        let sw_small_corner = visited_area(&reachable, (-2, 1), size);
        let nw_small_corner = visited_area(&reachable, (-2, -1), size);

        // Our input distance is odd.
        // After exiting the middle area (65) we need to move accross
        // (26501365 - 65) / 131 gardens to reach the point
        let n = ((26501365 - ((size - 1) / 2)) / size) as u64;

        // On a piece of paper with some geometry and logic I determined the count of shapes to be
        let a_count = (n - 1).pow(2);
        let b_count = n.pow(2);
        let small_corner_count = n;
        let missing_corner_count = n - 1;

        Ok(a_shape * a_count
            + b_shape * b_count
            + north
            + east
            + south
            + west
            + (ne_missing_corner + se_missing_corner + sw_missing_corner + nw_missing_corner)
                * missing_corner_count
            + (ne_small_corner + se_small_corner + sw_small_corner + nw_small_corner)
                * small_corner_count)
    }
}

fn find_reachable(
    tiles: &[Vec<Tile>],
    start: (isize, isize),
    max_steps: usize,
    can_wrap: bool,
) -> HashSet<Coords> {
    let mut stack = vec![(start, 0)];
    let mut visited: HashSet<(Coords, usize)> = HashSet::new();
    let mut unique: HashSet<Coords> = HashSet::new();

    let width = tiles[0].len() as isize;
    let height = tiles.len() as isize;

    while let Some(((x, y), distance)) = stack.pop() {
        if !visited.insert(((x, y), distance)) {
            continue;
        }

        if distance == max_steps {
            unique.insert((x, y));
            continue;
        }

        for (dx, dy) in DIRECTIONS {
            let (next_x, next_y) = (x + dx as isize, y + dy as isize);

            if can_wrap {
                if tiles[next_x.rem_euclid(width) as usize][next_y.rem_euclid(height) as usize]
                    != Tile::Rock
                {
                    stack.push(((next_x, next_y), distance + 1));
                }
            } else if next_x >= 0
                && next_y >= 0
                && next_x < width
                && next_y < height
                && tiles[next_y as usize][next_x as usize] != Tile::Rock
            {
                stack.push(((next_x, next_y), distance + 1));
            }
        }
    }

    unique
}

fn visited_area(visited: &HashSet<Coords>, garden: Coords, size: isize) -> u64 {
    visited
        .iter()
        .filter(|(x, y)| {
            *x >= garden.0 * size
                && *x < (garden.0 + 1) * size
                && *y >= garden.1 * size
                && *y < (garden.1 + 1) * size
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const EXAMPLE_INPUT: &str =
        "...........\n\
         .....###.#.\n\
         .###.##..#.\n\
         ..#.#...#..\n\
         ....#.#....\n\
         .##..S####.\n\
         .##..#...#.\n\
         .......##..\n\
         .##.#.####.\n\
         .##..##.##.\n\
         ...........\n";

    #[test]
    fn it_solves_part1_example() {
        let parsed = parse(EXAMPLE_INPUT);
        assert!(parsed.is_ok());

        let (tiles, start) = parsed.unwrap();

        assert_eq!(find_reachable(&tiles, start, 6, false).len(), 16)
    }

    #[test]
    fn it_solves_part2_examples() {
        let parsed = parse(EXAMPLE_INPUT);
        assert!(parsed.is_ok());

        let (tiles, start) = parsed.unwrap();

        assert_eq!(find_reachable(&tiles, start, 10, true).len(), 50);
        assert_eq!(find_reachable(&tiles, start, 50, true).len(), 1594);
        assert_eq!(find_reachable(&tiles, start, 100, true).len(), 6536);
        // assert_eq!(find_reachable(&tiles, start, 500, true).len(), 167004);
        // assert_eq!(find_reachable(&tiles, start, 1000, true).len(), 668697);
        // assert_eq!(find_reachable(&tiles, start, 5000, true).len(), 16733044);
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(Day21.part_2(Day21.default_input()), Ok(592723929260582))
    }
}
