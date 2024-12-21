use std::collections::{HashMap, HashSet};

use crate::solution::{AocError, Solution};

type Coords = (usize, usize);
type Grid = Vec<Vec<bool>>;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

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

fn is_outside_bounds(height: isize, width: isize, x: isize, y: isize) -> bool {
    x < 0 || y < 0 || x >= width || y >= height
}

fn race_to_end(grid: &Grid, start: Coords, end: Coords) -> (Vec<Coords>, HashMap<Coords, u32>) {
    let mut distances: HashMap<Coords, u32> = HashMap::new();
    let mut route: Vec<Coords> = vec![];

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut distance = 0;
    let mut position = start;

    loop {
        route.push(position);
        distances.insert(position, distance);

        if position == end {
            return (route, distances);
        }

        for (dx, dy) in DIRECTIONS {
            let (x, y) = (position.0 as isize + dx, position.1 as isize + dy);

            if is_outside_bounds(height, width, x, y) {
                continue;
            }

            let next_pos = (x as usize, y as usize);

            if grid[y as usize][x as usize] && !distances.contains_key(&next_pos) {
                position = next_pos;
                distance += 1;
                break;
            }
        }
    }
}

fn manhattan(a: &Coords, b: &Coords) -> u32 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u32
}

fn cheat_targets(grid: &Grid, start: &Coords, cheat_duration: u32) -> HashSet<Coords> {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut targets = HashSet::new();
    let duration = cheat_duration as isize;

    for dy in -duration..=duration {
        let max_x = duration - dy.abs();
        for dx in -max_x..=max_x {
            let (x, y) = (start.0 as isize + dx, start.1 as isize + dy);
            if !is_outside_bounds(height, width, x, y) && grid[y as usize][x as usize] {
                targets.insert((x as usize, y as usize));
            }
        }
    }

    targets
}

fn explore_cheats(
    grid: &Grid,
    start: Coords,
    end: Coords,
    cheat_duration: u32,
) -> HashMap<u32, u32> {
    let (route, distances) = race_to_end(grid, start, end);
    let fair_distance = distances[&end];

    let mut cheats: HashMap<u32, u32> = HashMap::new();

    for (distance, position) in route.iter().enumerate() {
        for cheat_target in cheat_targets(grid, position, cheat_duration).iter() {
            let remaining_distance = fair_distance - distances[cheat_target];
            let cheat_distance =
                distance as u32 + manhattan(position, cheat_target) + remaining_distance;
            let savings = fair_distance.saturating_sub(cheat_distance);

            *cheats.entry(savings).or_default() += 1;
        }
    }

    cheats
}

fn filter_threshold(cheats: HashMap<u32, u32>, threshold: u32) -> u32 {
    cheats
        .into_iter()
        .filter(|(distance, _cheats)| *distance >= threshold)
        .map(|(_distance, cheats)| cheats)
        .sum()
}

pub struct Day20;
impl Solution for Day20 {
    type A = u32;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day20.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (grid, start, end) = parse(input)?;
        let cheats = explore_cheats(&grid, start, end, 2);
        let good_cheats = filter_threshold(cheats, 100);

        Ok(good_cheats)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let (grid, start, end) = parse(input)?;
        let cheats = explore_cheats(&grid, start, end, 20);
        let good_cheats = filter_threshold(cheats, 100);

        Ok(good_cheats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str =
        "###############\n\
         #...#...#.....#\n\
         #.#.#.#.#.###.#\n\
         #S#...#.#.#...#\n\
         #######.#.#.###\n\
         #######.#.#...#\n\
         #######.#.###.#\n\
         ###..E#...#...#\n\
         ###.#######.###\n\
         #...###...#...#\n\
         #.#####.#.###.#\n\
         #.#...#.#.#...#\n\
         #.#.#.#.#.#.###\n\
         #...#...#...###\n\
         ###############";

    #[test]
    fn it_solves_part1_example() {
        let (grid, start, end) = parse(INPUT).unwrap();

        let cheats = explore_cheats(&grid, start, end, 2);
        assert_eq!(cheats[&2], 14);
        assert_eq!(cheats[&4], 14);
        assert_eq!(cheats[&6], 2);
        assert_eq!(cheats[&8], 4);
        assert_eq!(cheats[&10], 2);
        assert_eq!(cheats[&12], 3);
        assert_eq!(cheats[&20], 1);
        assert_eq!(cheats[&36], 1);
        assert_eq!(cheats[&38], 1);
        assert_eq!(cheats[&40], 1);
        assert_eq!(cheats[&64], 1);
    }

    #[test]
    fn it_solves_part2_example() {
        let (grid, start, end) = parse(INPUT).unwrap();

        let cheats = explore_cheats(&grid, start, end, 20);

        assert_eq!(cheats[&50], 32);
        assert_eq!(cheats[&52], 31);
        assert_eq!(cheats[&56], 39);
        assert_eq!(cheats[&58], 25);
        assert_eq!(cheats[&60], 23);
        assert_eq!(cheats[&62], 20);
        assert_eq!(cheats[&64], 19);
        assert_eq!(cheats[&66], 12);
        assert_eq!(cheats[&68], 14);
        assert_eq!(cheats[&70], 12);
        assert_eq!(cheats[&72], 22);
        assert_eq!(cheats[&74], 4);
        assert_eq!(cheats[&76], 3);
    }

    #[test]
    fn it_finds_expected_cheats_1() {
        let (grid, _start, _end) = parse(INPUT).unwrap();

        assert_eq!(
            cheat_targets(&grid, &(2, 3), 1),
            HashSet::from([(1, 3), (3, 3)])
        );
    }

    #[test]
    fn it_finds_expected_cheats_2() {
        let (grid, _start, _end) = parse(INPUT).unwrap();

        assert_eq!(
            cheat_targets(&grid, &(2, 3), 2),
            HashSet::from([(1, 2), (2, 1), (3, 2), (4, 3), (1, 3), (3, 3)])
        );
    }

    #[test]
    fn it_finds_expected_cheats_3() {
        let (grid, _start, _end) = parse(INPUT).unwrap();

        assert_eq!(
            cheat_targets(&grid, &(2, 3), 3),
            HashSet::from([
                (3, 3),
                (3, 2),
                (3, 1),
                (1, 1),
                (2, 1),
                (4, 3),
                (1, 2),
                (1, 3),
                (5, 3)
            ])
        );
    }

    #[test]
    fn it_finds_expected_cheats_5() {
        let (grid, _start, _end) = parse(INPUT).unwrap();

        assert_eq!(cheat_targets(&grid, &(2, 3), 5).len(), 13);
    }

    #[test]
    fn it_finds_expected_cheats_20() {
        let (grid, _start, _end) = parse(INPUT).unwrap();

        assert_eq!(cheat_targets(&grid, &(2, 3), 20).len(), 85);
    }
}
