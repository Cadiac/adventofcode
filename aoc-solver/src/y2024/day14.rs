use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

type Coords = (i64, i64);
const DIRECTIONS: [Coords; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug, PartialEq)]
struct Robot {
    v: Coords,
    pos: Coords,
}

fn parse(input: &str) -> Result<Vec<Robot>, AocError> {
    let machines = input
        .trim()
        .lines()
        .map(|robot_input| {
            robot_input
                .strip_prefix("p=")
                .and_then(|i| i.split_once(" v="))
                .and_then(|(p_input, v_input)| {
                    let p = p_input.split_once(",")?;
                    let v = v_input.split_once(",")?;

                    let pos = (p.0.parse::<i64>().ok()?, p.1.parse::<i64>().ok()?);
                    let v = (v.0.parse::<i64>().ok()?, v.1.parse::<i64>().ok()?);

                    Some(Robot { pos, v })
                })
                .ok_or_else(|| AocError::parse(robot_input, "Invalid robot input"))
        })
        .try_collect()?;

    Ok(machines)
}

fn predict(robot: &Robot, width: u64, height: u64, seconds: i64) -> Coords {
    let x = (robot.pos.0 + seconds * robot.v.0).rem_euclid(width as i64);
    let y = (robot.pos.1 + seconds * robot.v.1).rem_euclid(height as i64);
    (x, y)
}

fn predict_and_check_safety(robots: Vec<Robot>, width: u64, height: u64) -> u64 {
    let middle_x = width as i64 / 2;
    let middle_y = height as i64 / 2;

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    robots
        .iter()
        .map(|robot| predict(robot, width, height, 100))
        .for_each(|(x, y)| match (x.cmp(&middle_x), y.cmp(&middle_y)) {
            (Ordering::Less, Ordering::Less) => a += 1,
            (Ordering::Less, Ordering::Greater) => b += 1,
            (Ordering::Greater, Ordering::Less) => c += 1,
            (Ordering::Greater, Ordering::Greater) => d += 1,
            _ => {}
        });

    a * b * c * d
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn lcm_of_vector(numbers: Vec<i64>) -> i64 {
    numbers
        .iter()
        .fold(1, |acc, &num| lcm(acc, std::cmp::max(num, 1)))
}

fn repeats(robot: &Robot, width: u64, height: u64) -> i64 {
    let x_repeat = width as i64 / gcd(robot.v.0, width as i64);
    let y_repeat = height as i64 / gcd(robot.v.1, height as i64);

    lcm(x_repeat, y_repeat)
}

fn repeats_every(robots: &[Robot], width: u64, height: u64) -> i64 {
    let robot_repeats = robots
        .iter()
        .map(|robot| repeats(robot, width, height))
        .collect::<Vec<_>>();

    lcm_of_vector(robot_repeats)
}

fn flood_fill(start: &Coords, visited: &mut HashSet<Coords>, robots: &HashSet<Coords>) -> usize {
    let mut region = HashSet::new();
    let mut stack = vec![*start];

    while let Some(pos) = stack.pop() {
        if !visited.insert(pos) {
            continue;
        }

        region.insert(pos);

        for (dx, dy) in DIRECTIONS {
            let next = (pos.0 + dx, pos.1 + dy);

            if robots.contains(&next) {
                stack.push(next);
            }
        }
    }

    region.len()
}

pub struct Day14;
impl Solution for Day14 {
    type A = u64;
    type B = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day14.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let robots = parse(input)?;
        let width = 101;
        let height = 103;

        let safety_rating = predict_and_check_safety(robots, width, height);

        Ok(safety_rating)
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let robots = parse(input)?;

        let width = 101;
        let height = 103;

        // Turns out this logic was more or less useless - all the robot velocities
        // are less than the (prime) width & height, so the repeated positions are just
        // determined by lcm(101, 103). But this logic would find the repeating patterns
        // even if some robots were moving faster than the size of the area and
        // possibly teleporting more than once per second.
        let loops_after = repeats_every(&robots, width, height);

        // Assume that the christmas tree probably has a large continuous area,
        // which normal random states won't have.
        let (seconds, _largest) = (0..loops_after)
            .map(|seconds| {
                let mut visited: HashSet<Coords> = HashSet::new();
                let predicted: HashSet<Coords> = robots
                    .iter()
                    .map(|robot| predict(robot, width, height, seconds))
                    .collect();

                let largest_continuous = predicted
                    .iter()
                    .filter_map(|start| {
                        (!visited.contains(start)).then_some(flood_fill(
                            start,
                            &mut visited,
                            &predicted,
                        ))
                    })
                    .max()
                    .unwrap_or(0);

                (seconds, largest_continuous)
            })
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .ok_or_else(|| AocError::logic("No continuous areas?"))?;

        Ok(seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_input() {
        assert_eq!(
            parse(
                "p=0,4 v=3,-3\n\
                 p=6,3 v=-1,-3"
            ),
            Ok(vec![
                Robot {
                    pos: (0, 4),
                    v: (3, -3),
                },
                Robot {
                    pos: (6, 3),
                    v: (-1, -3),
                },
            ])
        )
    }

    #[test]
    fn it_predicts_correctly() {
        let robot = Robot {
            pos: (2, 4),
            v: (2, -3),
        };

        assert_eq!(predict(&robot, 11, 7, 1), (4, 1));
        assert_eq!(predict(&robot, 11, 7, 2), (6, 5));
        assert_eq!(predict(&robot, 11, 7, 3), (8, 2));
        assert_eq!(predict(&robot, 11, 7, 4), (10, 6));
    }

    #[test]
    fn it_solves_part1_example_1() {
        let robots = parse(
            "p=0,4 v=3,-3\n\
                 p=6,3 v=-1,-3\n\
                 p=10,3 v=-1,2\n\
                 p=2,0 v=2,-1\n\
                 p=0,0 v=1,3\n\
                 p=3,0 v=-2,-2\n\
                 p=7,6 v=-1,-3\n\
                 p=3,0 v=-1,-2\n\
                 p=9,3 v=2,3\n\
                 p=7,3 v=-1,2\n\
                 p=2,4 v=2,-3\n\
                 p=9,5 v=-3,-3",
        )
        .unwrap();

        assert_eq!(predict_and_check_safety(robots, 11, 7), 12);
    }

    #[test]
    #[ignore]
    fn it_solves_part2() {
        assert_eq!(Day14.part_2(Day14.default_input()), Ok(7892));
    }
}
