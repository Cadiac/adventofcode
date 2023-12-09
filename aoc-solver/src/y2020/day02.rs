use regex::Regex;

use crate::solution::{AocError, Solution};

pub struct Day02;

struct DatabaseRow {
    password: String,
    check_char: char,
    min: usize,
    max: usize,
}

fn validate_part1(db_row: &DatabaseRow) -> bool {
    let occurences = db_row
        .password
        .chars()
        .filter(|c| c == &db_row.check_char)
        .count();
    occurences >= db_row.min && occurences <= db_row.max
}

fn validate_part2(db_row: &DatabaseRow) -> bool {
    let first_match = db_row.password.chars().nth(db_row.min - 1).unwrap() == db_row.check_char;
    let second_match = db_row.password.chars().nth(db_row.max - 1).unwrap() == db_row.check_char;
    first_match ^ second_match
}

fn parse_input(input: &str) -> Vec<DatabaseRow> {
    let input_regex = Regex::new(r"^(\d+)-(\d+) (\S): (\S+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let capture = input_regex.captures(line).unwrap();

            DatabaseRow {
                min: capture[1].parse::<usize>().unwrap(),
                max: capture[2].parse::<usize>().unwrap(),
                password: capture[4].parse::<String>().unwrap(),
                check_char: capture[3].parse::<char>().unwrap(),
            }
        })
        .collect()
}

impl Solution for Day02 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day02.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let parsed_input = parse_input(input);
        let count = parsed_input
            .iter()
            .filter(|row| validate_part1(row))
            .count();

        Ok(count)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let parsed_input = parse_input(input);
        let count = parsed_input
            .iter()
            .filter(|row| validate_part2(row))
            .count();

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_part1_pws_correctly() {
        assert!(validate_part1(&DatabaseRow {
            password: String::from("abcde"),
            check_char: 'a',
            min: 1,
            max: 3
        }));
        assert!(!validate_part1(&DatabaseRow {
            password: String::from("cdefg"),
            check_char: 'b',
            min: 1,
            max: 3
        }));
        assert!(validate_part1(&DatabaseRow {
            password: String::from("ccccccccc"),
            check_char: 'c',
            min: 2,
            max: 9
        }))
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day02.part_1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"),
            Ok(2)
        );
    }

    #[test]
    fn it_validates_part2_pws_correctly() {
        assert!(validate_part2(&DatabaseRow {
            password: String::from("abcde"),
            check_char: 'a',
            min: 1,
            max: 3
        }));
        assert!(!validate_part2(&DatabaseRow {
            password: String::from("cdefg"),
            check_char: 'b',
            min: 1,
            max: 3
        }));
        assert!(!validate_part2(&DatabaseRow {
            password: String::from("ccccccccc"),
            check_char: 'c',
            min: 2,
            max: 9
        }))
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day02.part_2("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"),
            Ok(1)
        );
    }
}
