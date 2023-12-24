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

fn visit_reachable(tiles: &[Vec<Tile>], start: (isize, isize), max_steps: usize) -> (u64, u64) {
    let mut stack = vec![(start, 0)];
    let mut visited: HashSet<(Coords, usize)> = HashSet::new();

    let mut even = HashSet::new();
    let mut odd = HashSet::new();

    let width = tiles[0].len() as isize;
    let height = tiles.len() as isize;

    while let Some(((x, y), distance)) = stack.pop() {
        if !visited.insert(((x, y), distance)) {
            continue;
        }

        if distance == max_steps - 1 {
            odd.insert((x, y));
        }

        if distance == max_steps {
            even.insert((x, y));
            continue;
        }

        for (dx, dy) in DIRECTIONS {
            let (next_x, next_y) = (x + dx as isize, y + dy as isize);

            if next_x >= 0
                && next_y >= 0
                && next_x < width
                && next_y < height
                && tiles[next_y as usize][next_x as usize] != Tile::Rock
            {
                stack.push(((next_x, next_y), distance + 1));
            }
        }
    }

    (even.len() as u64, odd.len() as u64)
}

impl Solution for Day21 {
    type A = u64;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day21.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let (tiles, start) = parse(input)?;
        let reachable = visit_reachable(&tiles, start, 64).0;

        Ok(reachable)
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

        // Figure out the area of visited tiles in each of these gardens by
        // visiting them starting from different points and with limited distances.

        // Assume that the garden is square shaped. Size of it seems to be odd.
        let size = tiles.len();

        let edge = size - 1;
        let middle = (size - 1) / 2;
        let midpoints = [
            (start.0, edge as isize),
            (0, start.1),
            (start.0, 0),
            (edge as isize, start.1),
        ];
        let corners = [
            (0, 0),
            (edge as isize, 0),
            (0, edge as isize),
            (edge as isize, edge as isize),
        ];

        // I tried to optimize this by calculating the shapes that are formed from common starting
        // points together, but this only saved time on even and odd pieces. If the difference between the distances
        // is high the gains of that approach are lost to checking `distances.contains`. Something better
        // should be done, perhaps providing it a sorted list of distances to search and only considering
        // the current target distance. But I'm not going to bother.

        let (even, odd) = visit_reachable(&tiles, start, size);

        let pointy_pieces: u64 = midpoints
            .iter()
            .map(|start| visit_reachable(&tiles, *start, edge).0)
            .sum();

        let missing_corner_pieces: u64 = corners
            .iter()
            .map(|start| visit_reachable(&tiles, *start, edge + middle).0)
            .sum();

        let small_corner_pieces: u64 = corners
            .iter()
            .map(|start| visit_reachable(&tiles, *start, middle - 1).0)
            .sum();

        // Our input distance is odd.
        // After exiting the middle area (65) we need to move accross
        // (26501365 - 65) / 131 gardens to reach the point
        let n = ((26501365 - ((size - 1) / 2)) / size) as u64;

        // On a piece of paper with some geometry and logic I determined the count of shapes to be
        let even_count = (n - 1).pow(2);
        let odd_count = n.pow(2);
        let small_corner_count = n;
        let missing_corner_count = n - 1;

        Ok(even * even_count
            + odd * odd_count
            + pointy_pieces
            + missing_corner_pieces * missing_corner_count
            + small_corner_pieces * small_corner_count)
    }
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

        assert_eq!(visit_reachable(&tiles, start, 6).0, 16)
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(Day21.part_2(Day21.default_input()), Ok(592723929260582))
    }

    #[test]
    fn it_finds_correct_shapes() {
        // A = 7226
        let parsed = parse(Day21.default_input());
        assert!(parsed.is_ok());

        let (tiles, start) = parsed.unwrap();
        let size = tiles.len();

        assert_eq!(visit_reachable(&tiles, start, size), (7226, 7257));
    }
}
