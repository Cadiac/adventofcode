use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::solution::{AocError, Solution};

pub struct Day08;

fn parse(input: &str) -> Vec<(Vec<HashSet<char>>, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ");

            let unique_signal_patterns = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|pattern| pattern.chars().collect())
                .collect();

            let four_digit_output_value = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|pattern| pattern.chars().sorted().collect())
                .collect();

            (unique_signal_patterns, four_digit_output_value)
        })
        .collect()
}

fn signal_to_decimal_mappings(unique_signal_patterns: Vec<HashSet<char>>) -> HashMap<String, u32> {
    //  0000     ....     0000     0000     ....     0000     0000     0000     0000     0000
    // 1    2   .    2   .    2   .    2   1    2   1    .   1    .   .    2   1    2   1    2
    // 1    2   .    2   .    2   .    2   1    2   1    .   1    .   .    2   1    2   1    2
    //  ....     ....     3333     3333     3333     3333     3333     ....     3333     3333
    // 4    5   .    5   4    .   .    5   .    5   .    5   4    5   .    5   4    5   .    5
    // 4    5   .    5   4    .   .    5   .    5   .    5   4    5   .    5   4    5   .    5
    //  6666     ....     6666     6666     ....     6666     6666     ....     6666     6666
    // len: 6   len: 2   len: 5   len: 5   len: 4   len: 5   len: 6   len: 3   len: 7   len: 6

    let mut numbers_by_signal: HashMap<String, u32> = HashMap::new();

    // Number 1
    let segments_1 = unique_signal_patterns
        .iter()
        .find(|pattern| pattern.len() == 2)
        .unwrap()
        .clone();
    numbers_by_signal.insert(segments_1.iter().sorted().collect(), 1);

    // Number 7
    numbers_by_signal.insert(
        unique_signal_patterns
            .iter()
            .find(|pattern| pattern.len() == 3)
            .unwrap()
            .iter()
            .sorted()
            .collect(),
        7,
    );

    // Number 4
    let segments_4 = unique_signal_patterns
        .iter()
        .find(|pattern| pattern.len() == 4)
        .unwrap()
        .clone();

    numbers_by_signal.insert(segments_4.iter().sorted().collect(), 4);

    // Number 8
    numbers_by_signal.insert(
        unique_signal_patterns
            .iter()
            .find(|pattern| pattern.len() == 7)
            .unwrap()
            .iter()
            .sorted()
            .collect(),
        8,
    );

    let mut segments_5: HashSet<char> = HashSet::new();
    let mut segments_3: HashSet<char> = HashSet::new();

    // Numbers 2, 3 or 5
    for pattern in unique_signal_patterns
        .iter()
        .filter(|pattern| pattern.len() == 5)
    {
        let is_5 = pattern.intersection(&segments_4).count() == 3
            && pattern.intersection(&segments_1).count() == 1;
        let is_2 = !is_5 && pattern.intersection(&segments_1).count() == 1;

        if is_2 {
            numbers_by_signal.insert(pattern.iter().sorted().collect(), 2);
        } else if is_5 {
            segments_5 = pattern.clone();
            numbers_by_signal.insert(pattern.iter().sorted().collect(), 5);
        } else {
            segments_3 = pattern.clone();
            numbers_by_signal.insert(pattern.iter().sorted().collect(), 3);
        }
    }

    // one of 0, 6, or 9
    for pattern in unique_signal_patterns
        .iter()
        .filter(|pattern| pattern.len() == 6)
    {
        let is_6 = pattern.intersection(&segments_5).count() == 5
            && pattern.intersection(&segments_1).count() == 1;
        let is_9 = !is_6 && pattern.intersection(&segments_3).count() == 5;

        if is_6 {
            numbers_by_signal.insert(pattern.iter().sorted().collect(), 6);
        } else if is_9 {
            numbers_by_signal.insert(pattern.iter().sorted().collect(), 9);
        } else {
            // The pattern is 0
            numbers_by_signal.insert(pattern.iter().sorted().collect(), 0);
        }
    }

    numbers_by_signal
}

impl Solution for Day08 {
    type A = usize;
    type B = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day08.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let displays = parse(input);

        let sum = displays
            .into_iter()
            .map(|(_unique_signal_patterns, four_digit_output_values)| {
                four_digit_output_values
                    .into_iter()
                    .filter(|output| {
                        output.len() == 7
                            || output.len() == 3
                            || output.len() == 2
                            || output.len() == 4
                    })
                    .count()
            })
            .sum();

        Ok(sum)
    }

    fn part_2(&self, input: &str) -> Result<u32, AocError> {
        let displays = parse(input);

        let sum = displays
            .into_iter()
            .map(|(unique_signal_patterns, four_digit_output_value)| -> u32 {
                let signal_mappings = signal_to_decimal_mappings(unique_signal_patterns);

                four_digit_output_value
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(factor, output)| {
                        let numeric_output = signal_mappings.get(output).unwrap();

                        numeric_output * 10u32.pow(factor as u32)
                    })
                    .sum()
            })
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day08.part_1(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
             edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
             fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
             fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
             aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
             fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
             dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
             bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
             egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
             gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"), Ok(26));
    }

    #[test]
    fn it_solves_part2_simple_example() {
        assert_eq!(Day08.part_2(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), Ok(5353));
    }

    #[test]
    fn it_solves_part2_full_example() {
        assert_eq!(Day08.part_2(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
             edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
             fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
             fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
             aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
             fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
             dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
             bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
             egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
             gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"), Ok(61229));
    }
}
