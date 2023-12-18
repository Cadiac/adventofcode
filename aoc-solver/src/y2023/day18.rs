use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day18;

type Point = (i64, i64);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn as_delta(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

struct Instruction {
    direction: Direction,
    steps: u64,
    color: String,
}

fn parse(input: &str) -> Result<Vec<Instruction>, AocError> {
    let instructions = input
        .trim()
        .lines()
        .map(|line| {
            let (direction, steps, color) = line
                .splitn(3, ' ')
                .collect_tuple()
                .ok_or(AocError::parse(line, "Invalid mapping"))?;

            let direction = match direction {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => return Err(AocError::parse(direction, "Invalid direction")),
            };

            let steps = steps.parse().map_err(|err| AocError::parse(steps, err))?;
            let color = color
                .strip_prefix('(')
                .and_then(|color| color.strip_suffix(')'))
                .ok_or(AocError::parse(color, "Invalid color"))?;

            Ok(Instruction {
                direction,
                steps,
                color: color.to_owned(),
            })
        })
        .try_collect()?;

    Ok(instructions)
}

fn execute(instructions: &[Instruction]) -> Result<(u64, Vec<Point>), AocError> {
    let mut current = (0, 0);
    let mut trench_len = 0;
    let mut vertices = Vec::with_capacity(instructions.len());

    for instruction in instructions {
        vertices.push((current.0, current.1));

        trench_len += instruction.steps;

        let (dx, dy) = instruction.direction.as_delta();
        current.0 += instruction.steps as i64 * dx as i64;
        current.1 += instruction.steps as i64 * dy as i64;
    }

    Ok((trench_len, vertices))
}

fn shoelace(vertices: &[Point]) -> i64 {
    let mut area = 0;
    let n = vertices.len();

    for i in 0..n {
        let j = (i + 1) % n;

        let x1 = vertices[i].0;
        let y1 = vertices[i].1;
        let x2 = vertices[j].0;
        let y2 = vertices[j].1;

        area += x1 * y2 - x2 * y1;
    }

    area.abs() / 2
}

fn calculate_area(vertices: &[Point], trench_len: u64) -> u64 {
    // Calculate the area "A" of polygon using Shoelace formula
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let area = shoelace(vertices);

    // Solve the amount of interior points "i" with Pick's theorem,
    // using trench length as "b" and the area from shoelace as "A"
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b/2 - 1
    // i = -b/2 + 1 + A
    let interior_points = trench_len as i64 / -2 + 1 + area;

    // Add together the volume dug out while digging the trench and
    // the volume contained within it
    trench_len + interior_points as u64
}

impl Solution for Day18 {
    type A = u64;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day18.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let instructions = parse(input)?;

        let (trench_len, vertices) = execute(&instructions)?;
        let total_area = calculate_area(&vertices, trench_len);

        Ok(total_area)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let instructions: Vec<_> = parse(input)?
            .into_iter()
            .map(|instruction| {
                let color = instruction
                    .color
                    .strip_prefix('#')
                    .ok_or(AocError::parse(&instruction.color, "Missing '#'-prefix"))?;

                let (steps, direction) = color.split_at(5);

                let steps =
                    u64::from_str_radix(steps, 16).map_err(|err| AocError::parse(color, err))?;

                let direction = match direction {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => return Err(AocError::parse(color, "Invalid direction mapping")),
                };

                Ok(Instruction {
                    direction,
                    steps,
                    color: instruction.color,
                })
            })
            .try_collect()?;

        let (trench_len, vertices) = execute(&instructions)?;
        let total_area = calculate_area(&vertices, trench_len);

        Ok(total_area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day18.part_1(
                "R 6 (#70c710)\n\
                 D 5 (#0dc571)\n\
                 L 2 (#5713f0)\n\
                 D 2 (#d2c081)\n\
                 R 2 (#59c680)\n\
                 D 2 (#411b91)\n\
                 L 5 (#8ceee2)\n\
                 U 2 (#caa173)\n\
                 L 1 (#1b58a2)\n\
                 U 2 (#caa171)\n\
                 R 2 (#7807d2)\n\
                 U 3 (#a77fa3)\n\
                 L 2 (#015232)\n\
                 U 2 (#7a21e3)\n"
            ),
            Ok(62)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day18.part_2(
                "R 6 (#70c710)\n\
                 D 5 (#0dc571)\n\
                 L 2 (#5713f0)\n\
                 D 2 (#d2c081)\n\
                 R 2 (#59c680)\n\
                 D 2 (#411b91)\n\
                 L 5 (#8ceee2)\n\
                 U 2 (#caa173)\n\
                 L 1 (#1b58a2)\n\
                 U 2 (#caa171)\n\
                 R 2 (#7807d2)\n\
                 U 3 (#a77fa3)\n\
                 L 2 (#015232)\n\
                 U 2 (#7a21e3)\n"
            ),
            Ok(952408144115)
        );
    }
}
