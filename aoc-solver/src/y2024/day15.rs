use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub type Coords = (isize, isize);
pub type Grid = HashMap<Coords, Tile>;

#[derive(Debug, PartialEq)]
pub enum Tile {
    Box,
    WideBox(Coords),
    Wall,
}

pub fn parse(input: &str, is_large: bool) -> Result<(Grid, Vec<Coords>, Coords), AocError> {
    let (grid_input, instructions_input) = input
        .trim()
        .split_once("\n\n")
        .ok_or_else(|| AocError::parse(input, "Missing grid and instructions sections"))?;

    let mut start = (0, 0);
    let mut grid = HashMap::new();

    for (y, line) in grid_input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            if is_large {
                let left = (x as isize * 2, y as isize);
                let right = (x as isize * 2 + 1, y as isize);

                match tile {
                    'O' => {
                        grid.insert(left, Tile::WideBox(right));
                        grid.insert(right, Tile::WideBox(left));
                    }
                    '#' => {
                        grid.insert(left, Tile::Wall);
                        grid.insert(right, Tile::Wall);
                    }
                    '@' => {
                        start = left;
                    }
                    _ => {}
                }
            } else {
                match tile {
                    'O' => {
                        grid.insert((x as isize, y as isize), Tile::Box);
                    }
                    '#' => {
                        grid.insert((x as isize, y as isize), Tile::Wall);
                    }
                    '@' => {
                        start = (x as isize, y as isize);
                    }
                    _ => {}
                }
            }
        }
    }
    let instructions = instructions_input
        .lines()
        .flat_map(|line| {
            line.chars().map(|instruction| match instruction {
                '^' => Ok((0, -1)),
                '>' => Ok((1, 0)),
                'v' => Ok((0, 1)),
                '<' => Ok((-1, 0)),
                _ => Err(AocError::parse(instruction, "Invalid move instruction")),
            })
        })
        .try_collect()?;

    Ok((grid, instructions, start))
}

pub fn move_robot(robot: &mut Coords, (dx, dy): Coords, grid: &mut Grid) {
    let mut stack = vec![(robot.0 + dx, robot.1 + dy)];
    let mut to_move = HashSet::new();

    while let Some(current) = stack.pop() {
        let tile = grid.get(&current);

        match tile {
            Some(Tile::Wall) => return,
            Some(Tile::Box) => {
                stack.push((current.0 + dx, current.1 + dy));
                to_move.insert(current);
            }
            Some(Tile::WideBox(linked)) if dx != 0 => {
                stack.push((linked.0 + dx, linked.1 + dy));

                to_move.insert(current);
                to_move.insert(*linked);
            }
            Some(Tile::WideBox(linked)) => {
                stack.push((current.0 + dx, current.1 + dy));
                stack.push((linked.0 + dx, linked.1 + dy));

                to_move.insert(current);
                to_move.insert(*linked);
            }
            None => {}
        }
    }

    let moved: Vec<_> = to_move
        .iter()
        .filter_map(|pos| {
            grid.remove(pos).map(|moved| {
                let tile = match moved {
                    Tile::Box => Tile::Box,
                    Tile::WideBox(linked) => Tile::WideBox((linked.0 + dx, linked.1 + dy)),
                    _ => unreachable!(),
                };

                (pos, tile)
            })
        })
        .collect();

    for (prev, tile) in moved {
        grid.insert((prev.0 + dx, prev.1 + dy), tile);
    }

    *robot = (robot.0 + dx, robot.1 + dy);
}

fn gps_coordinate((pos, tile): (&Coords, &Tile)) -> Option<u64> {
    match tile {
        Tile::Box => Some((pos.1 * 100 + pos.0) as u64),
        Tile::WideBox(linked) if pos.0 < linked.0 => Some((pos.1 * 100 + pos.0) as u64),
        _ => None,
    }
}

pub struct Day15;
impl Solution for Day15 {
    type A = u64;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day15.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let (mut grid, instructions, mut robot) = parse(input, false)?;

        for instruction in instructions {
            move_robot(&mut robot, instruction, &mut grid);
        }

        let gps_sum = grid.iter().filter_map(gps_coordinate).sum();

        Ok(gps_sum)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let (mut grid, instructions, mut robot) = parse(input, true)?;

        for instruction in instructions {
            move_robot(&mut robot, instruction, &mut grid);
        }

        let gps_sum = grid.iter().filter_map(gps_coordinate).sum();

        Ok(gps_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day15.part_1(
                "##########\n\
                 #..O..O.O#\n\
                 #......O.#\n\
                 #.OO..O.O#\n\
                 #..O@..O.#\n\
                 #O#..O...#\n\
                 #O..O..O.#\n\
                 #.OO.O.OO#\n\
                 #....O...#\n\
                 ##########\n\
                 \n\
                 <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                 vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                 ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                 <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                 ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                 ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                 >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                 <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                 ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                 v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            Ok(10092)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day15.part_1(
                "########\n\
                 #..O.O.#\n\
                 ##@.O..#\n\
                 #...O..#\n\
                 #.#.O..#\n\
                 #...O..#\n\
                 #......#\n\
                 ########\n\
                 \n\
                 <^^>>>vv<v>>v<<"
            ),
            Ok(2028)
        )
    }

    #[test]
    fn it_moves_robot_1() {
        let (mut grid, _, mut robot) = parse(
            "########\n\
             #..O.O.#\n\
             ##@.O..#\n\
             #...O..#\n\
             #.#.O..#\n\
             #...O..#\n\
             #......#\n\
             ########\n\
             \n<",
            false,
        )
        .unwrap();

        move_robot(&mut robot, (-1, 0), &mut grid);
        assert_eq!(robot, (2, 2));
    }

    #[test]
    fn it_moves_robot_2() {
        let (mut grid, _, mut robot) = parse(
            "########\n\
             #.OO.O.#\n\
             ##@.O..#\n\
             #...O..#\n\
             #.#.O..#\n\
             #...O..#\n\
             #......#\n\
             ########\n\
             \n<",
            false,
        )
        .unwrap();

        move_robot(&mut robot, (0, -1), &mut grid);
        assert_eq!(robot, (2, 2));
        assert_eq!(grid.get(&(2, 1)), Some(Tile::Box).as_ref());
    }

    #[test]
    fn it_moves_robot_3() {
        let (mut grid, _, mut robot) = parse(
            "########\n\
             #.OO.O.#\n\
             ##@OO..#\n\
             #...O..#\n\
             #.#.O..#\n\
             #...O..#\n\
             #......#\n\
             ########\n\
             \n<",
            false,
        )
        .unwrap();

        move_robot(&mut robot, (1, 0), &mut grid);
        assert_eq!(robot, (3, 2));
        assert_eq!(grid.get(&(3, 2)), None);
        assert_eq!(grid.get(&(4, 2)), Some(Tile::Box).as_ref());
        assert_eq!(grid.get(&(5, 2)), Some(Tile::Box).as_ref());
    }

    #[test]
    fn it_calculates_gps_box() {
        assert_eq!(gps_coordinate((&(4, 1), &Tile::Box)), Some(104))
    }

    #[test]
    fn it_calculates_gps_wall() {
        assert_eq!(gps_coordinate((&(4, 1), &Tile::Wall)), None)
    }

    #[test]
    fn it_calculates_gps_wide_left() {
        assert_eq!(gps_coordinate((&(4, 1), &Tile::WideBox((5, 1)))), Some(104))
    }

    #[test]
    fn it_calculates_gps_wide_right() {
        assert_eq!(gps_coordinate((&(4, 1), &Tile::WideBox((3, 1)))), None)
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day15.part_2(
                "#######\n\
                 #...#.#\n\
                 #.....#\n\
                 #..OO@#\n\
                 #..O..#\n\
                 #.....#\n\
                 #######\n\
                 \n\
                 <vv<<^^<<^^"
            ),
            Ok(618)
        )
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            Day15.part_2(
                "##########\n\
                 #..O..O.O#\n\
                 #......O.#\n\
                 #.OO..O.O#\n\
                 #..O@..O.#\n\
                 #O#..O...#\n\
                 #O..O..O.#\n\
                 #.OO.O.OO#\n\
                 #....O...#\n\
                 ##########\n\
                 \n\
                 <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
                 vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
                 ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
                 <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
                 ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
                 ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
                 >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
                 <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
                 ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
                 v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            Ok(9021)
        );
    }
}
