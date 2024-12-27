use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::solution::{AocError, Solution};

type Coords = (usize, usize);
type Grid = Vec<Vec<bool>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn rotate_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Search {
    score: u32,
    position: Coords,
    direction: Direction,
    visited: HashSet<Coords>,
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<(Grid, Coords, Coords), AocError> {
    let mut grid = vec![];
    let mut start = None;
    let mut end = None;

    for (y, line) in input.trim().lines().enumerate() {
        let mut row = vec![];
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '#' => row.push(false),
                '.' => row.push(true),
                'S' => {
                    row.push(true);
                    start = Some((x, y))
                }
                'E' => {
                    row.push(true);
                    end = Some((x, y))
                }
                other => return Err(AocError::parse(other, "Unexpected tile")),
            }
        }
        grid.push(row);
    }

    match (start, end) {
        (Some(start), Some(end)) => Ok((grid, start, end)),
        _ => Err(AocError::parse(input, "Missing start or end")),
    }
}

fn dijkstra(grid: &Grid, start: Coords, end: Coords, find_all: bool) -> Option<u32> {
    let mut scores: HashMap<(Coords, Direction), u32> = HashMap::new();
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    heap.push(Search {
        score: 0,
        direction: Direction::East,
        position: start,
        visited: HashSet::new(),
    });

    let mut best_score: Option<u32> = None;
    let mut best_path_tiles: HashSet<Coords> = HashSet::new();

    while let Some(Search {
        score,
        direction,
        position,
        mut visited,
    }) = heap.pop()
    {
        if !visited.insert(position) {
            continue;
        }

        if score > *scores.get(&(position, direction)).unwrap_or(&u32::MAX) {
            continue;
        }

        if position == end {
            if find_all {
                match best_score {
                    None => {
                        best_score = Some(score);
                        best_path_tiles.extend(visited.iter());
                    }
                    Some(best) if best == score => {
                        best_path_tiles.extend(visited.iter());
                    }
                    _ => {}
                }
            } else {
                return Some(score);
            }
        }

        for (next_direction, turn_cost) in [
            (direction, 0),
            (direction.rotate_left(), 1000),
            (direction.rotate_right(), 1000),
        ] {
            let next_score = score + turn_cost + 1;

            let (dx, dy) = next_direction.as_delta();
            let (x, y) = (position.0 as isize + dx, position.1 as isize + dy);

            if x < 0 || y < 0 || x >= width || y >= height {
                continue;
            }

            if !grid[y as usize][x as usize] {
                continue;
            }

            let next_pos = (x as usize, y as usize);
            let best_known = scores.entry((next_pos, next_direction)).or_insert(u32::MAX);

            if next_score <= *best_known {
                *best_known = next_score;

                heap.push(Search {
                    score: next_score,
                    direction: next_direction,
                    position: next_pos,
                    visited: visited.clone(),
                });
            }
        }
    }

    if find_all {
        return Some(best_path_tiles.len() as u32);
    }

    None
}

pub struct Day16;
impl Solution for Day16 {
    type Part1 = u32;
    type Part2 = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day16.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (grid, start, end) = parse(input)?;

        let smallest_score =
            dijkstra(&grid, start, end, false).ok_or_else(|| AocError::logic("No path found"))?;

        Ok(smallest_score)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let (grid, start, end) = parse(input)?;

        let smallest_score =
            dijkstra(&grid, start, end, true).ok_or_else(|| AocError::logic("No path found"))?;

        Ok(smallest_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day16.part_1(
                "###############\n\
                 #.......#....E#\n\
                 #.#.###.#.###.#\n\
                 #.....#.#...#.#\n\
                 #.###.#####.#.#\n\
                 #.#.#.......#.#\n\
                 #.#.#####.###.#\n\
                 #...........#.#\n\
                 ###.#.#####.#.#\n\
                 #...#.....#.#.#\n\
                 #.#.#.###.#.#.#\n\
                 #.....#...#.#.#\n\
                 #.###.#.#.#.#.#\n\
                 #S..#.....#...#\n\
                 ###############"
            ),
            Ok(7036)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day16.part_1(
                "#################\n\
                 #...#...#...#..E#\n\
                 #.#.#.#.#.#.#.#.#\n\
                 #.#.#.#...#...#.#\n\
                 #.#.#.#.###.#.#.#\n\
                 #...#.#.#.....#.#\n\
                 #.#.#.#.#.#####.#\n\
                 #.#...#.#.#.....#\n\
                 #.#.#####.#.###.#\n\
                 #.#.#.......#...#\n\
                 #.#.###.#####.###\n\
                 #.#.#...#.....#.#\n\
                 #.#.#.#####.###.#\n\
                 #.#.#.........#.#\n\
                 #.#.#.#########.#\n\
                 #S#.............#\n\
                 #################"
            ),
            Ok(11048)
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day16.part_2(
                "###############\n\
                 #.......#....E#\n\
                 #.#.###.#.###.#\n\
                 #.....#.#...#.#\n\
                 #.###.#####.#.#\n\
                 #.#.#.......#.#\n\
                 #.#.#####.###.#\n\
                 #...........#.#\n\
                 ###.#.#####.#.#\n\
                 #...#.....#.#.#\n\
                 #.#.#.###.#.#.#\n\
                 #.....#...#.#.#\n\
                 #.###.#.#.#.#.#\n\
                 #S..#.....#...#\n\
                 ###############"
            ),
            Ok(45)
        );
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            Day16.part_2(
                "#################\n\
                 #...#...#...#..E#\n\
                 #.#.#.#.#.#.#.#.#\n\
                 #.#.#.#...#...#.#\n\
                 #.#.#.#.###.#.#.#\n\
                 #...#.#.#.....#.#\n\
                 #.#.#.#.#.#####.#\n\
                 #.#...#.#.#.....#\n\
                 #.#.#####.#.###.#\n\
                 #.#.#.......#...#\n\
                 #.#.###.#####.###\n\
                 #.#.#...#.....#.#\n\
                 #.#.#.#####.###.#\n\
                 #.#.#.........#.#\n\
                 #.#.#.#########.#\n\
                 #S#.............#\n\
                 #################"
            ),
            Ok(64)
        );
    }
}
