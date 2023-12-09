use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::solution::{AocError, Solution};

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    multi::many1,
    IResult,
};

pub type Coords = (i64, i64);
pub type Bounds = HashMap<i64, Coords>;
pub type World = HashMap<Coords, Tile>;

#[derive(Debug)]
pub enum Instruction {
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

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Open,
    Solid,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Down,
    Direction::Left,
    Direction::Up,
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_left(&mut self) {
        match *self {
            Direction::Right => *self = Direction::Up,
            Direction::Down => *self = Direction::Right,
            Direction::Left => *self = Direction::Down,
            Direction::Up => *self = Direction::Left,
        }
    }

    fn turn_right(&mut self) {
        match *self {
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
            Direction::Up => *self = Direction::Right,
        }
    }

    fn turn(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::L => self.turn_left(),
            Instruction::R => self.turn_right(),
            _ => {}
        }
    }

    fn to_password(self) -> i64 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn to_delta(self) -> Coords {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        }
    }

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }
}

#[derive(Debug)]
pub struct Facet {
    pub tiles: HashMap<Coords, (Tile, Coords)>,
    pub neighbours: HashMap<Direction, (Coords, Direction)>,
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
    pub fn parse(input: &str) -> Result<(World, Vec<Instruction>, Bounds, Bounds), AocError> {
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

    pub fn fold_cube(facets: &mut HashMap<Coords, Facet>) {
        let mut distance = 0;

        // Search for folded edges by slowly increasing the search distance one by one.
        // This causes the inner neighbours to be found first, and the far away opposite sides last.
        while facets.iter().any(|(_, facet)| facet.neighbours.len() != 4) {
            distance += 1;

            let keys: Vec<_> = facets.keys().cloned().collect();
            for source in keys {
                for starting_direction in DIRECTIONS.iter() {
                    if facets.get(&source).unwrap().neighbours.contains_key(starting_direction) {
                        continue;
                    }

                    if let Some((neighbour, arrived_to_direction)) =
                        Day22::find_neighbour(*starting_direction, &source, facets, distance)
                    {
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
    }

    // Modified Dijkstra style search algorithm that runs up to a max distance,
    // finding edges that should link to each other in the folded cube.
    fn find_neighbour(
        starting_direction: Direction,
        source: &Coords,
        facets: &HashMap<Coords, Facet>,
        max_dist: i32,
    ) -> Option<(Coords, Direction)> {
        // The arrival direction matters, so collect distances by those
        let mut dist: HashMap<(Coords, Direction), i32> = HashMap::new();
        let mut heap: BinaryHeap<Search> = BinaryHeap::new();

        let delta = starting_direction.to_delta();
        let start = (source.0 + delta.0, source.1 + delta.1);

        // Consider source and start as starting points from all directions
        for direction in DIRECTIONS {
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

            // A shorter path has already been found
            if distance > *dist.get(&(current, direction)).unwrap_or(&i32::MAX) {
                continue;
            }

            if current != *source {
                if let Some(neighbour) = facets.get(&(current)) {
                    // The neighbour slot from this direction has to be empty
                    if neighbour.neighbours.get(&direction.reverse()).is_some() {
                        continue;
                    }

                    // NOTE: This should perhaps consider the original direction,
                    // as there can never be two adjacent neighbours with the same direction?
                    // Ignoring this doesn't seem to matter in both my input and the examples,
                    // but it could maybe matter on "S" shaped patterns?

                    // No tile can have the same tile as neighbour twice from different directions
                    if !neighbour
                        .neighbours
                        .values()
                        .any(|(existing_neighbour, _)| *existing_neighbour == *source)
                    {
                        // Suitable neighbour found
                        return Some((current, direction.reverse()));
                    }
                }
            }

            for next_dir in DIRECTIONS.iter() {
                let (dx, dy) = next_dir.to_delta();
                let next_pos = (current.0 + dx, current.1 + dy);
                let bounds = (-1, 4);
                let is_within_bounds = next_pos.0 >= bounds.0
                    && next_pos.0 <= bounds.1
                    && next_pos.1 >= bounds.0
                    && next_pos.1 <= bounds.1;

                if is_within_bounds {
                    let next = Search {
                        distance: distance + 1,
                        current: next_pos,
                        direction: *next_dir,
                    };

                    if next.distance < *dist.get(&(next_pos, *next_dir)).unwrap_or(&i32::MAX) {
                        *dist.entry((next_pos, *next_dir)).or_insert(0) = next.distance;
                        heap.push(next);
                    }
                }
            }
        }

        None
    }

    pub fn find_leftmost_open(
        world: &HashMap<Coords, Tile>,
        row_dimensions: &HashMap<i64, Coords>,
    ) -> Result<Coords, AocError> {
        let (start, end) = row_dimensions.get(&1).unwrap_or(&(0, 0));

        for x in *start..=*end {
            if let Some(Tile::Open) = world.get(&(x, 1)) {
                return Ok((x, 1));
            }
        }

        Err(AocError::logic("no leftmost open tile"))
    }

    pub fn find_cube_facets(size: i64, world: HashMap<Coords, Tile>, facets: &mut HashMap<Coords, Facet>, leftmost_open: Coords) -> Coords {
        let mut starting_facet: Coords = (-1, -1);

        // Sample points from the corner of each possible piece to see if the pattern has facet there
        for id in 0..16 {
            let x = id % 4;
            let y = id / 4;
    
            let x_world = x * size + 1;
            let y_world = y * size + 1;
    
            if world.contains_key(&(x_world, y_world)) {
                // Store the original coordinates and tiles by facet coordinates
                let mut tiles = HashMap::new();
                for y_facet in 0..size {
                    for x_facet in 0..size {
                        let world_coords = (x_world + x_facet, y_world + y_facet);
                        let tile = world.get(&world_coords).unwrap();
    
                        tiles.insert((x_facet, y_facet), (*tile, world_coords));
                    }
                }
    
                facets.insert(
                    (x, y),
                    Facet {
                        tiles,
                        neighbours: HashMap::new(),
                    },
                );
    
                // Find the starting position's facet while doing this
                if x_world == leftmost_open.0 && y_world == leftmost_open.1 {
                    starting_facet = (x, y)
                }
            };
        }
        starting_facet
    }    

    #[rustfmt::skip]
    fn translate_position(direction: Direction, arrival_direction: &Direction, max: i64, position: Coords) -> Coords {
        match (direction, arrival_direction) {
            (Direction::Right, Direction::Left) => (0, position.1),
            (Direction::Left, Direction::Right) => (max, position.1),

            (Direction::Down, Direction::Up) => (position.0, 0),
            (Direction::Up, Direction::Down) => (position.0, max),

            (Direction::Down, Direction::Right) => (max, position.0),
            (Direction::Right, Direction::Down) => (position.1, max),

            (Direction::Down, Direction::Left) => (0, max - position.0),
            (Direction::Left, Direction::Down) => (max - position.1, max),
            
            (Direction::Up, Direction::Right) => (max, max - position.0),
            (Direction::Right, Direction::Up) => (max - position.1, 0),
            
            (Direction::Up, Direction::Left) => (0, position.0),
            (Direction::Left, Direction::Up) => (position.1, 0),

            (Direction::Down, Direction::Down) | (Direction::Up, Direction::Up) => (max - position.0, position.1),
            (Direction::Right, Direction::Right) | (Direction::Left, Direction::Left) => (position.0, max - position.1),
        }
    }

    pub fn follow_path(starting_facet: Coords, path: Vec<Instruction>, facets: &HashMap<Coords, Facet>, size: i64) -> Result<(Coords, Coords, Direction), AocError> {
        let mut facet = starting_facet;
        let mut position = (0, 0);
        let mut direction = Direction::Right;
        for instruction in path {
            match instruction {
                Instruction::L | Instruction::R => direction.turn(instruction),
                Instruction::Move(steps) => {
                    for _step in 0..steps {
                        let delta = direction.to_delta();
                        let target_pos = (position.0 + delta.0, position.1 + delta.1);
                        let current_facet = facets.get(&facet).unwrap();
    
                        match current_facet.tiles.get(&target_pos) {
                            Some((Tile::Open, _)) => position = target_pos,
                            Some((Tile::Solid, _)) => break,
                            None => {
                                // Jump to a neighbour facet in that direction
                                match current_facet.neighbours.get(&direction) {
                                    Some((next_facet, arrival_direction)) => {
                                        let arrival_position = Day22::translate_position(
                                            direction,
                                            arrival_direction,
                                            size - 1,
                                            position,
                                        );
    
                                        // Check if there would be an immediate collision
                                        match facets
                                            .get(next_facet)
                                            .and_then(|f| f.tiles.get(&arrival_position))
                                        {
                                            Some((Tile::Open, _)) => {
                                                facet = *next_facet;
                                                position = arrival_position;
                                                direction = arrival_direction.reverse();
                                            }
                                            Some((Tile::Solid, _)) => break,
                                            None => {
                                                return Err(AocError::logic(
                                                    "missing tile at wrap to neighbour",
                                                ))
                                            }
                                        }
                                    }
                                    None => {
                                        return Err(AocError::logic(
                                            "missing facet at wrap to neighbour",
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok((facet, position, direction))
    }    
}

impl Solution for Day22 {
    type A = i64;
    type B = i64;


    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day22.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let (world, path, rows, columns) = Day22::parse(input)?;

        let mut direction = Direction::Right;
        let mut position: Coords = Day22::find_leftmost_open(&world, &rows)?;

        for instruction in path {
            match instruction {
                Instruction::L | Instruction::R => direction.turn(instruction),
                Instruction::Move(steps) => {
                    for _step in 0..steps {
                        let delta = direction.to_delta();
                        let target_pos = (position.0 + delta.0, position.1 + delta.1);

                        match world.get(&target_pos) {
                            Some(Tile::Open) => position = target_pos,
                            Some(Tile::Solid) => break,
                            None => {
                                // Wrap around to the other side based on the known bounds
                                let wrap_target = match direction {
                                    Direction::Right => {
                                        (rows.get(&position.1).unwrap().0, position.1)
                                    }
                                    Direction::Down => {
                                        (position.0, columns.get(&position.0).unwrap().0)
                                    }
                                    Direction::Left => {
                                        (rows.get(&position.1).unwrap().1, position.1)
                                    }
                                    Direction::Up => {
                                        (position.0, columns.get(&position.0).unwrap().1)
                                    }
                                };

                                match world.get(&wrap_target) {
                                    Some(Tile::Open) => position = wrap_target,
                                    Some(Tile::Solid) => break,
                                    None => return Err(AocError::logic("wrap missed the map")),
                                };
                            }
                        }
                    }
                }
            }
        }

        Ok(1000 * position.1 + 4 * position.0 + direction.to_password())
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let (world, path, rows, _) = Day22::parse(input)?;

        let size = rows.values().map(|(min, max)| max - min + 1).min().unwrap();

        let mut facets: HashMap<Coords, Facet> = HashMap::new();
        let leftmost_open: Coords = Day22::find_leftmost_open(&world, &rows)?;
        let starting_facet = Day22::find_cube_facets(size, world, &mut facets, leftmost_open);

        // Then fold the cube and solve neighbour links, increasing the search distance
        // from one to up until all the neighbours are found.
        Day22::fold_cube(&mut facets);

        let (facet, position, direction) = Day22::follow_path(starting_facet, path, &facets, size)?;

        let final_facet = facets.get(&facet).unwrap();
        match final_facet.tiles.get(&position) {
            Some((_, world_position)) => {
                Ok(1000 * world_position.1 + 4 * world_position.0 + direction.to_password())
            }
            None => Err(AocError::logic("final facet missing")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "        ...#    ",
        "        .#..    ",
        "        #...    ",
        "        ....    ",
        "...#.......#    ",
        "........#...    ",
        "..#....#....    ",
        "..........#.    ",
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
}
