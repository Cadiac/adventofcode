use crate::solution::{AocError, Solution};

pub struct Day17;

type TargetArea = ((i32, i32), (i32, i32));

fn parse(input: &str) -> TargetArea {
    let input = input.lines().next().unwrap();
    let target_area: TargetArea =
        serde_scan::scan!("target area: x={}..{}, y={}..{}" <- input).unwrap();
    target_area
}

fn solve(input: &str) -> (i32, usize) {
    let target_area = parse(input);

    let ((min_x, max_x), (min_y, _max_y)) = target_area;

    // Only pick x velocities that actually reach the target area
    let x_min_v = ((((1 + 8 * min_x) as f64).sqrt() - 1f64) / 2f64) as i32 + 1;
    // Don't pick x velocities that cause us to be past the target area during the first step
    let x_max_v = max_x;

    let mut max_height = i32::MIN;
    let mut hits_count = 0;

    for x_v in x_min_v..=x_max_v {
        // Don't aim too low, that would make the probe be past the target area after first step
        let y_min_v = min_y - 1;

        // If the velocity on x axis causes the probe to stop on x-axis before
        // it leaves the target area, this means that we can aim as high as we want
        // and still go through the target area.
        // If we launch the probe up at velocity V, when it is coming back down it has
        // velocity -V at y=0, but it still lands on the y=0 coordinate momentarily.
        // During the next step from that it is moving with velocity -(V - 1), and if
        // the distance from y=0 to min_y is greater than that, it always goes through
        // the area without stopping and is useless to check.
        let y_max_v = -(min_y - 1);

        for y_v in y_min_v..=y_max_v {
            if let Some(height) = find_max_height(target_area, x_v, y_v) {
                if height > max_height {
                    max_height = height;
                }
                hits_count += 1;
            }
        }
    }

    (max_height, hits_count)
}

#[allow(clippy::comparison_chain)]
fn find_max_height(target_area: TargetArea, initial_x_v: i32, initial_y_v: i32) -> Option<i32> {
    let ((min_x, max_x), (min_y, max_y)) = target_area;
    let mut x_v = initial_x_v;
    let mut y_v = initial_y_v;

    let mut x = 0;
    let mut y = 0;
    let mut max_height = i32::MIN;

    loop {
        x += x_v;
        y += y_v;

        if y > max_height {
            max_height = y;
        }

        if x_v > 0 {
            x_v -= 1;
        } else if x_v < 0 {
            x_v += 1;
        }

        y_v -= 1;

        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            return Some(max_height);
        }

        if x > max_x || y < min_y {
            return None;
        }
    }
}

impl Solution for Day17 {
    type A = i32;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day17.txt")
    }

    fn part_1(&self, input: &str) -> Result<i32, AocError> {
        let (part_1, _) = solve(input);

        Ok(part_1)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let (_, part_2) = solve(input);

        Ok(part_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_checks_if_shot_hits() {
        assert_eq!(find_max_height(((20, 30), (-10, -5)), 7, 2), Some(3));
        assert_eq!(find_max_height(((20, 30), (-10, -5)), 6, 3), Some(6));
        assert_eq!(find_max_height(((20, 30), (-10, -5)), 9, 0), Some(0));
        assert_eq!(find_max_height(((20, 30), (-10, -5)), 17, -4), None);
        assert_eq!(find_max_height(((20, 30), (-10, -5)), 6, 9), Some(45));
    }

    #[test]
    fn it_solves_example() {
        assert_eq!(solve("target area: x=20..30, y=-10..-5"), (45, 112));
    }
}
