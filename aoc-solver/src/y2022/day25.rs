use crate::solution::{AocError, Solution};
use num::Integer;

pub struct Day25;

impl Day25 {
    fn from_snafu(snafu: &str) -> Result<i64, AocError> {
        let mut decimal = 0;

        for (exp, digit) in snafu.chars().rev().enumerate() {
            let multiplier = match digit {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                unexpected => return Err(AocError::parse(unexpected, "unknown snafu digit")),
            };

            decimal += multiplier * 5_i64.pow(exp as u32)
        }

        Ok(decimal)
    }

    fn to_snafu(mut decimal: i64) -> String {
        let mut snafu = String::new();

        while decimal > 0 {
            let (quotient, remainder) = decimal.div_rem(&5);
            match remainder {
                0 | 1 | 2 => {
                    decimal = quotient;
                    snafu = format!("{remainder}{snafu}")
                }
                3 => {
                    decimal = quotient + 1;
                    snafu = format!("={snafu}")
                }
                4 => {
                    decimal = quotient + 1;
                    snafu = format!("-{snafu}")
                }
                _ => unreachable!(),
            }
        }

        snafu
    }
}

impl Solution for Day25 {
    type F = String;
    type S = String;

    fn meta(&self) -> (u32, u32) {
        (25, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2022/day25.txt")
    }

    fn part_1(&self, input: &str) -> Result<String, AocError> {
        let mut decimal_sum = 0;

        for snafu in input.lines() {
            decimal_sum += Day25::from_snafu(snafu)?;
        }

        println!("{decimal_sum}");

        Ok(Day25::to_snafu(decimal_sum))
    }

    fn part_2(&self, _input: &str) -> Result<String, AocError> {
        Ok([
            "",
            "                               ",
            "               *               ",
            "               ^^              ",
            "              ^^o              ",
            "              o^^              ",
            "              ^^o^             ",
            "             o^^^^o            ",
            "             ^^o^^^^           ",
            "        _______||_______       ",
            "            AoC 2022           ",
        ]
        .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1=-0-2\n\
         12111\n\
         2=0=\n\
         21\n\
         2=01\n\
         111\n\
         20012\n\
         112\n\
         1=-1=\n\
         1-12\n\
         12\n\
         1=\n\
         122";

    const EXAMPLES: [(i64, &str); 28] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
        (1747, "1=-0-2"),
        (906, "12111"),
        (198, "2=0="),
        (11, "21"),
        (201, "2=01"),
        (31, "111"),
        (1257, "20012"),
        (32, "112"),
        (353, "1=-1="),
        (107, "1-12"),
        (7, "12"),
        (3, "1="),
        (37, "122"),
    ];

    #[test]
    fn it_solves_part1() {
        assert_eq!(Day25.part_1(INPUT), Ok(String::from("2=-1=0")));
    }

    #[test]
    fn it_converts_snafu_to_decimal() {
        for (decimal, snafu) in EXAMPLES {
            assert_eq!(Day25::from_snafu(snafu), Ok(decimal));
        }
    }

    #[test]
    fn it_converts_decimal_to_snafu() {
        for (decimal, snafu) in EXAMPLES {
            assert_eq!(Day25::to_snafu(decimal), snafu);
        }
    }
}
