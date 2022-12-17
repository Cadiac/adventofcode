use std::collections::{HashSet};

use crate::solution::{AocError, Solution};

const SHAPE_PIXELS: [&[(i64, i64)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],         // "horizontal line"
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // "plus"
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // "reverse L"
    &[(0, 0), (0, 1), (0, 2), (0, 3)],         // "vertical line"
    &[(0, 0), (1, 0), (0, 1), (1, 1)],         // "square"
];

const CHAMBER_WIDTH: i64 = 7;

enum Shape {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

enum Direction {
    Left,
    Right,
    Down,
}

struct Rock {
    shape: Shape,
    x: i64,
    y: i64,
}

impl Rock {
    fn new(index: usize, max_y: i64) -> Self {
        let shape = match index {
            0 => Shape::Horizontal,
            1 => Shape::Plus,
            2 => Shape::Corner,
            3 => Shape::Vertical,
            4 => Shape::Square,
            _ => unreachable!(),
        };

        Rock {
            shape,
            x: 2,
            y: 3 + max_y,
        }
    }

    fn move_direction(&mut self, direction: Direction, chamber: &HashSet<(i64, i64)>) -> bool {
        let delta = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        };

        let hit_something = self
            .pixels()
            .iter()
            .any(|pixel| Rock::intersects(pixel, &delta, chamber));

        if hit_something {
            return false;
        }

        self.x += delta.0;
        self.y += delta.1;
        true
    }

    fn intersects(pixel: &(i64, i64), delta: &(i64, i64), chamber: &HashSet<(i64, i64)>) -> bool {
        let coords = (pixel.0 + delta.0, pixel.1 + delta.1);

        chamber.contains(&coords) || coords.0 >= CHAMBER_WIDTH || coords.0 < 0 || coords.1 < 0
    }

    fn pixels(&self) -> Vec<(i64, i64)> {
        let base = match &self.shape {
            Shape::Horizontal => SHAPE_PIXELS[0],
            Shape::Plus => SHAPE_PIXELS[1],
            Shape::Corner => SHAPE_PIXELS[2],
            Shape::Vertical => SHAPE_PIXELS[3],
            Shape::Square => SHAPE_PIXELS[4],
        };

        base.iter().map(|(x, y)| (self.x + x, self.y + y)).collect()
    }
}

pub struct Day17;

impl Day17 {
    fn simulate(input: &str, count: usize) -> i64 {
        let mut rocks = 0;

        let mut chamber: HashSet<(i64, i64)> = HashSet::new();
        let mut movements = input.chars().cycle();

        let mut max_y = chamber.iter().map(|(_, y)| y).max().unwrap_or(&0);
        let mut rock = Rock::new(rocks % 5, *max_y);

        loop {
            let direction = match movements.next() {
                Some('<') => Direction::Left,
                Some('>') => Direction::Right,
                _ => unreachable!(),
            };

            rock.move_direction(direction, &chamber);

            if !rock.move_direction(Direction::Down, &chamber) {
                // Hit the bottom
                for pixel in rock.pixels() {
                    chamber.insert(pixel);
                }

                max_y = chamber.iter().map(|(_, y)| y).max().unwrap_or(&0);
                if rocks >= count - 1 {
                    return *max_y + 1;
                }

                // Spawn new rock
                rocks += 1;
                rock = Rock::new(rocks % 5, *max_y + 1);
            }
        }
    }
}

impl Solution for Day17 {
    type F = i64;
    type S = i64;

    fn name(&self) -> &'static str {
        "Day 17"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day17.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        Ok(Day17::simulate(input, 2022))
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        // Ok(Day17::simulate(input, 1000000000000))
        // TODO
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[ignore]
    #[test]
    fn it_solves_part1() {
        assert_eq!(Day17.part_1(INPUT), Ok(3068));
    }
}
