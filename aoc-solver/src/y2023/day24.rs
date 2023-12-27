use itertools::Itertools;
use num_bigint::BigInt;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;

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
                .split(',')
                .collect_tuple()
                .ok_or(AocError::parse(position, "Missing position"))?;

            let (vx, vy, vz) = velocity
                .split(',')
                .collect_tuple()
                .ok_or(AocError::parse(velocity, "Missing velocity"))?;

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

fn determinant(matrix: &[Vec<i64>]) -> BigInt {
    let size = matrix.len();

    if size == 1 {
        return BigInt::from(matrix[0][0]);
    }

    if size == 2 {
        return BigInt::from(matrix[0][0]) * BigInt::from(matrix[1][1])
            - BigInt::from(matrix[1][0]) * BigInt::from(matrix[0][1]);
    }

    let mut det = BigInt::zero();

    for det_column in 0..size {
        let mut submatrix = vec![vec![0; size - 1]; size - 1];

        for row in 1..size {
            let mut sub_column = 0;
            for column in 0..size {
                if column == det_column {
                    continue;
                }
                submatrix[row - 1][sub_column] = matrix[row][column];
                sub_column += 1;
            }
        }

        let submatrix_det = determinant(&submatrix);

        let sign = if det_column % 2 == 0 {
            BigInt::one()
        } else {
            -BigInt::one()
        };

        det += sign * BigInt::from(matrix[0][det_column]) * submatrix_det;
    }

    det
}

fn solve_cramers_rule(matrix: &[Vec<i64>]) -> Option<Vec<f64>> {
    // Last column of the matrix contains the constant solutions
    // a*i + b*j + c*k + d*l = e*m
    // ^-------------------^   ^-^
    //     coefficients         solution
    let size = matrix[0].len() - 1;

    let coefficients_matrix: Vec<Vec<i64>> = matrix
        .iter()
        .map(|row| row.iter().copied().take(size).collect())
        .collect();

    let det = determinant(&coefficients_matrix);

    if det.is_zero() {
        return None;
    }

    let det = det.to_f64().expect("Overflow at determinant");

    let mut results = Vec::new();

    for column in 0..size {
        let mut det_matrix = coefficients_matrix.clone();

        for row in 0..size {
            det_matrix[row][column] = matrix[row][size];
        }

        let det_column = determinant(&det_matrix)
            .to_f64()
            .expect("Overflow at column determinant");

        results.push(det_column / det);
    }

    Some(results)
}

fn check_intersection(a: &Hailstone, b: &Hailstone, (min, max): (f64, f64)) -> bool {
    // System of two equations and two unknowns (t_a and t_b)
    // x_a + vx_a * t_a = x_b + vx_b * tb
    // y_a + vy_a * t_a = y_b + vy_b * tb

    // Represent this system as a coefficients matrix and solve it using Cramer's Rule
    // vx_a * ta - vx_b * tb = x_a - x_b
    // ^--^        ^--^        ^-------^
    //  A           B           C
    let matrix = vec![
        vec![a.velocity.x, -b.velocity.x, b.position.x - a.position.x],
        vec![a.velocity.y, -b.velocity.y, b.position.y - a.position.y],
    ];

    let (t_a, t_b) = match solve_cramers_rule(&matrix) {
        Some(solution) => (solution[0], solution[1]),
        None => return false,
    };

    // Intersection coordinates
    let x = a.position.x as f64 + a.velocity.x as f64 * t_a;
    let y = a.position.y as f64 + a.velocity.y as f64 * t_a;

    let is_within_area = x >= min && y >= min && x <= max && y <= max;
    let is_in_future = t_a > 0.0 && t_b > 0.0;

    is_within_area && is_in_future
}

fn count_intersections_within_bounds(hailstones: Vec<Hailstone>, bounds: (f64, f64)) -> u64 {
    hailstones
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| check_intersection(a, b, bounds))
        .count() as u64
}

fn find_position(hailstones: Vec<Hailstone>) -> Result<Vec3, AocError> {
    // Take a hailstone n and the position (x,y,z) where we're
    // throwing the rock with (vx,vy,vz) velocity. By treating
    // each axis separately, solve t on X-axis
    // X + t*VX = x_n + t*vx_n
    // t = (x_n - X) / (VX - vx_n)

    // Repeat this for the other axis
    // t = (y_n - Y) / (VY - vy_n)
    // t = (z_n - Z) / (VZ - vz_n)

    // From the equation of X-axis substitute t with one
    // of these other two axis, say Y-axis.
    // (x_n - X) / (VX - vx_n) = (y_n - Y) / (VY - vy_n)
    // (x_n - X) * (VY - vy_n) = (y_n - Y) * (VX - vx_n)
    // vy_n * X - vx_n * Y - y_n * VX + x_n * VY - x_n * vy_n + vx_n * y_n = X * VY - VX * X
    // ^-----------------------------LHS---------------------------------^   ^-----RHS-----^

    // RHS of the equation doens't contain anything specific to this hailstone.
    // If we now take another hailstone and substitute X * VY - VX * X with its LHS, we get
    // vy_0 * X - vx_0 * Y - y_0 * VX + x_0 * VY - x_0 * vy_0 + vx_0 * y_0
    // = vy_1 * X - vx_1 * Y - y_1 * VX + x_1 * VY - x_1 * vy_1 + vx_1 * y_1
    //
    // (vy_0-vy_1)X + (vx_1-vx_0)Y + (y_1-y_0)VX + (x_0-x_1)VY = x_0 * vy_0 - vx_0 * y_0 - x_1 * vy_1 + vx_1 * y_1
    // ^---------^    ^---------^    ^-------^     ^-------^     ^-----------------------------------------------^
    //  A              B              C             D             E

    // Take a sample of five hailstones from the input, and do this for each of the pairs.
    // This results in a system of four equations with four unknowns (X, Y, VX and VY)

    // Represent this system as a 5x4 coefficients matrix and solve it using Cramer's Rule
    let matrix: Vec<Vec<i64>> = hailstones
        .iter()
        .tuple_windows()
        .take(4)
        .map(|(a, b)| {
            vec![
                (a.velocity.y - b.velocity.y),
                (b.velocity.x - a.velocity.x),
                (b.position.y - a.position.y),
                (a.position.x - b.position.x),
                (a.position.x * a.velocity.y
                    - a.velocity.x * a.position.y
                    - b.position.x * b.velocity.y
                    + b.velocity.x * b.position.y),
            ]
        })
        .collect();

    let (x, y, vx, _vy) = match solve_cramers_rule(&matrix) {
        Some(solution) => (solution[0], solution[1], solution[2], solution[3]),
        None => return Err(AocError::logic("No solution was found")),
    };

    // With these we can get the value of t for every hailstone collision.

    // Take the equation of t for Z-axis, and construct a
    // system of two equations with two unknowns (Z and VZ). Solve it.
    // t_n = (z_n - Z) / (VZ - vz_n) = (x_n - X) / (VZ - vx_n)
    // Z + t_n * VZ = z_n + t_n * vz_n

    let t0 = (hailstones[0].position.x as f64 - x) / (vx - hailstones[0].velocity.x as f64);
    let t1 = (hailstones[1].position.x as f64 - x) / (vx - hailstones[1].velocity.x as f64);

    let matrix = vec![
        vec![
            1,
            t0 as i64,
            hailstones[0].position.z + t0 as i64 * hailstones[0].velocity.z,
        ],
        vec![
            1,
            t1 as i64,
            hailstones[1].position.z + t1 as i64 * hailstones[1].velocity.z,
        ],
    ];

    let (z, _vz) = match solve_cramers_rule(&matrix) {
        Some(solution) => (solution[0], solution[1]),
        None => return Err(AocError::logic("No solution was found")),
    };

    Ok(Vec3 {
        x: x as i64,
        y: y as i64,
        z: z as i64,
    })
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
        let count = count_intersections_within_bounds(hailstones, bounds);

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let hailstones = parse(input)?;
        let position = find_position(hailstones)?;

        Ok((position.x + position.y + position.z) as u64)
    }
}

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
        let hailstones = parse(EXAMPLE_INPUT);
        assert!(hailstones.is_ok());
        let hailstones = hailstones.unwrap();
        let bounds = (7., 27.);

        assert_eq!(count_intersections_within_bounds(hailstones, bounds), 2);
    }

    #[test]
    fn it_solves_part1_real() {
        assert_eq!(Day24.part_1(Day24.default_input()), Ok(14799));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day24.part_2(EXAMPLE_INPUT), Ok(47));
    }

    #[test]
    fn it_solves_part2_real() {
        assert_eq!(Day24.part_2(Day24.default_input()), Ok(1007148211789625));
    }

    #[rustfmt::skip]
    #[test]
    fn it_calculates_determinant() {
        assert_eq!(
            determinant([
                vec![3, 7],
                vec![1,-4]].as_slice()),
            BigInt::from(-19)
        );

        assert_eq!(
            determinant([
                vec![-2,-1, 2],
                vec![ 2, 1, 4],
                vec![-3, 3,-1]].as_slice()),
            BigInt::from(54)
        );

        assert_eq!(
            determinant([
                vec![-1, 1, 4, 2],
                vec![ 2,-1, 2, 5],
                vec![ 1, 2, 3, 4],
                vec![ 3, 4,-1, 2]].as_slice()),
            BigInt::from(-26)
        )
    }
}
