use std::collections::HashMap;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

use log::debug;

type Puzzle = HashMap<(usize, usize), Tile>;

use crate::solution::{AocError, Solution};

pub struct Day20;

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    edges: [String; 4],
    data: Vec<Vec<char>>,
    rotation: u8,
    is_flipped: bool,
}

fn rotate_left<T>(data: Vec<Vec<T>>) -> Vec<Vec<T>> {
    // reverse every row + transpose rotates the data counter-clockwise 90 degrees.
    // we could probably live with that
    // Transpose the data
    let size = data.len();

    let mut line_iters: Vec<_> = data
        .into_iter()
        .map(|line| line.into_iter().rev())
        .collect();

    (0..size)
        .map(|_| {
            line_iters
                .iter_mut()
                .map(|line| line.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn flip<T>(data: Vec<Vec<T>>) -> Vec<Vec<T>> {
    data.into_iter()
        .map(|line| line.into_iter().rev().collect())
        .collect()
}

impl FromStr for Tile {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let title_row = lines.next();
        let id = match title_row {
            Some(row) => {
                let parts: Vec<&str> = row.split(' ').collect();
                let id_part: Vec<&str> = parts[1].split(':').collect();
                id_part[0].parse::<u64>().unwrap()
            }
            None => 0,
        };

        let mut top_edge: String = String::new();
        let mut right_edge: Vec<char> = Vec::new();
        let mut bottom_edge: String = String::new();
        let mut left_edge: Vec<char> = Vec::new();

        let mut data: Vec<Vec<char>> = vec![];

        for (y, line) in lines.enumerate() {
            let mut chars: Vec<char> = vec![];
            for (x, pixel) in line.chars().enumerate() {
                if x == 0 {
                    left_edge.push(pixel);
                } else if x == 9 {
                    right_edge.push(pixel);
                } else {
                    chars.push(pixel);
                }
            }

            if y == 0 {
                top_edge = String::from(line);
            } else if y == 9 {
                bottom_edge = String::from(line);
            } else {
                data.push(chars);
            }
        }

        let edges: [String; 4] = [
            top_edge,
            right_edge.iter().collect(),
            bottom_edge,
            left_edge.iter().collect(),
        ];

        Ok(Tile {
            id,
            edges,
            data,
            rotation: 0,
            is_flipped: false,
        })
    }
}

impl Tile {
    fn rotate_left(&mut self) {
        self.edges = [
            std::mem::take(&mut self.edges[1]),
            self.edges[2].chars().rev().collect(),
            std::mem::take(&mut self.edges[3]),
            self.edges[0].chars().rev().collect(),
        ];

        self.rotation = (self.rotation + 1) % 4;

        self.data = rotate_left(std::mem::take(&mut self.data));
    }

    // flip vertically
    fn flip(&mut self) {
        self.edges = [
            self.edges[0].chars().rev().collect(),
            std::mem::take(&mut self.edges[3]),
            self.edges[2].chars().rev().collect(),
            std::mem::take(&mut self.edges[1]),
        ];

        self.is_flipped = !self.is_flipped;

        self.data = flip(std::mem::take(&mut self.data));
    }
}

struct Solver {
    tiles: VecDeque<Tile>,
    puzzle: Puzzle,
    size: usize,
}

impl Solver {
    fn new(input: &str) -> Solver {
        let tiles: VecDeque<Tile> = Self::parse_input(input);

        let size = (tiles.len() as f64).sqrt() as usize;

        debug!("Solving {:?} x {:?} puzzle", size, size);

        Solver {
            tiles,
            size,
            puzzle: HashMap::new(),
        }
    }

    fn parse_input(input: &str) -> VecDeque<Tile> {
        input
            .split("\n\n")
            .map(|tile_str| tile_str.parse::<Tile>().unwrap())
            .collect()
    }

    fn is_tile_valid_at_cursor(&self, tile: &Tile, cursor: (usize, usize)) -> bool {
        // above
        if cursor.1 > 0 {
            if let Some(neighbour) = self.puzzle.get(&(cursor.0, cursor.1 - 1)) {
                if tile.edges[0] != neighbour.edges[2] {
                    return false;
                }
            }
        }
        // below
        if let Some(neighbour) = self.puzzle.get(&(cursor.0, cursor.1 + 1)) {
            if tile.edges[2] != neighbour.edges[0] {
                return false;
            }
        }
        // to right
        if let Some(neighbour) = self.puzzle.get(&(cursor.0 + 1, cursor.1)) {
            if tile.edges[1] != neighbour.edges[3] {
                return false;
            }
        }
        // to left
        if cursor.0 > 0 {
            if let Some(neighbour) = self.puzzle.get(&(cursor.0 - 1, cursor.1)) {
                if tile.edges[3] != neighbour.edges[1] {
                    return false;
                }
            }
        }
        true
    }

    fn find_next_cursor(&self, cursor: (usize, usize)) -> (usize, usize) {
        if cursor.0 + 1 < self.size {
            return (cursor.0 + 1, cursor.1);
        }
        // This will go over `size` on y-axis, but it should never actually be used like that
        (0, cursor.1 + 1)
    }

    fn build_puzzle(&mut self, cursor: (usize, usize)) -> bool {
        // Done, we managed to place all tiles somewhere
        if self.tiles.is_empty() {
            return true;
        }

        // Otherwise try to fit some tile to current cursor
        let next_cursor = self.find_next_cursor(cursor);

        for _ in 0..self.tiles.len() {
            // Take a tile out of remaining tiles and check if it fits
            let mut tile = self.tiles.pop_front().unwrap();

            for rotation in 0..8 {
                if rotation == 4 {
                    // Try both sides
                    tile.flip();
                }
                tile.rotate_left();

                if self.is_tile_valid_at_cursor(&tile, cursor) {
                    self.puzzle.insert(cursor, tile);

                    if self.build_puzzle(next_cursor) {
                        // One solution is all we need
                        return true;
                    }

                    tile = self.puzzle.remove(&cursor).unwrap();
                }
            }

            // Tile didn't fit, put it back in and try another
            self.tiles.push_back(tile);
        }

        // No tile fit on board, abort this and go back to trying something else
        false
    }

    fn check_for_sea_monsters(data: &mut Vec<Vec<(char, bool)>>) {
        // A sea monster will look like this:
        //                   #
        // #    ##    ##    ###
        //  #  #  #  #  #  #
        #[rustfmt::skip]
        let sea_monster_pattern: Vec<(usize, usize)> = vec![
                                                                  (18, 0),
            (0, 1),   (5, 1),(6, 1),   (11, 1),(12, 1),   (17, 1),(18, 1),(19, 1),
             (1, 2), (4, 2),  (7, 2), (10, 2),  (13, 2), (16, 2),
        ];
        let height = data.len();
        for y in 0..height {
            if y + 2 >= height {
                break;
            }
            let width = data[y].len();
            for x in 0..width {
                if x + 19 >= width {
                    break;
                }
                let is_sea_monster = sea_monster_pattern
                    .iter()
                    .all(|(offset_x, offset_y)| data[y + offset_y][x + offset_x].0 == '#');
                if is_sea_monster {
                    // We've found a sea monster!
                    debug!("Found a sea monster, starting from ({}, {}):", x, y);
                    // Save that we've found monster from these pixels
                    for (offset_x, offset_y) in sea_monster_pattern.iter() {
                        data[y + offset_y][x + offset_x].1 = true;
                    }
                }
            }
        }
    }
}

impl Solution for Day20 {
    type F = u64;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day20.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let mut solver = Solver::new(input);
        if solver.build_puzzle((0, 0)) {
            debug!("Found a solution to the puzzle!");

            let result = solver.puzzle.get(&(0, 0)).unwrap().id
                * solver.puzzle.get(&(0, solver.size - 1)).unwrap().id
                * solver
                    .puzzle
                    .get(&(solver.size - 1, solver.size - 1))
                    .unwrap()
                    .id
                * solver.puzzle.get(&(solver.size - 1, 0)).unwrap().id;

            Ok(result)
        } else {
            Err(AocError::logic("No solution found"))
        }
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut solver = Solver::new(input);
        if solver.build_puzzle((0, 0)) {
            debug!("Found a solution to the puzzle!");

            let mut pixels: Vec<Vec<(char, bool)>> = vec![];
            for tile_y in 0..solver.size {
                let mut rows = vec![vec![]; 8];

                let mut row = vec![];
                for tile_x in 0..solver.size {
                    let tile = solver.puzzle.get(&(tile_x, tile_y)).unwrap();
                    row.push(tile.id.to_string());
                    for (y, row) in rows.iter_mut().enumerate().take(8) {
                        row.append(&mut tile.data[y].iter().cloned().map(|c| (c, false)).collect());
                    }
                }
                debug!("{}", row.join(" "));
                pixels.append(&mut rows);
            }
            debug!("Gathered total {}x{} pixels", pixels.len(), pixels.len());
            for row in pixels.iter() {
                debug!("{}", row.iter().map(|(c, _)| c).collect::<String>());
            }

            // Search for sea monsters in all rotations and on both sides
            for _rotation in 0..4 {
                Solver::check_for_sea_monsters(&mut pixels);
                pixels = rotate_left(pixels);
            }

            pixels = flip(pixels);

            for _flipped_rotation in 0..4 {
                Solver::check_for_sea_monsters(&mut pixels);
                pixels = rotate_left(pixels);
            }

            // Rotate once more to get nicer debug prints
            pixels = rotate_left(pixels);
            for pixel_row in &pixels {
                let mut row = vec![];
                for pixel in pixel_row.iter() {
                    if pixel.1 {
                        row.push("0");
                    } else {
                        row.push(" ");
                    }
                }
                debug!("{}", row.join(""));
            }

            // Count the pixels with sea monsters
            let result = pixels
                .iter()
                .map(|line| {
                    line.iter()
                        .filter(|(c, is_monster)| *c == '#' && !is_monster)
                        .count()
                })
                .sum();

            Ok(result)
        } else {
            Err(AocError::logic("No solution found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE: &str = include_str!("../../../inputs/2020/day20-example.txt");

    #[test]
    fn it_parses_tile() {
        let tile_str = "Tile 2311:\n\
             ..##.#..#.\n\
             ##..#.....\n\
             #...##..#.\n\
             ####.#...#\n\
             ##.##.###.\n\
             ##...#.###\n\
             .#.#.#..##\n\
             ..#....#..\n\
             ###...#.#.\n\
             ..###..###";

        let parsed_tile = tile_str.parse::<Tile>().unwrap();
        assert_eq!(parsed_tile.id, 2311);
        assert_eq!(parsed_tile.edges.len(), 4);
        assert_eq!(parsed_tile.edges[0], "..##.#..#.".to_owned());
        assert_eq!(parsed_tile.edges[1], "...#.##..#".to_owned());
        assert_eq!(parsed_tile.edges[2], "..###..###".to_owned());
        assert_eq!(parsed_tile.edges[3], ".#####..#.".to_owned());
    }

    #[test]
    fn it_parses_tiles() {
        let solver = Solver::new(EXAMPLE_FILE);
        assert_eq!(solver.tiles.len(), 9);
    }

    #[test]
    fn it_rotates_tile() {
        let tile_str = "Tile 2311:\n\
             ..##.#..#.\n\
             ##..#.....\n\
             #...##..#.\n\
             ####.#...#\n\
             ##.##.###.\n\
             ##...#.###\n\
             .#.#.#..##\n\
             ..#....#..\n\
             ###...#.#.\n\
             ..###..###";

        let mut parsed_tile = tile_str.parse::<Tile>().unwrap();
        assert_eq!(
            parsed_tile.data,
            vec![
                vec!['#', '.', '.', '#', '.', '.', '.', '.'],
                vec!['.', '.', '.', '#', '#', '.', '.', '#'],
                vec!['#', '#', '#', '.', '#', '.', '.', '.'],
                vec!['#', '.', '#', '#', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '#', '.', '#', '#'],
                vec!['#', '.', '#', '.', '#', '.', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '#', '.', '#'],
            ]
        );

        parsed_tile.rotate_left();
        assert_eq!(parsed_tile.edges[0], "...#.##..#".to_owned());
        assert_eq!(parsed_tile.edges[1], "###..###..".to_owned());
        assert_eq!(parsed_tile.edges[2], ".#####..#.".to_owned());
        assert_eq!(parsed_tile.edges[3], ".#..#.##..".to_owned());

        assert_eq!(
            parsed_tile.data,
            vec![
                vec!['.', '#', '.', '#', '#', '#', '.', '#'],
                vec!['.', '.', '.', '#', '#', '.', '#', '.'],
                vec!['.', '.', '.', '#', '.', '.', '.', '#'],
                vec!['.', '#', '#', '.', '#', '#', '.', '.'],
                vec!['#', '#', '.', '#', '.', '.', '.', '.'],
                vec!['.', '.', '#', '#', '.', '#', '.', '.'],
                vec!['.', '.', '#', '.', '.', '.', '#', '#'],
                vec!['#', '.', '#', '#', '#', '#', '.', '#']
            ]
        );

        parsed_tile.rotate_left();
        parsed_tile.rotate_left();
        parsed_tile.rotate_left();
        assert_eq!(parsed_tile.edges[0], "..##.#..#.".to_owned());
        assert_eq!(parsed_tile.edges[1], "...#.##..#".to_owned());
        assert_eq!(parsed_tile.edges[2], "..###..###".to_owned());
        assert_eq!(parsed_tile.edges[3], ".#####..#.".to_owned());

        assert_eq!(
            parsed_tile.data,
            vec![
                vec!['#', '.', '.', '#', '.', '.', '.', '.'],
                vec!['.', '.', '.', '#', '#', '.', '.', '#'],
                vec!['#', '#', '#', '.', '#', '.', '.', '.'],
                vec!['#', '.', '#', '#', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '#', '.', '#', '#'],
                vec!['#', '.', '#', '.', '#', '.', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '#', '.', '#'],
            ]
        );
    }

    #[test]
    fn it_flips_tile() {
        let tile_str = "Tile 2311:\n\
             ..##.#..#.\n\
             ##..#.....\n\
             #...##..#.\n\
             ####.#...#\n\
             ##.##.###.\n\
             ##...#.###\n\
             .#.#.#..##\n\
             ..#....#..\n\
             ###...#.#.\n\
             ..###..###";

        let mut parsed_tile = tile_str.parse::<Tile>().unwrap();

        assert_eq!(
            parsed_tile.data,
            vec![
                vec!['#', '.', '.', '#', '.', '.', '.', '.'],
                vec!['.', '.', '.', '#', '#', '.', '.', '#'],
                vec!['#', '#', '#', '.', '#', '.', '.', '.'],
                vec!['#', '.', '#', '#', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '#', '.', '#', '#'],
                vec!['#', '.', '#', '.', '#', '.', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '#', '.', '#'],
            ]
        );

        parsed_tile.flip();
        assert_eq!(parsed_tile.edges[0], ".#..#.##..".to_owned());
        assert_eq!(parsed_tile.edges[1], ".#####..#.".to_owned());
        assert_eq!(parsed_tile.edges[2], "###..###..".to_owned());
        assert_eq!(parsed_tile.edges[3], "...#.##..#".to_owned());

        assert_eq!(
            parsed_tile.data,
            vec![
                vec!['.', '.', '.', '.', '#', '.', '.', '#'],
                vec!['#', '.', '.', '#', '#', '.', '.', '.'],
                vec!['.', '.', '.', '#', '.', '#', '#', '#'],
                vec!['#', '#', '#', '.', '#', '#', '.', '#'],
                vec!['#', '#', '.', '#', '.', '.', '.', '#'],
                vec!['#', '.', '.', '#', '.', '#', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.'],
                vec!['#', '.', '#', '.', '.', '.', '#', '#']
            ]
        );

        parsed_tile.flip();
        assert_eq!(parsed_tile.edges[0], "..##.#..#.".to_owned());
        assert_eq!(parsed_tile.edges[1], "...#.##..#".to_owned());
        assert_eq!(parsed_tile.edges[2], "..###..###".to_owned());
        assert_eq!(parsed_tile.edges[3], ".#####..#.".to_owned());

        assert_eq!(
            parsed_tile.data,
            vec![
                vec!['#', '.', '.', '#', '.', '.', '.', '.'],
                vec!['.', '.', '.', '#', '#', '.', '.', '#'],
                vec!['#', '#', '#', '.', '#', '.', '.', '.'],
                vec!['#', '.', '#', '#', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '#', '.', '#', '#'],
                vec!['#', '.', '#', '.', '#', '.', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '#', '.', '#'],
            ]
        );
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day20.part_1(EXAMPLE_FILE), Ok(20899048083289));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day20.part_2(EXAMPLE_FILE), Ok(273));
    }
}
