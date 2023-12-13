use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day13;

fn parse(input: &str) -> Result<Vec<Vec<Vec<bool>>>, AocError> {
    input
        .trim()
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    let row = line
                        .chars()
                        .map(|spring| match spring {
                            '.' => Ok(false),
                            '#' => Ok(true),
                            _ => Err(AocError::parse(spring, "Unexpected symbol")),
                        })
                        .try_collect()?;

                    Ok(row)
                })
                .try_collect()
        })
        .try_collect()
}

fn find_reflection_line(
    pattern: &[Vec<bool>],
    ignore_mirror: Option<(usize, bool)>,
) -> Option<(usize, bool)> {
    for y in 0..pattern.len() - 1 {
        // Does this row match the next one?
        let mut is_mirrored = true;
        for x in 0..pattern[y].len() {
            if pattern[y][x] != pattern[y + 1][x] {
                is_mirrored = false;
                break;
            }
        }

        if !is_mirrored {
            continue;
        }

        let mut distance = 1;
        while is_mirrored && y + distance + 1 < pattern.len() && y >= distance {
            for x in 0..pattern[y].len() {
                if pattern[y + distance + 1][x] != pattern[y - distance][x] {
                    is_mirrored = false;
                    break;
                }
            }

            distance += 1;
        }

        if is_mirrored {
            if let Some(ignore) = ignore_mirror {
                if !ignore.1 || ignore.0 != y + 1 {
                    return Some((y + 1, true));
                }
            } else {
                return Some((y + 1, true));
            }
        }
    }

    for x in 0..pattern[0].len() - 1 {
        // Does this column match the next one?
        let mut is_mirrored = true;
        for y in 0..pattern.len() {
            if pattern[y][x] != pattern[y][x + 1] {
                is_mirrored = false;
                break;
            }
        }

        if !is_mirrored {
            continue;
        }

        let mut distance = 1;
        while is_mirrored && x + distance + 1 < pattern[0].len() && x >= distance {
            for y in 0..pattern.len() {
                if pattern[y][x + distance + 1] != pattern[y][x - distance] {
                    is_mirrored = false;
                    break;
                }
            }

            distance += 1;
        }

        if is_mirrored {
            if let Some(ignore) = ignore_mirror {
                if ignore.1 || ignore.0 != x + 1 {
                    return Some((x + 1, false));
                }
            } else {
                return Some((x + 1, false));
            }
        }
    }

    None
}

impl Solution for Day13 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day13.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let patterns = parse(input)?;

        let total = patterns
            .iter()
            .map(|pattern| match find_reflection_line(pattern, None) {
                Some((line, true)) => 100 * line,
                Some((line, false)) => line,
                None => 0,
            })
            .sum();

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let patterns = parse(input)?;

        let total = patterns
            .iter()
            .map(|pattern| {
                let original = find_reflection_line(pattern, None);

                for y in 0..pattern.len() {
                    for x in 0..pattern[y].len() {
                        let mut fixed = pattern.clone();
                        fixed[y][x] = !fixed[y][x];

                        if let Some(fixed_reflection) = find_reflection_line(&fixed, original) {
                            if fixed_reflection.1 {
                                return 100 * fixed_reflection.0;
                            } else {
                                return fixed_reflection.0;
                            }
                        }
                    }
                }

                0
            })
            .sum();

        // Too low

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_vertical() {
        assert_eq!(
            Day13.part_1(
                "#.##..##.\n\
                 ..#.##.#.\n\
                 ##......#\n\
                 ##......#\n\
                 ..#.##.#.\n\
                 ..##..##.\n\
                 #.#.##.#.\n"
            ),
            Ok(5)
        );
    }

    #[test]
    fn it_solves_part1_horizontal() {
        assert_eq!(
            Day13.part_1(
                "#...##..#\n\
                 #....#..#\n\
                 ..##..###\n\
                 #####.##.\n\
                 #####.##.\n\
                 ..##..###\n\
                 #....#..#\n"
            ),
            Ok(400)
        );
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day13.part_1(
                "#.##..##.\n\
                 ..#.##.#.\n\
                 ##......#\n\
                 ##......#\n\
                 ..#.##.#.\n\
                 ..##..##.\n\
                 #.#.##.#.\n\
                 \n\
                 #...##..#\n\
                 #....#..#\n\
                 ..##..###\n\
                 #####.##.\n\
                 #####.##.\n\
                 ..##..###\n\
                 #....#..#\n"
            ),
            Ok(405)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day13.part_2(
                "#.##..##.\n\
                 ..#.##.#.\n\
                 ##......#\n\
                 ##......#\n\
                 ..#.##.#.\n\
                 ..##..##.\n\
                 #.#.##.#.\n\
                 \n\
                 #...##..#\n\
                 #....#..#\n\
                 ..##..###\n\
                 #####.##.\n\
                 #####.##.\n\
                 ..##..###\n\
                 #....#..#\n"
            ),
            Ok(400)
        );
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            Day13.part_2(
                "...#.#.##\n\
                 .#####.##\n\
                 .#.##.#..\n\
                 .#.##.#..\n\
                 .#####.##\n\
                 ...#.#.##\n\
                 ###..##..\n\
                 ####.####\n\
                 #..#.#.#.\n\
                 ##...#...\n\
                 ...###...\n\
                 #####....\n\
                 #..##..##\n"
            ),
            Ok(8)
        );
    }
}
