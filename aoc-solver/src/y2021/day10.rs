use nom::{
    branch::alt,
    character::complete::char,
    combinator::{cut, opt},
    error::{VerboseError, VerboseErrorKind::Char},
    multi::many1,
    sequence::preceded,
    Err, IResult,
};

use crate::solution::{AocError, Solution};

enum ParseResult {
    Ok,
    Incomplete { expected: char },
    Corrupted { illegal: char },
}

pub struct Day10;

fn parse(input: &str) -> ParseResult {
    match chunks(input) {
        Err(Err::Incomplete(_needed)) => unimplemented!(),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            let (unhandled, error_kind) = e.errors.first().unwrap().clone();
            let expected = match error_kind {
                Char(expected_char) => expected_char,
                _ => unimplemented!(),
            };
            if let Some(illegal) = unhandled.chars().next() {
                ParseResult::Corrupted { illegal }
            } else {
                ParseResult::Incomplete { expected }
            }
        }
        Ok(_) => ParseResult::Ok,
    }
}

fn chunks(input: &str) -> IResult<&str, Vec<char>, VerboseError<&str>> {
    let (unhandled, parsed) = many1(alt((
        preceded(char('('), cut(preceded(opt(chunks), char(')')))),
        preceded(char('['), cut(preceded(opt(chunks), char(']')))),
        preceded(char('{'), cut(preceded(opt(chunks), char('}')))),
        preceded(char('<'), cut(preceded(opt(chunks), char('>')))),
    )))(input)?;

    Ok((unhandled, parsed))
}

fn score_syntax_error(parsed: ParseResult) -> usize {
    match parsed {
        ParseResult::Corrupted { illegal } => match illegal {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        },
        _ => 0,
    }
}

fn score_autocomplete(completion: Vec<char>) -> usize {
    completion.iter().fold(0, |mut acc, c| {
        // Start with a total score of 0.
        // Then, for each character, multiply the total score by 5
        acc *= 5;

        // and then increase the total score by the point value given
        // for the character in the following table:
        acc += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };

        acc
    })
}

fn parse_with_autocomplete(input: &str) -> Option<Vec<char>> {
    let mut completion: Vec<char> = Vec::new();
    let mut completed_input = String::from(input);

    loop {
        match parse(completed_input.as_str()) {
            ParseResult::Incomplete { expected, .. } => {
                completed_input.push(expected);
                completion.push(expected);
            }
            ParseResult::Ok => {
                return Some(completion);
            }
            ParseResult::Corrupted { .. } => {
                // Discard the corrupted lines
                return None;
            }
        };
    }
}

impl Solution for Day10 {
    type F = usize;
    type S = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day10.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        Ok(input.lines().map(parse).map(score_syntax_error).sum())
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let mut scores: Vec<usize> = input
            .lines()
            .flat_map(parse_with_autocomplete)
            .map(score_autocomplete)
            .collect();

        scores.sort_unstable();
        let median = scores[scores.len() / 2];

        Ok(median)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day10.part_1(
                "[({(<(())[]>[[{[]{<()<>>\n\
                 [(()[<>])]({[<{<<[]>>(\n\
                 {([(<{}[<>[]}>{[]{[(<()>\n\
                 (((({<>}<{<{<>}{[]{[]{}\n\
                 [[<[([]))<([[{}[[()]]]\n\
                 [{[{({}]{}}([{[{{{}}([]\n\
                 {<[[]]>}<{[{[{[]{()[[[]\n\
                 [<(<(<(<{}))><([]([]()\n\
                 <{([([[(<>()){}]>(<<{{\n\
                 <{([{{}}[<[[[<>{}]]]>[]]"
            ),
            Ok(26397)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day10.part_2(
                "[({(<(())[]>[[{[]{<()<>>\n\
                 [(()[<>])]({[<{<<[]>>(\n\
                 {([(<{}[<>[]}>{[]{[(<()>\n\
                 (((({<>}<{<{<>}{[]{[]{}\n\
                 [[<[([]))<([[{}[[()]]]\n\
                 [{[{({}]{}}([{[{{{}}([]\n\
                 {<[[]]>}<{[{[{[]{()[[[]\n\
                 [<(<(<(<{}))><([]([]()\n\
                 <{([([[(<>()){}]>(<<{{\n\
                 <{([{{}}[<[[[<>{}]]]>[]]"
            ),
            Ok(288957)
        );
    }
}
