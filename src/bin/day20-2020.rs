const INPUT_FILE: &str = include_str!("../../inputs/day20-2020.txt");
const EXAMPLE_FILE: &str = include_str!("../../inputs/day20-2020-example.txt");

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
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
            bottom_edge.chars().rev().collect(),
            left_edge.iter().rev().collect(),
        ];

        Ok(Tile { id, edges })
    }
}

impl Tile {
    fn rotate(&mut self, turns: usize) {
        self.edges.rotate_right(turns);
    }

    fn flip(&mut self) {
        self.edges = self
            .edges
            .iter()
            .map(|edge| edge.chars().rev().collect())
            .collect();
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

    let dimensions = (tiles.len() as f64).sqrt() as usize;

    let current_tile = tiles[0].clone();

    loop {
        // Pick one of tile tiles and use that as the corner?
        // Then start looking for pieces that fit to its two edges? And move to edge[1]?
        for tile in tiles.iter() {
            if  current_tile.edges[0] == tile.edges[2].chars().rev().collect::<String>() &&
                current_tile.edges[1] == tile.edges[3].chars().rev().collect::<String>() &&
                current_tile.edges[2] == tile.edges[0].chars().rev().collect::<String>() &&
                current_tile.edges[3] == tile.edges[1].chars().rev().collect::<String>() {
                
            } else {
                // Rotate, flip?
            }
        }

    }

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
        assert_eq!(parsed_tile.edges[0], "..##.#..#.".to_owned());
        assert_eq!(parsed_tile.edges[1], "...#.##..#".to_owned());
        assert_eq!(parsed_tile.edges[2], "###..###..".to_owned());
        assert_eq!(parsed_tile.edges[3], ".#..#####.".to_owned());
    }

    #[test]
    fn it_parses_tiles() {
        let tiles = parse_tiles(EXAMPLE_FILE);
        assert_eq!(tiles.len(), 9);
    }

    #[test]
    fn it_rotates_tile() {
        let tiles = parse_tiles(EXAMPLE_FILE);

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

        let mut parsed_tile = tile_str.parse::<Tile>().unwrap();
        parsed_tile.rotate(1);
        assert_eq!(parsed_tile.edges[0], ".#..#####.".to_owned());
        assert_eq!(parsed_tile.edges[1], "..##.#..#.".to_owned());
        assert_eq!(parsed_tile.edges[2], "...#.##..#".to_owned());
        assert_eq!(parsed_tile.edges[3], "###..###..".to_owned());
        parsed_tile.rotate(3);
        assert_eq!(parsed_tile.edges[0], "..##.#..#.".to_owned());
        assert_eq!(parsed_tile.edges[1], "...#.##..#".to_owned());
        assert_eq!(parsed_tile.edges[2], "###..###..".to_owned());
        assert_eq!(parsed_tile.edges[3], ".#..#####.".to_owned());
    }

    #[test]
    fn it_flips_tile() {
        let tiles = parse_tiles(EXAMPLE_FILE);

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

        let mut parsed_tile = tile_str.parse::<Tile>().unwrap();
        parsed_tile.flip();
        assert_eq!(parsed_tile.edges[0], ".#..#.##..".to_owned());
        assert_eq!(parsed_tile.edges[1], "#..##.#...".to_owned());
        assert_eq!(parsed_tile.edges[2], "..###..###".to_owned());
        assert_eq!(parsed_tile.edges[3], ".#####..#.".to_owned());
        parsed_tile.flip();
        assert_eq!(parsed_tile.edges[0], "..##.#..#.".to_owned());
        assert_eq!(parsed_tile.edges[1], "...#.##..#".to_owned());
        assert_eq!(parsed_tile.edges[2], "###..###..".to_owned());
        assert_eq!(parsed_tile.edges[3], ".#..#####.".to_owned());
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(part_1(EXAMPLE_FILE), 20899048083289);
    }
}
