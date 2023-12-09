use std::{cmp::Ordering, collections::BinaryHeap};

use crate::solution::{AocError, Solution};

const NEIGHBOURS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

type Coord = (usize, usize);

pub struct Day12;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Search {
    coords: Coord,
    elevation: u8,
    distance: u32,
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

impl Day12 {
    fn parse(input: &str) -> Result<(Vec<Vec<u8>>, Coord, Coord), AocError> {
        let mut grid = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in input.lines().enumerate() {
            grid.push(Vec::new());
            for (x, c) in line.bytes().enumerate() {
                if c == b'S' {
                    start = (x, y);
                    grid[y].push(0);
                } else if c == b'E' {
                    end = (x, y);
                    grid[y].push(b'z' - b'a' + 1);
                } else {
                    grid[y].push(c - b'a');
                }
            }
        }

        Ok((grid, start, end))
    }
}

// Copied my 2021/day15 solution, seems to work here too
fn dijkstra(grid: Vec<Vec<u8>>, source: Coord, target: Coord) -> Option<u32> {
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut heap: BinaryHeap<Search> = BinaryHeap::new();

    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    dist[source.1][source.0] = 0;
    heap.push(Search {
        elevation: grid[source.1][source.0],
        distance: 0,
        coords: source,
    });

    while let Some(Search {
        coords,
        elevation,
        distance,
    }) = heap.pop()
    {
        if coords == target {
            return Some(dist[coords.1][coords.0]);
        }

        // we've already found a better way
        if distance > dist[coords.1][coords.0] {
            continue;
        }

        for offset in NEIGHBOURS {
            let c: (i32, i32) = (coords.0 as i32 + offset.0, coords.1 as i32 + offset.1);
            if c.0 >= 0 && c.1 >= 0 && c.0 < width && c.1 < height {
                let neighbour = (c.0 as usize, c.1 as usize);
                if grid[neighbour.1][neighbour.0] > elevation
                    && grid[neighbour.1][neighbour.0] - elevation > 1
                {
                    // We can only climb at most one step higher, skip this path
                    continue;
                }

                let next = Search {
                    distance: distance + 1,
                    elevation: grid[neighbour.1][neighbour.0],
                    coords: neighbour,
                };

                if next.distance < dist[neighbour.1][neighbour.0] {
                    heap.push(next);
                    dist[neighbour.1][neighbour.0] = next.distance;
                }
            }
        }
    }

    None
}

impl Solution for Day12 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day12.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (grid, source, target) = Self::parse(input)?;
        dijkstra(grid, source, target).ok_or_else(|| AocError::logic("no path"))
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let (grid, _, target) = Self::parse(input)?;

        let mut shortest = None;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 0 {
                    if let Some(distance) = dijkstra(grid.clone(), (x, y), target) {
                        match shortest {
                            None => {
                                shortest = Some(distance);
                            }
                            Some(previous) => {
                                if previous > distance {
                                    shortest = Some(distance);
                                }
                            }
                        }
                    }
                }
            }
        }

        shortest.ok_or_else(|| AocError::logic("no path"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1() {
        assert_eq!(
            Day12.part_1(
                "Sabqponm\n\
                 abcryxxl\n\
                 accszExk\n\
                 acctuvwj\n\
                 abdefghi"
            ),
            Ok(31)
        );
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(
            Day12.part_2(
                "Sabqponm\n\
                 abcryxxl\n\
                 accszExk\n\
                 acctuvwj\n\
                 abdefghi"
            ),
            Ok(29)
        );
    }
}
