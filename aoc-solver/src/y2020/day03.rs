use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day03;

fn count_trees(
    trees: &HashSet<(usize, usize)>,
    columns: usize,
    rows: usize,
    right: usize,
    down: usize,
) -> usize {
    let mut current: (usize, usize) = (0, 0);
    let mut hits = 0;

    while current.1 < rows {
        if trees.contains(&current) {
            hits += 1;
        }

        current.0 = (current.0 + right) % columns;
        current.1 += down;
    }

    hits
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, usize, usize) {
    let mut trees: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            if tile == '#' {
                trees.insert((x, y));
            }
        }
    }

    let rows = input.lines().count();
    let columns = input.lines().next().unwrap().len();

    (trees, columns, rows)
}

impl Solution for Day03 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day03.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let (trees, columns, rows) = parse(input);
        let count = count_trees(&trees, columns, rows, 3, 1);

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (trees, columns, rows) = parse(input);
        let count = count_trees(&trees, columns, rows, 1, 1)
            * count_trees(&trees, columns, rows, 3, 1)
            * count_trees(&trees, columns, rows, 5, 1)
            * count_trees(&trees, columns, rows, 7, 1)
            * count_trees(&trees, columns, rows, 1, 2);

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        let (trees, rows, columns) = parse(
            "..##.......\n\
             #...#...#..\n\
             .#....#..#.\n\
             ..#.#...#.#\n\
             .#...##..#.\n\
             ..#.##.....\n\
             .#.#.#....#\n\
             .#........#\n\
             #.##...#...\n\
             #...##....#\n\
             .#..#...#.#",
        );

        assert_eq!(columns, 11);
        assert_eq!(rows, 11);
        assert_eq!(trees.len(), 37);

        assert_eq!(count_trees(&trees, columns, rows, 3, 1), 7);
    }

    #[test]
    fn it_solves_part2_example() {
        let (trees, rows, columns) = parse(
            "..##.......\n\
             #...#...#..\n\
             .#....#..#.\n\
             ..#.#...#.#\n\
             .#...##..#.\n\
             ..#.##.....\n\
             .#.#.#....#\n\
             .#........#\n\
             #.##...#...\n\
             #...##....#\n\
             .#..#...#.#",
        );

        assert_eq!(count_trees(&trees, columns, rows, 1, 1), 2);
        assert_eq!(count_trees(&trees, columns, rows, 3, 1), 7);
        assert_eq!(count_trees(&trees, columns, rows, 5, 1), 3);
        assert_eq!(count_trees(&trees, columns, rows, 7, 1), 4);
        assert_eq!(count_trees(&trees, columns, rows, 1, 2), 2);
    }
}
