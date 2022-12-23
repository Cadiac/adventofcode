use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::solution::{AocError, Solution};

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    multi::many1,
    IResult,
};

type Coords = (i64, i64);
type Bounds = HashMap<i64, Coords>;
type World = HashMap<Coords, Tile>;

const DIRECTIONS: [(i64, i64, Direction); 4] = [
    (1, 0, Direction::Right),
    (0, 1, Direction::Down),
    (-1, 0, Direction::Left),
    (0, -1, Direction::Up),
];

#[derive(Debug)]
enum Instruction {
    Move(i64),
    L,
    R,
}

fn parse_move(input: &str) -> IResult<&str, Instruction> {
    map(digit1, |s: &str| Instruction::Move(s.parse().unwrap()))(input)
}

fn parse_l(input: &str) -> IResult<&str, Instruction> {
    map(char('L'), |_| Instruction::L)(input)
}

fn parse_r(input: &str) -> IResult<&str, Instruction> {
    map(char('R'), |_| Instruction::R)(input)
}

fn parse_path(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((parse_move, parse_l, parse_r)))(input)
}

#[derive(Debug)]
enum Tile {
    Open,
    Solid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::L => match *self {
                Direction::Right => *self = Direction::Up,
                Direction::Down => *self = Direction::Right,
                Direction::Left => *self = Direction::Down,
                Direction::Up => *self = Direction::Left,
            },
            Instruction::R => match *self {
                Direction::Right => *self = Direction::Down,
                Direction::Down => *self = Direction::Left,
                Direction::Left => *self = Direction::Up,
                Direction::Up => *self = Direction::Right,
            },
            _ => {}
        }
    }

    fn to_password(&self) -> i64 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn to_delta(&self) -> (i64, i64) {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }
}

#[derive(Debug)]
struct Facet {
    x: Coords,
    y: Coords,
    neighbours: HashMap<Direction, (Coords, Direction)>,
}

#[derive(Clone, Eq, PartialEq)]
struct Search {
    current: Coords,
    direction: Direction,
    distance: i32,
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day22;

impl Day22 {
    fn parse(input: &str) -> Result<(World, Vec<Instruction>, Bounds, Bounds), AocError> {
        let mut world = HashMap::new();

        let (map, path) = input
            .split_once("\n\n")
            .ok_or_else(|| AocError::parse(input, "split"))?;
        let mut x_bounds: Bounds = HashMap::new();
        let mut y_bounds: Bounds = HashMap::new();

        for (y, row) in map.lines().enumerate() {
            for (x, tile) in row.chars().enumerate() {
                let (x, y) = (x as i64 + 1, y as i64 + 1);

                let tile = match tile {
                    '.' => Some(Tile::Open),
                    '#' => Some(Tile::Solid),
                    ' ' => None,
                    unexpected => return Err(AocError::parse(unexpected, "unexpected tile")),
                };

                if let Some(tile) = tile {
                    world.insert((x, y), tile);

                    let bounds = x_bounds.entry(y).or_insert((x, x));
                    bounds.1 = i64::max(bounds.1, x);

                    let bounds = y_bounds.entry(x).or_insert((y, y));
                    bounds.1 = i64::max(bounds.1, y);
                }
            }
        }

        let (_, path) = parse_path(path).map_err(|err| AocError::parse(path, err))?;

        Ok((world, path, x_bounds, y_bounds))
    }

    // Modified dijkstra style search that runs up to a max distance,
    // finding edges that should link to each other.
    fn find_neighbour(
        starting_direction: Direction,
        source: &Coords,
        facets: &HashMap<Coords, Facet>,
        max_dist: i32,
    ) -> Option<(Coords, Direction)> {
        // The direction we arrive from matters
        let mut dist: HashMap<(Coords, Direction), i32> = HashMap::new();
        let mut heap: BinaryHeap<Search> = BinaryHeap::new();

        let bounds = (-1, 4);

        let delta = starting_direction.to_delta();
        let start = (source.0 + delta.0, source.1 + delta.1);

        for (_, _, direction) in DIRECTIONS {
            *dist.entry((*source, direction)).or_insert(0) = 0;
            *dist.entry((start, direction)).or_insert(0) = 0;    
        }
        
        heap.push(Search {
            distance: 0,
            current: start,
            direction: starting_direction,
        });

        while let Some(Search {
            current,
            distance,
            direction,
        }) = heap.pop()
        {
            if distance > max_dist {
                continue;
            }

            // We've already found a shorter way
            if distance > *dist.get(&(current, direction)).unwrap_or(&i32::MAX) {
                continue;
            }

            if current != *source {
                if let Some(neighbour) = facets.get(&(current)) {
                    // The neighbour slot from this direction has to be empty
                    if neighbour.neighbours.get(&direction.opposite()).is_some() {
                        continue;
                    }

                    // TODO: Consider the original direction, can't ever have two adjacent neighbours with the same direction?
                    // Or don't care? Doesn't seem to matter, but could maybe on "'--," shaped cubes?

                    // Can only have the same tile once as a neighbour
                    if !neighbour
                        .neighbours
                        .values()
                        .any(|(existing_neighbour, _)| *existing_neighbour == *source)
                    {
                        // Found a suitable neighbour
                        return Some((current, direction.opposite()));
                    }
                }
            }

            for (dx, dy, dir) in DIRECTIONS.iter() {
                let next_pos = (current.0 + dx, current.1 + dy);
                if next_pos.0 >= bounds.0
                    && next_pos.0 <= bounds.1
                    && next_pos.1 >= bounds.0
                    && next_pos.1 <= bounds.1
                {
                    let next = Search {
                        distance: distance + 1,
                        current: next_pos,
                        direction: *dir,
                    };

                    // TODO: We care about the direction we're travelling to,
                    // So this doesn't quite work. Keep dist map of pos + direction?
                    if next.distance < *dist.get(&(next_pos, *dir)).unwrap_or(&i32::MAX) {
                        *dist.entry((next_pos, *dir)).or_insert(0) = next.distance;
                        heap.push(next);
                    }
                }
            }
        }

        None
    }
}

impl Solution for Day22 {
    type F = i64;
    type S = i64;

    fn name(&self) -> &'static str {
        "Day 22"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day22.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let (world, path, x_bounds, y_bounds) = Day22::parse(input)?;

        let mut direction = Direction::Right;
        let mut position: Coords = (x_bounds.get(&1).unwrap().0, 1);

        for instruction in path {
            println!(
                "Position: {position:?}, direction: {direction:?}, instruction: {instruction:?}"
            );
            match instruction {
                Instruction::L | Instruction::R => direction.turn(instruction),
                Instruction::Move(steps) => {
                    for _step in 0..steps {
                        let delta = match direction {
                            Direction::Right => (1, 0),
                            Direction::Down => (0, 1),
                            Direction::Left => (-1, 0),
                            Direction::Up => (0, -1),
                        };

                        match world.get(&(position.0 + delta.0, position.1 + delta.1)) {
                            Some(Tile::Open) => {
                                position = (position.0 + delta.0, position.1 + delta.1)
                            }
                            Some(Tile::Solid) => {
                                println!("Hit the wall");
                                break;
                            }
                            None => {
                                // Wrap around
                                println!("Wrap around");
                                let wrap_target = match direction {
                                    Direction::Right => {
                                        (x_bounds.get(&position.1).unwrap().0, position.1)
                                    }
                                    Direction::Down => {
                                        (position.0, y_bounds.get(&position.0).unwrap().0)
                                    }
                                    Direction::Left => {
                                        (x_bounds.get(&position.1).unwrap().1, position.1)
                                    }
                                    Direction::Up => {
                                        (position.0, y_bounds.get(&position.0).unwrap().1)
                                    }
                                };

                                match world.get(&wrap_target) {
                                    Some(Tile::Open) => position = wrap_target,
                                    Some(Tile::Solid) => {
                                        println!("Hit the wall at wrap");
                                        break;
                                    }
                                    None => unreachable!(),
                                };
                            }
                        };

                        println!("Step: {position:?}");
                    }
                }
            }
        }

        println!("Final position was {position:?} direction {direction:?}");

        Ok(1000 * position.1 + 4 * position.0 + direction.to_password())
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let (world, path, x_bounds, y_bounds) = Day22::parse(input)?;

        let mut direction = Direction::Right;
        let mut position: Coords = (x_bounds.get(&1).unwrap().0, 1);

        let cube_size = x_bounds
            .values()
            .map(|(min, max)| max - min + 1)
            .min()
            .unwrap();

        // Is this a horizontal or vertical pattern (4 height/width)?
        // TODO: Assume vertical

        // Sample points from the corner of each possible piece to see if the pattern is there
        let mut facets: HashMap<Coords, Facet> = HashMap::new();

        for id in 0..16 {
            let x = id % 4;
            let y = id / 4;

            if let Some(_) = world.get(&(x * cube_size + 1, y * cube_size + 1)) {
                facets.insert(
                    (x, y),
                    Facet {
                        x: (x * cube_size + 1, x * cube_size + 1 + cube_size),
                        y: (y * cube_size + 1, y * cube_size + 1 + cube_size),
                        neighbours: HashMap::new(),
                    },
                );
            };
        }

        // Then collect the neighbours, increasing the search distance
        // from zero to up until all the neighbours are found
        // I guess the distance can never be more than 16?
        for distance in 1..16 {
            let keys: Vec<_> = facets.keys().cloned().collect();
            for source in keys {
                let current_source = facets.get(&source).unwrap();
                let empty_directions: Vec<_> = [
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                ]
                .iter()
                .filter(|direction| !current_source.neighbours.contains_key(direction))
                .collect();

                for starting_direction in empty_directions.into_iter() {
                    if let Some((neighbour, arrived_to_direction)) =
                        Day22::find_neighbour(*starting_direction, &source, &facets, distance)
                    {
                        println!("Neighbour for source {source:?} -> {starting_direction:?}: {neighbour:?} <- {arrived_to_direction:?}, distance {distance}");

                        facets
                            .get_mut(&source)
                            .unwrap()
                            .neighbours
                            .insert(*starting_direction, (neighbour, arrived_to_direction));
                        facets
                            .get_mut(&neighbour)
                            .unwrap()
                            .neighbours
                            .insert(arrived_to_direction, (source, *starting_direction));
                    }
                }
            }
        }

        println!("Facets: {facets:?}");

        // TODO: Using the facets travel around and wrap with the neighbour rules

        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "        ...#",
        "        .#..",
        "        #...",
        "        ....",
        "...#.......#",
        "........#...",
        "..#....#....",
        "..........#.",
        "        ...#....",
        "        .....#..",
        "        .#......",
        "        ......#.",
        "",
        "10R5L5R10L4R5L5",
    ];

    #[test]
    fn it_solves_part1() {
        assert_eq!(Day22.part_1(&INPUT.join("\n")), Ok(6032));
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(Day22.part_2(&INPUT.join("\n")), Ok(5031));
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(Day22.part_2(Day22.default_input()), Ok(5031));
    }
}
