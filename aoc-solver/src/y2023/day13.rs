use crate::solution::{AocError, Solution};

pub struct Day13;

enum Direction {
    Horizontal,
    Vertical,
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .trim()
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn find_reflection_line(pattern: &[Vec<char>], part_2: bool) -> Option<(usize, Direction)> {
    for y in 0..pattern.len() - 1 {
        let mut differences = 0;
        let mut distance = 0;

        while y + distance < pattern.len() - 1 && y >= distance {
            for x in 0..pattern[y].len() {
                if pattern[y + distance + 1][x] != pattern[y - distance][x] {
                    differences += 1;
                }
            }

            distance += 1;
        }

        if (part_2 && differences == 1) || (!part_2 && differences == 0) {
            return Some((y + 1, Direction::Horizontal));
        }
    }

    for x in 0..pattern[0].len() - 1 {
        let mut differences = 0;
        let mut distance = 0;

        while x + distance < pattern[0].len() - 1 && x >= distance {
            for line in pattern {
                if line[x + distance + 1] != line[x - distance] {
                    differences += 1;
                }
            }

            distance += 1;
        }

        if (part_2 && differences == 1) || (!part_2 && differences == 0) {
            return Some((x + 1, Direction::Vertical));
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
        let patterns = parse(input);

        let total = patterns
            .iter()
            .map(|pattern| match find_reflection_line(pattern, false) {
                Some((line, Direction::Horizontal)) => 100 * line,
                Some((line, Direction::Vertical)) => line,
                None => 0,
            })
            .sum();

        Ok(total)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let patterns = parse(input);

        let total = patterns
            .iter()
            .map(|pattern| match find_reflection_line(pattern, true) {
                Some((line, Direction::Horizontal)) => 100 * line,
                Some((line, Direction::Vertical)) => line,
                None => 0,
            })
            .sum();

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
