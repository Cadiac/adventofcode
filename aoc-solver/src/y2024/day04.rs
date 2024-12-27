use crate::solution::{AocError, Solution};

type Grid = Vec<Vec<char>>;
type Coords = (isize, isize);

const DIRECTIONS: [Coords; 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn parse(input: &str) -> Result<Grid, AocError> {
    let grid: Grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    Ok(grid)
}

fn is_within_bounds(pos: Coords, width: isize, height: isize) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < width && pos.1 < height
}

fn xmas(x: usize, y: usize, (dx, dy): Coords, width: isize, height: isize, grid: &Grid) -> usize {
    let mut pos = (x as isize, y as isize);

    for current in ['M', 'A', 'S'] {
        pos = (pos.0 + dx, pos.1 + dy);

        if !is_within_bounds(pos, width, height) || grid[pos.1 as usize][pos.0 as usize] != current
        {
            return 0;
        }
    }

    1
}

fn cross(x: usize, y: usize, width: isize, height: isize, grid: &Grid) -> usize {
    let diagonals = [[(-1, 1), (1, -1)], [(-1, -1), (1, 1)]];

    let valid = diagonals.iter().all(|[(dx1, dy1), (dx2, dy2)]| {
        let first = (x as isize + dx1, y as isize + dy1);
        let second = (x as isize + dx2, y as isize + dy2);

        if !is_within_bounds(first, width, height) || !is_within_bounds(second, width, height) {
            return false;
        }

        (grid[first.1 as usize][first.0 as usize] == 'M'
            && grid[second.1 as usize][second.0 as usize] == 'S')
            || (grid[first.1 as usize][first.0 as usize] == 'S'
                && grid[second.1 as usize][second.0 as usize] == 'M')
    });

    if !valid {
        return 0;
    }

    1
}

pub struct Day04;
impl Solution for Day04 {
    type Part1 = usize;
    type Part2 = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day04.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let grid = parse(input)?;

        let mut count = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let width = grid[y].len() as isize;
                let height = grid.len() as isize;

                if grid[y][x] == 'X' {
                    for direction in DIRECTIONS.into_iter() {
                        count += xmas(x, y, direction, width, height, &grid);
                    }
                }
            }
        }

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let grid = parse(input)?;

        let mut count = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let width = grid[y].len() as isize;
                let height = grid.len() as isize;

                if grid[y][x] == 'A' {
                    count += cross(x, y, width, height, &grid);
                }
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day04.part_1(
                "..X...\n\
                 .SAMX.\n\
                 .A..A.\n\
                 XMAS.S\n\
                 .X...."
            ),
            Ok(4)
        );
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(
            Day04.part_1(
                "MMMSXXMASM\n\
                 MSAMXMSMSA\n\
                 AMXSXMAAMM\n\
                 MSAMASMSMX\n\
                 XMASAMXAMM\n\
                 XXAMMXXAMA\n\
                 SMSMSASXSS\n\
                 SAXAMASAAA\n\
                 MAMMMXMMMM\n\
                 MXMXAXMASX"
            ),
            Ok(18)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day04.part_2(
                "MMMSXXMASM\n\
                 MSAMXMSMSA\n\
                 AMXSXMAAMM\n\
                 MSAMASMSMX\n\
                 XMASAMXAMM\n\
                 XXAMMXXAMA\n\
                 SMSMSASXSS\n\
                 SAXAMASAAA\n\
                 MAMMMXMMMM\n\
                 MXMXAXMASX"
            ),
            Ok(9)
        );
    }
}
