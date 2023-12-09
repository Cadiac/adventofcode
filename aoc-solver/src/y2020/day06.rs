use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day06;

fn parse_answers_1(input: &str) -> HashSet<char> {
    let answers: HashSet<char> = input.lines().flat_map(|line| line.chars()).collect();

    answers
}

fn parse_answers_2(input: &str) -> HashSet<char> {
    let mut common_answers: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    for line in input.lines() {
        let mut single_answers: HashSet<char> = line.chars().collect();
        common_answers = common_answers
            .iter()
            .filter_map(|v| single_answers.take(v))
            .collect();
    }

    common_answers
}

impl Solution for Day06 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day06.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let sum = input.split("\n\n").map(|i| parse_answers_1(i).len()).sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let sum = input.split("\n\n").map(|i| parse_answers_2(i).len()).sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_part1_examples() {
        assert_eq!(
            parse_answers_1("abc"),
            vec!['a', 'b', 'c'].into_iter().collect()
        );
        assert_eq!(
            parse_answers_1("a\nb\nc"),
            vec!['a', 'b', 'c'].into_iter().collect()
        );
        assert_eq!(
            parse_answers_1("ab\nac"),
            vec!['a', 'b', 'c'].into_iter().collect()
        );
        assert_eq!(
            parse_answers_1("a\na\na\na"),
            vec!['a'].into_iter().collect()
        );
        assert_eq!(parse_answers_1("b"), vec!['b'].into_iter().collect());
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day06.part_1(
                "abc\n\
                 \n\
                 a\n\
                 b\n\
                 c\n\
                 \n\
                 ab\n\
                 ac\n\
                 \n\
                 a\n\
                 a\n\
                 a\n\
                 a\n\
                 \n\
                 b"
            ),
            Ok(11)
        );
    }

    #[test]
    fn it_parses_part2_examples() {
        assert_eq!(
            parse_answers_2("abc"),
            vec!['a', 'b', 'c'].into_iter().collect()
        );
        assert_eq!(parse_answers_2("a\nb\nc"), vec![].into_iter().collect());
        assert_eq!(parse_answers_2("ab\nac"), vec!['a'].into_iter().collect());
        assert_eq!(
            parse_answers_2("a\na\na\na"),
            vec!['a'].into_iter().collect()
        );
        assert_eq!(parse_answers_2("b"), vec!['b'].into_iter().collect());
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day06.part_2(
                "abc\n\
                 \n\
                 a\n\
                 b\n\
                 c\n\
                 \n\
                 ab\n\
                 ac\n\
                 \n\
                 a\n\
                 a\n\
                 a\n\
                 a\n\
                 \n\
                 b"
            ),
            Ok(6)
        );
    }
}
