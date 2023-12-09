use std::collections::VecDeque;

use crate::solution::{AocError, Solution};

pub struct Day18;

fn shunting_yard(input: &str, is_advanced: bool) -> VecDeque<char> {
    let mut output_queue: VecDeque<char> = VecDeque::new();
    let mut operator_stack: Vec<char> = Vec::new();

    for token in input.chars().filter(|c| !c.is_whitespace()) {
        if token.is_numeric() {
            output_queue.push_back(token);
        } else if token == '+' || token == '*' {
            if is_advanced {
                // In advanced mode addition is evaluated before multiplication
                // while there is an operator at the top of the operator stack...
                while let Some(operator) = operator_stack.last() {
                    // ...AND the operator at the top of the operator stack is not a left parenthesis
                    if *operator == '(' {
                        break;
                    }
                    // ...AND the operator at the top of the operator stack has equal precedence OR
                    // ...the operator at the top of the operator stack has greater precedence
                    else if (token == *operator) || (token == '*' && *operator == '+') {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
            } else {
                // In basic mode all operators have the same precedence here
                while operator_stack.last() != Some(&'(') && operator_stack.last().is_some() {
                    output_queue.push_back(operator_stack.pop().unwrap());
                }
            }
            operator_stack.push(token);
        } else if token == '(' {
            operator_stack.push(token);
        } else if token == ')' {
            while operator_stack.last() != Some(&'(') {
                match operator_stack.pop() {
                    Some(operator) => output_queue.push_back(operator),
                    None => panic!("mismatched parenthesis"),
                };
            }

            if operator_stack.last() == Some(&'(') {
                operator_stack.pop();
            }
        }
    }

    while let Some(operator) = operator_stack.pop() {
        output_queue.push_back(operator);
    }

    output_queue
}

fn evaluate_rpn(mut rpn: VecDeque<char>) -> i64 {
    let mut stack: Vec<i64> = Vec::new();

    while let Some(token) = rpn.pop_front() {
        if token.is_numeric() {
            stack.push(token.to_digit(10).unwrap() as i64);
        } else {
            let operand_a = stack.pop().unwrap();
            let operand_b = stack.pop().unwrap();

            match token {
                '+' => stack.push(operand_a + operand_b),
                '*' => stack.push(operand_a * operand_b),
                operator => panic!("Unknown operator {:?}", operator),
            };
        }
    }

    assert_eq!(stack.len(), 1);

    stack.pop().unwrap()
}

impl Solution for Day18 {
    type A = i64;
    type B = i64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day18.txt")
    }

    fn part_1(&self, input: &str) -> Result<i64, AocError> {
        let sum = input
            .lines()
            .map(|line| evaluate_rpn(shunting_yard(line, false)))
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<i64, AocError> {
        let sum = input
            .lines()
            .map(|line| evaluate_rpn(shunting_yard(line, true)))
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_rpn_basic_mode() {
        assert_eq!(
            shunting_yard("1 + 2 * 3 + 4 * 5 + 6", false),
            vec!['1', '2', '+', '3', '*', '4', '+', '5', '*', '6', '+']
        );
        assert_eq!(
            shunting_yard("1 + ((2 + 3) + 4 + (5 + 6))", false),
            vec!['1', '2', '3', '+', '4', '+', '5', '6', '+', '+', '+']
        );
        assert_eq!(
            shunting_yard("1 + (2 * 3)", false),
            vec!['1', '2', '3', '*', '+']
        );
    }

    #[test]
    fn it_parses_rpn_advanced_mode() {
        assert_eq!(
            shunting_yard("1 * ((2 * 3) + (4 * 5 + 6))", true),
            vec!['1', '2', '3', '*', '4', '5', '6', '+', '*', '+', '*']
        );
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day18.part_1("1 + 2 * 3 + 4 * 5 + 6"), Ok(71));
        assert_eq!(Day18.part_1("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51));
        assert_eq!(Day18.part_1("2 * 3 + (4 * 5)"), Ok(26));
        assert_eq!(Day18.part_1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(437));
        assert_eq!(
            Day18.part_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(12240)
        );
        assert_eq!(
            Day18.part_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(13632)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day18.part_2("1 + 2 * 3 + 4 * 5 + 6"), Ok(231));
        assert_eq!(Day18.part_2("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51));
        assert_eq!(Day18.part_2("2 * 3 + (4 * 5)"), Ok(46));
        assert_eq!(Day18.part_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(1445));
        assert_eq!(
            Day18.part_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(669060)
        );
        assert_eq!(
            Day18.part_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(23340)
        );
    }
}
