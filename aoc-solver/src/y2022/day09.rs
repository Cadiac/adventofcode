use std::collections::HashSet;

use crate::{
    solution::{AocError, Solution},
    utils::Coords,
};

type Visited = HashSet<Coords<i32>>;
type Rope = Vec<Coords<i32>>;

pub struct Day09;

impl Day09 {
    fn parse(input: &str) -> Result<Vec<(Coords<i32>, usize)>, AocError> {
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_ascii_whitespace();

                let direction = match iter.next() {
                    Some("U") => Coords { x: 0, y: -1 },
                    Some("D") => Coords { x: 0, y: 1 },
                    Some("L") => Coords { x: -1, y: 0 },
                    Some("R") => Coords { x: 1, y: 0 },
                    Some(dir) => {
                        return Err(AocError::parse(line, format!("unknown direction {dir}")))
                    }
                    None => return Err(AocError::parse(line, "missing direction")),
                };

                let steps = iter
                    .next()
                    .ok_or_else(|| AocError::parse(line, "missing steps"))?
                    .parse()
                    .map_err(|err| AocError::parse(line, err))?;

                Ok((direction, steps))
            })
            .collect()
    }

    fn should_move(head: &Coords<i32>, tail: &Coords<i32>) -> Option<Coords<i32>> {
        let dist_y = i32::abs(head.y - tail.y);
        let dist_x = i32::abs(head.x - tail.x);

        // If the head is ever two steps directly up, down, left, or right from the tail,
        // the tail must also move one step in that direction
        if head.x == tail.x && dist_y > 1 {
            return Some(Coords {
                x: 0,
                y: i32::signum(head.y - tail.y),
            });
        }

        if head.y == tail.y && dist_x > 1 {
            return Some(Coords {
                x: i32::signum(head.x - tail.x),
                y: 0,
            });
        }

        // Otherwise, if the head and tail aren't touching and aren't in the same row or column,
        // the tail always moves one step diagonally to keep up
        if dist_x + dist_y > 2 {
            return Some(Coords {
                x: i32::signum(head.x - tail.x),
                y: i32::signum(head.y - tail.y),
            });
        }

        None
    }

    pub fn simulate(
        directions: Vec<(Coords<i32>, usize)>,
        mut rope: Rope,
        mut visited: HashSet<Coords<i32>>,
    ) -> (Visited, Rope) {
        let length = rope.len();

        visited.insert(rope[length - 1]);

        for (direction, steps) in directions {
            for _ in 0..steps {
                rope[0].x += direction.x;
                rope[0].y += direction.y;

                for index in 1..length {
                    if let Some(movement) = Self::should_move(&rope[index - 1], &rope[index]) {
                        rope[index].x += movement.x;
                        rope[index].y += movement.y;

                        if index == length - 1 {
                            visited.insert(rope[index]);
                        }
                    }
                }
            }
        }

        (visited, rope)
    }
}

impl Solution for Day09 {
    type F = usize;
    type S = usize;

    fn meta(&self) -> (u32, u32) {
        (9, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day09.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let directions = Self::parse(input)?;
        let rope = vec![Coords { x: 0, y: 0 }; 2];
        let (visited, _) = Self::simulate(directions, rope, HashSet::new());

        Ok(visited.len())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let directions = Self::parse(input)?;
        let rope = vec![Coords { x: 0, y: 0 }; 10];
        let (visited, _) = Self::simulate(directions, rope, HashSet::new());

        Ok(visited.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day09.part_1(
                "R 4\n\
                 U 4\n\
                 L 3\n\
                 D 1\n\
                 R 4\n\
                 D 1\n\
                 L 5\n\
                 R 2"
            ),
            Ok(13)
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day09.part_2(
                "R 4\n\
                 U 4\n\
                 L 3\n\
                 D 1\n\
                 R 4\n\
                 D 1\n\
                 L 5\n\
                 R 2"
            ),
            Ok(1)
        );
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(
            Day09.part_2(
                "R 5\n\
                 U 8\n\
                 L 8\n\
                 D 3\n\
                 R 17\n\
                 D 10\n\
                 L 25\n\
                 U 20"
            ),
            Ok(36)
        );
    }
}
