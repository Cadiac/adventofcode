use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

use crate::solution::{AocError, Solution};

pub struct Day24;

struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

fn parse(input: &str) -> Result<Vec<Hailstone>, AocError> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" @ ").ok_or(AocError::parse(
                line,
                "Missing position and velocity groups",
            ))?;

            let (x, y, z) = position
                .split(",")
                .collect_tuple()
                .ok_or(AocError::parse(position, "Missing position"))?;

            let (vx, vy, vz) = velocity
                .split(",")
                .collect_tuple()
                .ok_or(AocError::parse(velocity, "Missing position"))?;

            Ok(Hailstone {
                position: Vec3 {
                    x: parse_number(x)?,
                    y: parse_number(y)?,
                    z: parse_number(z)?,
                },
                velocity: Vec3 {
                    x: parse_number(vx)?,
                    y: parse_number(vy)?,
                    z: parse_number(vz)?,
                },
            })
        })
        .try_collect()
}

fn parse_number(number: &str) -> Result<i64, AocError> {
    number
        .trim()
        .parse()
        .map_err(|_| AocError::parse(number, "Error parsing number"))
}

fn solve_linear_equations_2(
    (a1, b1, c1): (f64, f64, f64),
    (a2, b2, c2): (f64, f64, f64),
) -> Option<(f64, f64)> {
    let det = a1 * b2 - a2 * b1;

    if det == 0.0 {
        None
    } else {
        let x = (b1 * c2 - b2 * c1) / det;
        let y = (c1 * a2 - c2 * a1) / det;
        Some((x, y))
    }
}

fn solve_linear_equations_3(
    (a1, b1, c1, d1): (f64, f64, f64, f64),
    (a2, b2, c2, d2): (f64, f64, f64, f64),
    (a3, b3, c3, d3): (f64, f64, f64, f64),
) -> Option<(f64, f64, f64)> {
    let det = a1 * (b2 * c3 - b3 * c2) - b1 * (a2 * c3 - a3 * c2) + c1 * (a2 * b3 - a3 * b2);

    if det == 0.0 {
        None
    } else {
        let det_x = d1 * (b2 * c3 - b3 * c2) - b1 * (d2 * c3 - d3 * c2) + c1 * (d2 * b3 - d3 * b2);
        let det_y = a1 * (d2 * c3 - d3 * c2) - d1 * (a2 * c3 - a3 * c2) + c1 * (a2 * d3 - a3 * d2);
        let det_z = a1 * (b2 * d3 - b3 * d2) - b1 * (a2 * d3 - a3 * d2) + d1 * (a2 * b3 - a3 * b2);

        let x = det_x / det;
        let y = det_y / det;
        let z = det_z / det;

        Some((x, y, z))
    }
}

fn find_coefficients(hail: &Hailstone) -> (f64, f64, f64) {
    let a = hail.velocity.y;
    let b = -hail.velocity.x;
    let c = hail.velocity.x * hail.position.y - hail.velocity.y * hail.position.x;

    (a as f64, b as f64, c as f64)
}

fn find_intersection(
    (a1, b1, c1, d1): (i64, i64, i64, i64),
    (x0, y0, z0): (i64, i64, i64),
    (vx, vy, vz): (i64, i64, i64),
) -> Option<(i64, i64, i64)> {
    // Calculate the parameter t at the point of intersection
    let numerator = -(a1 * x0 + b1 * y0 + c1 * z0 + d1);
    let denominator = a1 * vx + b1 * vy + c1 * vz;

    if denominator == 0 {
        // The line is parallel to the plane (or lies within it)
        return None;
    }

    if numerator % denominator != 0 {
        // t is not an integer, hence no integer intersection point
        return None;
    }

    let t = numerator / denominator;

    // Calculate the intersection point
    let x = x0 + t * vx;
    let y = y0 + t * vy;
    let z = z0 + t * vz;

    // Check if z-coordinate is greater than 0
    if z > 0 {
        Some((x, y, z))
    } else {
        None
    }
}

fn intersects(a: &Hailstone, b: &Hailstone, (min, max): (f64, f64)) -> bool {
    println!(
        "Hailstone A: {}, {}, velocity: {}, {}",
        a.position.x, a.position.y, a.velocity.x, a.velocity.y
    );
    println!(
        "Hailstone B: {}, {}, velocity: {}, {}",
        b.position.x, b.position.y, b.velocity.x, b.velocity.y
    );

    let coefficients_1 = find_coefficients(a);
    let coefficients_2 = find_coefficients(b);

    let (x, y) = match solve_linear_equations_2(coefficients_1, coefficients_2) {
        Some(intersection_point) => intersection_point,
        None => {
            println!("No intersection");
            return false;
        }
    };

    let is_within_area = x >= min && y >= min && x <= max && y <= max;

    let a_in_future = if coefficients_1.0 > 0.0 {
        (a.position.y as f64) < y
    } else {
        (a.position.y as f64) > y
    };

    let b_in_future = if coefficients_2.0 > 0.0 {
        (b.position.y as f64) < y
    } else {
        (b.position.y as f64) > y
    };

    let is_in_future = a_in_future && b_in_future;

    println!(
        "Intersection at {}, {} - area: {}, future: {} ({}, {})",
        x, y, is_within_area, is_in_future, a_in_future, b_in_future
    );

    is_within_area && is_in_future
}

fn print(hailstone: &Hailstone) {
    let vx = hailstone.velocity.x; // A
    let vy = hailstone.velocity.y; // B
    let x = hailstone.position.x; // a
    let y = hailstone.position.y; // b

    println!("{vy}x - {vx}y - {y}X + {x}Y - {} + {}", x * vy, vx * y)
}

impl Solution for Day24 {
    type A = u64;
    type B = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day24.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let hailstones = parse(input)?;

        let bounds = (200000000000000., 400000000000000.);
        // let bounds = (7., 27.);

        let intersections = hailstones
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| intersects(a, b, bounds))
            .count() as u64;

        Ok(intersections)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let hailstones = parse(input)?;

        for hailstone in hailstones {
            print(&hailstone)
        }

        todo!();
    }
}

// 19, 13, 30 @ -2,  1, -2

// a = 19
// b = 13
// c = 30

// A = -2
// B = 1
// C = -2

// 18, 19, 22 @ -1, -1, -2

// a = 18
// b = 19
// c = 22

// A = -1
// B = -1
// C = -2

// -- 20, 25, 34 @ -2, -2, -4
// -- 12, 31, 28 @ -1, -2, -1
// -- 20, 19, 15 @  1, -5, -3

// 1x + 2y - 13X + 19Y - 19 - 26
// -1x + 1y - 19X + 18Y + 18 - 19
// -2x + 2y - 25X + 20Y + 40 - 50
// -2x + 1y - 31X + 12Y + 24 - 31
// -5x - 1y - 19X + 20Y + 100 + 19

// 1x + 2y - 13X + 19Y - 19 - 26 = -1x + 1y - 19X + 18Y + 18 - 19
// -1x + 1y - 19X + 18Y + 18 - 19 = -2x + 2y - 25X + 20Y + 40 - 50
// -2x + 2y - 25X + 20Y + 40 - 50 = -2x + 1y - 31X + 12Y + 24 - 31
// -2x + 1y - 31X + 12Y + 24 - 31 = -5x - 1y - 19X + 20Y + 100 + 19

// x = 24 and X = -3 and y = 13 and Y = 1

// (c - z)/(Z - C) = (a - x)/(X - A)

// (30 - z)/(Z + 2) = (19 - 24)/(-3 + 2)
// (22 - z)/(Z + 2) = (18 - 24)/(-3 + 1)

// -21x - 54y - 201349632539530X + 246694783951603Y - -5180590462983663 + 10872880157134620 = 7x - 77y - 131993821472398X + 220339749104883Y - 1542378243734181 + 10163524253374646
// 7x - 77y - 131993821472398X + 220339749104883Y - 1542378243734181 + 10163524253374646 = 84x - 238y - 225554040514665X + 148729713759711Y - 12493295955815724 + 53681861642490270
// 84x - 238y - 225554040514665X + 148729713759711Y - 12493295955815724 + 53681861642490270 = -116x - 57y - 277335413285770X + 243519011458151Y - -28248205329145516 + 15808118557288890
// -116x - 57y - 277335413285770X + 243519011458151Y - -28248205329145516 + 15808118557288890 = -59x - 171y - 225367189590686X + 143332267182217Y - -8456603763750803 + 38537789420007306

// x = 270392223533307 and X = 26 and y = 463714142194110 and Y = -331

// a = 246694783951603
// b = 201349632539530
// c = 307741668306846

// A = 54
// B = -21
// C = 12

// (c - z)/(Z - C) = (a - x)/(X - A)

// (307741668306846 - z)/(Z - 12) = (246694783951603 - 270392223533307)/(26 - 54)

// a = 220339749104883
// b = 131993821472398
// c = 381979584524072

// A = 77
// B = 7
// C = -58

// (381979584524072 - z)/(Z + 58) = (220339749104883 - 270392223533307)/(26 - 77)

// z = 273041846062208 and Z = 53

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const EXAMPLE_INPUT: &str =
        "19, 13, 30 @ -2,  1, -2\n\
         18, 19, 22 @ -1, -1, -2\n\
         20, 25, 34 @ -2, -2, -4\n\
         12, 31, 28 @ -1, -2, -1\n\
         20, 19, 15 @  1, -5, -3\n";

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day24.part_1(EXAMPLE_INPUT), Ok(2));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day24.part_2(EXAMPLE_INPUT), Ok(47));
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(Day24.part_2(Day24.default_input()), Ok(47));
    }
}
