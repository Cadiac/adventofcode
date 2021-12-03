const INPUT_FILE: &str = include_str!("../../inputs/day20-2020.txt");
const EXAMPLE_FILE: &str = include_str!("../../inputs/day20-2020-example.txt");

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Tile {
    id: i64,
    edges: Vec<String>,
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
                id_part[0].parse::<i64>().unwrap()
            }
            None => 0,
        };

        let mut top_edge: String = String::new();
        let mut right_edge: Vec<char> = Vec::new();
        let mut bottom_edge: String = String::new();
        let mut left_edge: Vec<char> = Vec::new();

        for (y, line) in lines.enumerate() {
            if y == 0 {
                top_edge = String::from(line);
            } else if y == 9 {
                bottom_edge = String::from(line);
            }

            for (x, pixel) in line.chars().enumerate() {
                if x == 0 {
                    left_edge.push(pixel);
                } else if x == 9 {
                    right_edge.push(pixel);
                }
            }
        }

        let edges: Vec<String> = vec![
            top_edge,
            right_edge.iter().collect(),
            bottom_edge,
            left_edge.iter().collect(),
        ];

        Ok(Tile { id, edges })
    }
}

impl Tile {
    fn rotate(&mut self, turns: usize) {
        self.edges.rotate_right(turns);
    }

    fn flip() {
        unimplemented!();
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile_str| tile_str.parse::<Tile>().unwrap())
        .collect()
}

fn part_1(input: &str) -> i64 {
    let tiles = parse_tiles(input);
    unimplemented!();
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_tile() {
        let tile_str = 
           "Tile 2311:\n\
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
    }

    #[test]
    fn it_parses_tiles() {
        let tiles = parse_tiles(EXAMPLE_FILE);
        assert_eq!(tiles.len(), 9);
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(part_1(EXAMPLE_FILE), 20899048083289);
    }
}
