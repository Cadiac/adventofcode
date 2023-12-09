use crate::solution::{AocError, Solution};
use num::complex::Complex;
use std::collections::HashMap;

type Operations<'a> = HashMap<&'a str, Operation<'a>>;

#[derive(Clone, Debug)]
enum Operation<'a> {
    Value(Complex<f64>),
    Equation {
        left: &'a str,
        right: &'a str,
        op: fn(Complex<f64>, Complex<f64>) -> Complex<f64>,
    },
}

pub struct Day21;

impl Day21 {
    fn parse(input: &str) -> Result<Operations, AocError> {
        let mut monkeys = HashMap::new();

        for line in input.lines() {
            let (name, operation) = line
                .split_once(": ")
                .ok_or_else(|| AocError::parse(line, "split"))?;

            let operation = if !operation.contains(' ') {
                let value = operation
                    .parse::<f64>()
                    .map_err(|err| AocError::parse(operation, err))?;

                Operation::Value(Complex::new(value, 0.0))
            } else {
                let mut iter = operation.split_ascii_whitespace();

                match (iter.next(), iter.next(), iter.next()) {
                    (Some(left), Some(operator), Some(right)) => {
                        let op = match operator {
                            "+" => |a, b| a + b,
                            "-" => |a, b| a - b,
                            "*" => |a, b| a * b,
                            "/" => |a, b| a / b,
                            _ => return Err(AocError::parse(operator, "unknown operator")),
                        };
                        Operation::Equation { left, right, op }
                    }
                    _ => return Err(AocError::parse(operation, "unknown operation")),
                }
            };

            monkeys.insert(name, operation);
        }

        Ok(monkeys)
    }

    fn reduce(mut operations: Operations<'_>) -> Operations<'_> {
        let mut is_reduced = false;

        while !is_reduced {
            is_reduced = true;

            let keys: Vec<_> = operations.keys().cloned().collect();
            for name in keys {
                if let Some(Operation::Equation { left, right, op }) = operations.get(&name) {
                    if let (Some(Operation::Value(left_val)), Some(Operation::Value(right_val))) =
                        (operations.get(*left), operations.get(*right))
                    {
                        operations.insert(name, Operation::Value(op(*left_val, *right_val)));
                        is_reduced = false;
                    }
                };
            }
        }

        operations
    }
}

impl Solution for Day21 {
    type A = i64;
    type B = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day21.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let monkeys = Day21::parse(input)?;
        let reduced = Day21::reduce(monkeys);

        match reduced.get("root") {
            Some(Operation::Value(value)) => Ok(value.re as i64),
            _ => Err(AocError::logic("impossible to solve")),
        }
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let mut monkeys = Day21::parse(input)?;

        match monkeys.get_mut("humn") {
            Some(Operation::Value(humn)) => {
                // Set humn to "0 + i" and solve the "i" later from the reduced equation
                *humn = Complex::new(0.0, 1.0);
            }
            _ => return Err(AocError::logic("no humn")),
        };

        let (left, right) = match monkeys.get("root") {
            Some(Operation::Equation { left, right, .. }) => (*left, *right),
            _ => return Err(AocError::logic("no root")),
        };

        let reduced = Day21::reduce(monkeys);

        match (reduced.get(left), reduced.get(right)) {
            (Some(Operation::Value(left)), Some(Operation::Value(right))) => {
                // The side with imaginary part contained "humn" originally
                if left.im == 0.0 {
                    Ok(((left.re - right.re) / right.im) as i64)
                } else if right.im == 0.0 {
                    Ok(((right.re - left.re) / left.im) as i64)
                } else {
                    // The input didn't actually contain this, but if both sides of the
                    // equation depended on "humn" this would still solve it
                    Ok(((left.re - right.re) / (right.im - left.im)) as i64)
                }
            }
            _ => Err(AocError::logic("failed reduce")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn\n\
        dbpl: 5\n\
        cczh: sllz + lgvd\n\
        zczc: 2\n\
        ptdq: humn - dvpt\n\
        dvpt: 3\n\
        lfqf: 4\n\
        humn: 5\n\
        ljgn: 2\n\
        sjmn: drzm * dbpl\n\
        sllz: 4\n\
        pppw: cczh / lfqf\n\
        lgvd: ljgn * ptdq\n\
        drzm: hmdt - zczc\n\
        hmdt: 32";

    #[test]
    fn it_solves_part1() {
        assert_eq!(Day21.part_1(INPUT), Ok(152));
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(Day21.part_2(INPUT), Ok(301));
    }
}
