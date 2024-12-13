use itertools::Itertools;

use crate::solution::{AocError, Solution};

#[derive(Debug, PartialEq)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    price: (i64, i64),
}

fn parse(input: &str) -> Result<Vec<Machine>, AocError> {
    let machines = input
        .trim()
        .split("\n\n")
        .map(|machine_input| {
            let (a_input, b_input, price_input) = machine_input
                .lines()
                .next_tuple()
                .ok_or_else(|| AocError::parse(machine_input, "Invalid machine"))?;

            let a = a_input
                .strip_prefix("Button A: X+")
                .and_then(|a| a.split_once(", Y+"))
                .and_then(|(x_str, y_str)| {
                    let x = x_str.parse::<i64>().ok()?;
                    let y = y_str.parse::<i64>().ok()?;
                    Some((x, y))
                })
                .ok_or_else(|| AocError::parse(a_input, "Invalid button A"))?;

            let b = b_input
                .strip_prefix("Button B: X+")
                .and_then(|a| a.split_once(", Y+"))
                .and_then(|(x_str, y_str)| {
                    let x = x_str.parse::<i64>().ok()?;
                    let y = y_str.parse::<i64>().ok()?;
                    Some((x, y))
                })
                .ok_or_else(|| AocError::parse(b_input, "Invalid button B"))?;

            let price = price_input
                .strip_prefix("Prize: X=")
                .and_then(|a| a.split_once(", Y="))
                .and_then(|(x_str, y_str)| {
                    let x = x_str.parse::<i64>().ok()?;
                    let y = y_str.parse::<i64>().ok()?;
                    Some((x, y))
                })
                .ok_or_else(|| AocError::parse(b_input, "Invalid price"))?;

            Ok(Machine { a, b, price })
        })
        .try_collect()?;

    Ok(machines)
}

fn find_fewest_tokens(
    Machine { mut price, a, b }: &Machine,
    fix_unit_conversion_error: bool,
) -> Option<i64> {
    if fix_unit_conversion_error {
        price = (price.0 + 10000000000000, price.1 + 10000000000000)
    }

    // Solving `a_presses` and `b_presses`
    // from a system of two equations, accepting only integer solutions:
    // a_presses * a.x + b_presses * b.x = price.x
    // a_presses * a.y + b_presses * b.y = price.y

    let dividend_b = price.1 * a.0 - a.1 * price.0;
    let divisor_b = a.0 * b.1 - b.0 * a.1;

    let b_presses = dividend_b / divisor_b;
    let a_presses = (price.0 - b_presses * b.0) / a.0;

    let is_integer_a = (price.0 - b_presses * b.0) % a.0 == 0;
    let is_integer_b = dividend_b % divisor_b == 0;

    (is_integer_a && is_integer_b).then_some(3 * a_presses + b_presses)
}

pub struct Day13;
impl Solution for Day13 {
    type A = i64;
    type B = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day13.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let machines = parse(input)?;

        let fewest = machines
            .iter()
            .filter_map(|machine| find_fewest_tokens(machine, false))
            .sum();

        Ok(fewest)
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let machines = parse(input)?;

        let fewest = machines
            .iter()
            .filter_map(|machine| find_fewest_tokens(machine, true))
            .sum();

        Ok(fewest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_input() {
        assert_eq!(
            parse(
                "Button A: X+94, Y+34\n\
                 Button B: X+22, Y+67\n\
                 Prize: X=8400, Y=5400\n\
                 \n\
                 Button A: X+26, Y+66\n\
                 Button B: X+67, Y+21\n\
                 Prize: X=12748, Y=12176"
            ),
            Ok(vec![
                Machine {
                    a: (94, 34),
                    b: (22, 67),
                    price: (8400, 5400)
                },
                Machine {
                    a: (26, 66),
                    b: (67, 21),
                    price: (12748, 12176)
                }
            ])
        )
    }

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(
            Day13.part_1(
                "Button A: X+94, Y+34\n\
                 Button B: X+22, Y+67\n\
                 Prize: X=8400, Y=5400\n\
                 \n\
                 Button A: X+26, Y+66\n\
                 Button B: X+67, Y+21\n\
                 Prize: X=12748, Y=12176\n\
                 \n\
                 Button A: X+17, Y+86\n\
                 Button B: X+84, Y+37\n\
                 Prize: X=7870, Y=6450\n\
                 \n\
                 Button A: X+69, Y+23\n\
                 Button B: X+27, Y+71\n\
                 Prize: X=18641, Y=10279"
            ),
            Ok(480)
        );
    }

    #[test]
    fn it_solves_part1_example_single() {
        assert_eq!(
            Day13.part_1(
                "Button A: X+94, Y+34\n\
                 Button B: X+22, Y+67\n\
                 Prize: X=8400, Y=5400"
            ),
            Ok(280)
        );
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(
            Day13.part_2(
                "Button A: X+94, Y+34\n\
                 Button B: X+22, Y+67\n\
                 Prize: X=8400, Y=5400\n\
                 \n\
                 Button A: X+26, Y+66\n\
                 Button B: X+67, Y+21\n\
                 Prize: X=12748, Y=12176\n\
                 \n\
                 Button A: X+17, Y+86\n\
                 Button B: X+84, Y+37\n\
                 Prize: X=7870, Y=6450\n\
                 \n\
                 Button A: X+69, Y+23\n\
                 Button B: X+27, Y+71\n\
                 Prize: X=18641, Y=10279"
            ),
            Ok(875318608908)
        );
    }
}
