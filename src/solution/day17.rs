use std::collections::{BTreeSet, HashMap};

use crate::solution::{AocError, Solution};

const SHAPES: [&[(i64, i64)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],         // "horizontal line"
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // "plus"
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // "reverse L"
    &[(0, 0), (0, 1), (0, 2), (0, 3)],         // "vertical line"
    &[(0, 0), (1, 0), (0, 1), (1, 1)],         // "square"
];

const CHAMBER_WIDTH: i64 = 7;

type Chamber = BTreeSet<(i64, i64)>;

enum Direction {
    Left,
    Right,
    Down,
}

struct Rock {
    shape: usize,
    x: i64,
    y: i64,
}

impl Rock {
    fn new(shape: usize, height: i64) -> Self {
        Rock {
            shape,
            x: 2,
            y: 3 + height,
        }
    }

    fn move_direction(&mut self, direction: Direction, chamber: &Chamber) -> bool {
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

    fn intersects(pixel: &(i64, i64), delta: &(i64, i64), chamber: &Chamber) -> bool {
        let coords = (pixel.0 + delta.0, pixel.1 + delta.1);

        chamber.contains(&coords) || coords.0 >= CHAMBER_WIDTH || coords.0 < 0 || coords.1 < 0
    }

    fn pixels(&self) -> Vec<(i64, i64)> {
        let base_pixels = SHAPES[self.shape];
        base_pixels.iter().map(|(x, y)| (self.x + x, self.y + y)).collect()
    }
}

pub struct Day17;

impl Day17 {
    fn simulate(input: &str, count: u64) -> u64 {
        let mut rocks = 0;
        let mut total_height = 0;

        let mut chamber: Chamber = BTreeSet::new();
        let mut movements = input.chars().enumerate().cycle();

        let mut rock = Rock::new(0, 0);

        // Don't use the cache for small inputs
        let mut use_cache = count < 10000;
        let mut cache: HashMap<(usize, Chamber), (u64, u64)> = HashMap::new();

        loop {
            let (index, direction) = match movements.next() {
                Some((index, '<')) => (index, Direction::Left),
                Some((index, '>')) => (index, Direction::Right),
                _ => unreachable!(),
            };

            rock.move_direction(direction, &chamber);

            if !rock.move_direction(Direction::Down, &chamber) {
                // Hit the bottom
                for pixel in rock.pixels() {
                    chamber.insert(pixel);
                }

                // Find the lowest spots from every column and get rid of everything else
                let cutoff = *(0..7)
                    .map(|x| {
                        chamber
                            .iter()
                            .filter(|coords| coords.0 == x)
                            .map(|(_, y)| y)
                            .max()
                            .unwrap_or(&0)
                    })
                    .min()
                    .unwrap_or(&0);

                if cutoff > 0 {
                    chamber.retain(|(_, y)| *y >= cutoff);

                    // Shift all the rocks down
                    chamber = chamber.into_iter().map(|(x, y)| (x, y - cutoff)).collect();

                    total_height += cutoff as u64;

                    if !use_cache {
                        if let Some((previous_rocks, previous_height)) =
                            cache.insert((index, chamber.clone()), (rocks, total_height))
                        {
                            let rocks_gained = rocks - previous_rocks;
                            let height_gained = total_height - previous_height;

                            total_height = previous_height
                                + ((count - previous_rocks) / rocks_gained) * height_gained;
                            rocks = count - ((count - previous_rocks) % rocks_gained);
                            use_cache = true
                        };
                    }
                }

                let height = *chamber.iter().map(|(_, y)| y).max().unwrap_or(&0);

                if rocks + 1 >= count {
                    return total_height + height as u64 + 1;
                }

                // Spawn a new rock
                rocks += 1;
                rock = Rock::new((rocks % 5) as usize, height + 1);
            }
        }
    }
}

impl Solution for Day17 {
    type F = u64;
    type S = u64;

    fn name(&self) -> &'static str {
        "Day 17"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day17.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        Ok(Day17::simulate(input, 2022))
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        Ok(Day17::simulate(input, 1000000000000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn it_solves_part1() {
        assert_eq!(Day17.part_1(INPUT), Ok(3068));
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(Day17.part_2(INPUT), Ok(1514285714288));
    }
}
