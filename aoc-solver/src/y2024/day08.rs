use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

type Coords = (isize, isize);
type Grid = HashMap<char, HashSet<Coords>>;

fn parse(input: &str) -> Result<(Grid, usize, usize), AocError> {
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol == '.' {
                continue;
            }

            grid.entry(symbol)
                .or_insert(HashSet::new())
                .insert((x as isize, y as isize));
        }
    }

    let height = input.lines().count();
    let width = input
        .lines()
        .next()
        .ok_or_else(|| AocError::parse(input, "No lines?"))?
        .len();

    Ok((grid, width, height))
}

fn is_within_bounds(pos: &Coords, width: usize, height: usize) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < (width as isize) && pos.1 < (height as isize)
}

fn find_antinodes(
    grid: &HashMap<char, HashSet<(isize, isize)>>,
    width: usize,
    height: usize,
    allow_resonance: bool,
) -> HashSet<(isize, isize)> {
    grid.values()
        .flat_map(|positions| {
            positions.iter().combinations(2).flat_map(|pair| {
                let dx = pair[1].0 - pair[0].0;
                let dy = pair[1].1 - pair[0].1;

                let left = (1..)
                    .map(|i| (i, (pair[0].0 - i * dx, pair[0].1 - i * dy)))
                    .take_while(|(i, position)| {
                        (allow_resonance || *i == 1) && is_within_bounds(position, width, height)
                    })
                    .map(|(_, position)| position);

                let right = (1..)
                    .map(|i| (i, (pair[1].0 + i * dx, pair[1].1 + i * dy)))
                    .take_while(|(i, position)| {
                        (allow_resonance || *i == 1) && is_within_bounds(position, width, height)
                    })
                    .map(|(_, position)| position);

                left.chain(right).collect::<Vec<_>>()
            })
        })
        .collect()
}

pub struct Day08;
impl Solution for Day08 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day08.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (grid, width, height) = parse(input)?;
        let antinodes = find_antinodes(&grid, width, height, false);

        Ok(antinodes.len())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (grid, width, height) = parse(input)?;
        let mut antinodes = find_antinodes(&grid, width, height, true);

        for antennas in grid.values() {
            for antenna in antennas.iter() {
                antinodes.insert(*antenna);
            }
        }

        Ok(antinodes.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day08.part_1(
                "............\n\
                 ........0...\n\
                 .....0......\n\
                 .......0....\n\
                 ....0.......\n\
                 ......A.....\n\
                 ............\n\
                 ............\n\
                 ........A...\n\
                 .........A..\n\
                 ............\n\
                 ............"
            ),
            Ok(14)
        );
    }

    #[test]
    fn it_solves_part1_simple_1() {
        assert_eq!(
            Day08.part_1(
                "..........\n\
                 ..........\n\
                 ..........\n\
                 ....a.....\n\
                 ..........\n\
                 .....a....\n\
                 ..........\n\
                 ..........\n\
                 ..........\n\
                 .........."
            ),
            Ok(2)
        );
    }

    #[test]
    fn it_finds_antinodes() {
        assert_eq!(
            find_antinodes(
                &HashMap::from([('a', HashSet::from([(4, 3), (5, 5)]))]),
                10,
                10,
                false,
            ),
            HashSet::from([(3, 1), (6, 7)])
        );
    }

    #[test]
    fn it_finds_antinodes_with_resonance() {
        assert_eq!(
            find_antinodes(
                &HashMap::from([('a', HashSet::from([(4, 3), (5, 5)]))]),
                10,
                10,
                true,
            ),
            HashSet::from([(3, 1), (6, 7), (7, 9)])
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day08.part_2(
                "............\n\
                 ........0...\n\
                 .....0......\n\
                 .......0....\n\
                 ....0.......\n\
                 ......A.....\n\
                 ............\n\
                 ............\n\
                 ........A...\n\
                 .........A..\n\
                 ............\n\
                 ............"
            ),
            Ok(34)
        );
    }

    #[test]
    fn it_solves_part2_simple() {
        assert_eq!(
            Day08.part_2(
                "T.........\n\
                 ...T......\n\
                 .T........\n\
                 ..........\n\
                 ..........\n\
                 ..........\n\
                 ..........\n\
                 ..........\n\
                 ..........\n\
                 .........."
            ),
            Ok(9)
        );
    }
}
