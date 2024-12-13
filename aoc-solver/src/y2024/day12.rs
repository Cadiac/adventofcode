use std::collections::{HashMap, HashSet};

use crate::solution::{AocError, Solution};

type Coords = (isize, isize);
type Plots = HashMap<Coords, char>;

const DIRECTIONS: [Coords; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse(input: &str) -> Result<Plots, AocError> {
    let tiles = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, plot)| ((x as isize, y as isize), plot))
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(tiles)
}

fn flood_fill(
    start: &Coords,
    visited: &mut HashSet<Coords>,
    plots: &HashMap<Coords, char>,
    plant: &char,
) -> (HashSet<(isize, isize)>, usize) {
    let mut stack = vec![*start];
    let mut region: HashSet<Coords> = HashSet::new();
    let mut fences = 0;

    while let Some(coords) = stack.pop() {
        if !visited.insert(coords) {
            continue;
        }

        region.insert(coords);

        for (dx, dy) in DIRECTIONS {
            let next = (coords.0 + dx, coords.1 + dy);

            match plots.get(&next) {
                Some(next_plot) if next_plot == plant => stack.push(next),
                _ => fences += 1,
            }
        }
    }

    (region, fences)
}

fn count_corners(region: &HashSet<Coords>) -> usize {
    let mut rotation = DIRECTIONS;

    (0..4)
        .map(|_| {
            rotation.rotate_right(1);

            let (dx1, dy1) = rotation[0];
            let (dx2, dy2) = rotation[1];

            region
                .iter()
                .filter(|(x, y)| {
                    let is_outer_corner = !region.contains(&(x + dx1, y + dy1))
                        && !region.contains(&(x + dx2, y + dy2));

                    let is_inner_corner = region.contains(&(x + dx1, y + dy1))
                        && region.contains(&(x + dx2, y + dy2))
                        && !region.contains(&(x + dx1 + dx2, y + dy1 + dy2));

                    is_inner_corner || is_outer_corner
                })
                .count()
        })
        .sum()
}

pub struct Day12;
impl Solution for Day12 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day12.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let plots = parse(input)?;

        let mut visited: HashSet<Coords> = HashSet::new();

        let price = plots
            .iter()
            .map(|(plot, plant)| {
                let (region, fences) = flood_fill(plot, &mut visited, &plots, plant);
                region.len() * fences
            })
            .sum();

        Ok(price)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let plots = parse(input)?;

        let mut visited: HashSet<Coords> = HashSet::new();

        let price = plots
            .iter()
            .map(|(plot, plant)| {
                let (region, _fences) = flood_fill(plot, &mut visited, &plots, plant);
                let sides = count_corners(&region);
                region.len() * sides
            })
            .sum();

        Ok(price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day12.part_1(
                "AAAA\n\
                 BBCD\n\
                 BBCC\n\
                 EEEC"
            ),
            Ok(140)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day12.part_1(
                "OOOOO\n\
                 OXOXO\n\
                 OOOOO\n\
                 OXOXO\n\
                 OOOOO"
            ),
            Ok(772)
        );
    }

    #[test]
    fn it_solves_part1_example_3() {
        assert_eq!(
            Day12.part_1(
                "RRRRIICCFF\n\
                 RRRRIICCCF\n\
                 VVRRRCCFFF\n\
                 VVRCCCJFFF\n\
                 VVVVCJJCFE\n\
                 VVIVCCJJEE\n\
                 VVIIICJJEE\n\
                 MIIIIIJJEE\n\
                 MIIISIJEEE\n\
                 MMMISSJEEE"
            ),
            Ok(1930)
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day12.part_2(
                "AAAA\n\
                 BBCD\n\
                 BBCC\n\
                 EEEC"
            ),
            Ok(80)
        );
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            Day12.part_2(
                "OOOOO\n\
                 OXOXO\n\
                 OOOOO\n\
                 OXOXO\n\
                 OOOOO"
            ),
            Ok(436)
        );
    }

    #[test]
    fn it_solves_part2_example_3() {
        assert_eq!(
            Day12.part_2(
                "EEEEE\n\
                 EXXXX\n\
                 EEEEE\n\
                 EXXXX\n\
                 EEEEE"
            ),
            Ok(236)
        );
    }

    #[test]
    fn it_solves_part2_example_4() {
        assert_eq!(
            Day12.part_2(
                "AAAAAA\n\
                 AAABBA\n\
                 AAABBA\n\
                 ABBAAA\n\
                 ABBAAA\n\
                 AAAAAA"
            ),
            Ok(368)
        );
    }

    #[test]
    fn it_solves_part2_example_5() {
        assert_eq!(
            Day12.part_2(
                "RRRRIICCFF\n\
                 RRRRIICCCF\n\
                 VVRRRCCFFF\n\
                 VVRCCCJFFF\n\
                 VVVVCJJCFE\n\
                 VVIVCCJJEE\n\
                 VVIIICJJEE\n\
                 MIIIIIJJEE\n\
                 MIIISIJEEE\n\
                 MMMISSJEEE"
            ),
            Ok(1206)
        );
    }
}
