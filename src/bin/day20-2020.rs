const INPUT_FILE: &str = include_str!("../../inputs/day20-2020.txt");
const EXAMPLE_FILE: &str = include_str!("../../inputs/day20-2020-example.txt");

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

type Puzzle = HashMap<(usize, usize), Tile>;

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    edges: [String; 4],
    data: Vec<Vec<char>>,
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

        Ok(Tile { id, edges, data })
    }
}

impl Tile {
    // counter clockwise
    fn rotate(&mut self, turns: usize) {
        for _turn in 0..turns {
            self.edges = [
                self.edges[1].clone(),
                self.edges[2].chars().rev().collect(),
                self.edges[3].clone(),
                self.edges[0].chars().rev().collect(),
            ];

            // TODO: how to avoid cloning?
            self.data = rotate_left(self.data.clone());
        }
    }

    // flip vertically
    fn flip(&mut self) {
        self.edges = [
            self.edges[0].chars().rev().collect(),
            self.edges[3].clone(),
            self.edges[2].chars().rev().collect(),
            self.edges[1].clone(),
        ];

        // TODO: how to avoid cloning?
        self.data = flip(self.data.clone());
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile_str| tile_str.parse::<Tile>().unwrap())
        .collect()
}

fn is_valid_tile(current_board: &Puzzle, tile: &Tile, cursor: (usize, usize)) -> bool {
    // above
    if cursor.1 > 0 {
        if let Some(neighbour) = current_board.get(&(cursor.0, cursor.1 - 1)) {
            if tile.edges[0] != neighbour.edges[2] {
                return false;
            }
        }
    }

    // below
    if let Some(neighbour) = current_board.get(&(cursor.0, cursor.1 + 1)) {
        if tile.edges[2] != neighbour.edges[0] {
            return false;
        }
    }

    // to right
    if let Some(neighbour) = current_board.get(&(cursor.0 + 1, cursor.1)) {
        if tile.edges[1] != neighbour.edges[3] {
            return false;
        }
    }

    // to left
    if cursor.0 > 0 {
        if let Some(neighbour) = current_board.get(&(cursor.0 - 1, cursor.1)) {
            if tile.edges[3] != neighbour.edges[1] {
                return false;
            }
        }
    }

    return true;
}

fn find_next_cursor(size: usize, cursor: (usize, usize)) -> (usize, usize) {
    if cursor.0 + 1 < size {
        return (cursor.0 + 1, cursor.1);
    }
    // This will go over `size` on y-axis, but it should never actually be used like that
    (0, cursor.1 + 1)
}

// TODO: Add cache
fn build_puzzle_recursive(
    size: usize,
    remaining_tiles: Vec<Tile>,
    current_board: Puzzle,
    cursor: (usize, usize),
) -> Option<Puzzle> {
    // Done, we managed to place all tiles somewhere
    if remaining_tiles.is_empty() {
        return Some(current_board);
    }

    // Otherwise try to fit some tile to current cursor
    let next_cursor = find_next_cursor(size, cursor);

    for tile in remaining_tiles.iter() {
        let rest: Vec<Tile> = remaining_tiles
            .clone()
            .into_iter()
            .filter(|remaining| remaining.id != tile.id)
            .collect();

        // Try both sides
        let mut flipped_tile = tile.clone();
        flipped_tile.flip();

        for tile_side in [tile, &flipped_tile] {
            for rotation in 0..4 {
                let mut rotated_tile = tile_side.clone();
                rotated_tile.rotate(rotation);
                if is_valid_tile(&current_board, &rotated_tile, cursor) {
                    let mut next_board = current_board.clone();
                    next_board.insert(cursor, rotated_tile);
                    if let Some(solution) =
                        build_puzzle_recursive(size, rest.clone(), next_board, next_cursor)
                    {
                        return Some(solution);
                    }
                }
            }
        }
    }

    // No tile fit on board, abort this and go back to trying something else
    return None;
}

fn check_for_sea_monsters(data: &mut Vec<Vec<(char, bool)>>) {
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    let sea_monster_coordinates: Vec<(usize, usize)> = vec![
        (18, 0),
        (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
        (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2)
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

            let is_sea_monster = sea_monster_coordinates
                .iter()
                .all(|(window_x, window_y)| data[y + window_y][x + window_x].0 == '#');

            if is_sea_monster {
                // We found a sea monster!
                println!("Found a sea monster, starting from ({}, {})", x, y);

                for (window_x, window_y) in sea_monster_coordinates.iter() {
                    data[y + window_y][x + window_x].1 = true;
                }

                // Debug prints
                for yy in 0..3 {
                    for xx in 0..20 {
                        if data[y + yy][x + xx].1 {
                            print!("{}", 'O');
                        } else {
                            print!("{}", data[y + yy][x + xx].0);
                        }
                    }
                    print!("\n");
                }
                print!("\n");
            }
        }
    }
}

fn part_1(input: &str) -> u64 {
    let tiles = parse_tiles(input);

    let size = (tiles.len() as f64).sqrt() as usize;

    println!("Solving {:?} x {:?} puzzle", size, size);

    if let Some(puzzle) = build_puzzle_recursive(size, tiles, HashMap::new(), (0, 0)) {
        println!("Found solution to puzzle!");
        return puzzle.get(&(0, 0)).unwrap().id
            * puzzle.get(&(0, size - 1)).unwrap().id
            * puzzle.get(&(size - 1, size - 1)).unwrap().id
            * puzzle.get(&(size - 1, 0)).unwrap().id;
    }

    panic!("[ERROR]: No solution to Part 1!");
}

fn part_2(input: &str) -> usize {
    let tiles = parse_tiles(input);

    let size = (tiles.len() as f64).sqrt() as usize;

    println!("Solving {:?} x {:?} puzzle", size, size);

    if let Some(puzzle) = build_puzzle_recursive(size, tiles, HashMap::new(), (0, 0)) {
        println!("Found solution to puzzle!");
        let mut full_data: Vec<Vec<(char, bool)>> = vec![];

        for y in 0..size {
            let mut rows = vec![vec![]; 8];

            for x in 0..size {
                let tile = puzzle.get(&(x, y)).unwrap().clone();

                print!(" {} ", tile.id);

                for row in 0..8 {
                    rows[row]
                        .append(&mut tile.data[row].iter().cloned().map(|c| (c, false)).collect());
                }
            }

            print!("\n");

            full_data.append(&mut rows);
        }

        print!("\n");
        println!("Gathered total {} rows", full_data.len());

        for row in full_data.iter() {
            println!("{}", row.iter().map(|(c, _)| c).collect::<String>());
        }

        print!("\n");

        for _rotation in 0..4 {
            check_for_sea_monsters(&mut full_data);
            full_data = rotate_left(full_data.clone());
        }

        full_data = flip(full_data.clone());

        for _rotation in 0..4 {
            check_for_sea_monsters(&mut full_data);
            full_data = rotate_left(full_data.clone());
        }

        let not_monsters: usize = full_data
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|(c, is_monster)| *c == '#' && !is_monster)
                    .count()
            })
            .sum();

        return not_monsters;
    }

    panic!("[ERROR]: No solution to Part 2!");
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let tiles = parse_tiles(EXAMPLE_FILE);
        assert_eq!(tiles.len(), 9);
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

        parsed_tile.rotate(1);
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

        parsed_tile.rotate(3);
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
        assert_eq!(part_1(EXAMPLE_FILE), 20899048083289);
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(part_2(EXAMPLE_FILE), 273);
    }
}
