extern crate nom;

use nom::{
    branch::alt, branch::permutation, bytes::complete::tag, bytes::complete::take_while_m_n,
    character::complete::digit1, character::complete::multispace0, combinator::map_res,
    combinator::opt, sequence::delimited, sequence::pair, sequence::preceded, IResult,
};

const INPUT_FILE: &str = include_str!("../../inputs/day04.txt");

#[derive(Debug, PartialEq)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: (u32, String),
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

fn not_whitespace(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::is_not(" \t\n")(input)
}

fn parse_byr(input: &str) -> IResult<&str, u32> {
    map_res(
        delimited(multispace0, preceded(tag("byr:"), digit1), multispace0),
        |i: &str| u32::from_str_radix(i, 10),
    )(input)
}

fn parse_iyr(input: &str) -> IResult<&str, u32> {
    map_res(
        delimited(multispace0, preceded(tag("iyr:"), digit1), multispace0),
        |i: &str| u32::from_str_radix(i, 10),
    )(input)
}

fn parse_eyr(input: &str) -> IResult<&str, u32> {
    map_res(
        delimited(multispace0, preceded(tag("eyr:"), digit1), multispace0),
        |i: &str| u32::from_str_radix(i, 10),
    )(input)
}

fn parse_centimeters(input: &str) -> IResult<&str, (u32, &str)> {
    pair(
        map_res(digit1, |i: &str| u32::from_str_radix(i, 10)),
        alt((tag("cm"), tag("in"))),
    )(input)
}

fn parse_hgt(input: &str) -> IResult<&str, (u32, &str)> {
    delimited(
        multispace0,
        preceded(tag("hgt:"), parse_centimeters),
        multispace0,
    )(input)
}

fn is_char_hex_digit(c: char) -> bool {
    c.is_ascii() && c.is_digit(16)
}

fn is_char_digit(c: char) -> bool {
    return c.is_ascii() && c.is_digit(10);
}

fn parse_hex_color(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), take_while_m_n(6, 6, is_char_hex_digit))(input)
}

fn parse_hcl(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("hcl:"), parse_hex_color),
        multispace0,
    )(input)
}

fn parse_eye_color(input: &str) -> IResult<&str, &str> {
    alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(input)
}

fn parse_ecl(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("ecl:"), parse_eye_color),
        multispace0,
    )(input)
}

fn parse_passport_number(input: &str) -> IResult<&str, &str> {
    take_while_m_n(9, 9, is_char_digit)(input)
}

fn parse_pid(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        preceded(tag("pid:"), parse_passport_number),
        multispace0,
    )(input)
}

fn parse_cid(input: &str) -> IResult<&str, Option<&str>> {
    delimited(
        multispace0,
        opt(preceded(tag("cid:"), not_whitespace)),
        multispace0,
    )(input)
}

// fn parse_passport_part1(input: &str) -> IResult<&str, Passport> {
//     let (unhandled, (byr, iyr, eyr, hgt, hcl, ecl, pid, cid)) = permutation((
//         parse_byr, parse_iyr, parse_eyr, parse_hgt, parse_hcl, parse_ecl, parse_pid, parse_cid,
//     ))(input)?;

//     let cid_str = match cid {
//         Some(c) => Some(c.to_string()),
//         None => None,
//     };

//     let passport = Passport {
//         byr: String::from(byr),
//         iyr: String::from(iyr),
//         eyr: String::from(eyr),
//         hgt: String::from(hgt),
//         hcl: String::from(hcl),
//         ecl: String::from(ecl),
//         pid: String::from(pid),
//         cid: cid_str,
//     };
//     return Ok((unhandled, passport));
// }

fn parse_passport_part2(input: &str) -> IResult<&str, Passport> {
    let (unhandled, (byr, iyr, eyr, hgt, hcl, ecl, pid, cid)) = permutation((
        parse_byr, parse_iyr, parse_eyr, parse_hgt, parse_hcl, parse_ecl, parse_pid, parse_cid,
    ))(input)?;

    let cid_str = match cid {
        Some(c) => Some(c.to_string()),
        None => None,
    };

    let passport = Passport {
        byr: byr,
        iyr: iyr,
        eyr: eyr,
        hgt: (hgt.0, String::from(hgt.1)),
        hcl: String::from(hcl),
        ecl: String::from(ecl),
        pid: String::from(pid),
        cid: cid_str,
    };
    return Ok((unhandled, passport));
}

// fn part_1(input: &str) -> usize {
//     let valid_passports = input
//         .split("\n\n")
//         .map(parse_passport_part1)
//         .filter(|parsed| parsed.is_ok())
//         .count();

//     valid_passports
// }

fn part_2(input: &str) -> usize {
    let valid_passports = input
        .split("\n\n")
        .map(parse_passport_part2)
        .filter_map(|passport| {
            if !passport.is_ok() {
                return None;
            }

            let (_unhandled, valid) = passport.unwrap();

            if valid.byr < 1920 || valid.byr > 2002 {
                return None;
            }

            if valid.iyr < 2010 || valid.iyr > 2020 {
                return None;
            }

            if valid.eyr < 2020 || valid.eyr > 2030 {
                return None;
            }

            if valid.hgt.1 == "cm" && (valid.hgt.0 < 150 || valid.hgt.0 > 193) {
                return None;
            }

            if valid.hgt.1 == "in" && (valid.hgt.0 < 59 || valid.hgt.0 > 76) {
                return None;
            }
            return Some(valid);
        })
        .count();

    valid_passports
}

fn main() -> () {
    // let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);

    // println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_parses_passports() {
    //     assert_eq!(
    //         parse_passport_part1(
    //             "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
    //              byr:1937 iyr:2017 cid:147 hgt:183cm"
    //         ),
    //         Ok((
    //             "",
    //             Passport {
    //                 ecl: String::from("gry"),
    //                 pid: String::from("860033327"),
    //                 eyr: String::from("2020"),
    //                 hcl: String::from("#fffffd"),
    //                 byr: String::from("1937"),
    //                 iyr: String::from("2017"),
    //                 cid: Some(String::from("147")),
    //                 hgt: String::from("183cm")
    //             }
    //         ))
    //     );

    //     assert_eq!(
    //         parse_passport_part1(
    //             "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
    //              hcl:#cfa07d byr:1929"
    //         )
    //         .is_ok(),
    //         false
    //     );

    //     assert_eq!(
    //         parse_passport_part1(
    //             "hcl:#ae17e1 iyr:2013\n\
    //              eyr:2024\n\
    //              ecl:brn pid:760753108 byr:1931\n\
    //              hgt:179cm"
    //         ),
    //         Ok((
    //             "",
    //             Passport {
    //                 hcl: String::from("#ae17e1"),
    //                 iyr: String::from("2013"),
    //                 eyr: String::from("2024"),
    //                 ecl: String::from("brn"),
    //                 pid: String::from("760753108"),
    //                 byr: String::from("1931"),
    //                 cid: None,
    //                 hgt: String::from("179cm")
    //             }
    //         ))
    //     );

    //     assert_eq!(
    //         parse_passport_part1(
    //             "hcl:#cfa07d eyr:2025 pid:166559648\n\
    //              iyr:2011 ecl:brn hgt:59in"
    //         )
    //         .is_ok(),
    //         false
    //     );
    // }

    // #[test]
    // fn it_solves_part1_example() {
    //     assert_eq!(
    //         part_1(
    //             "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
    //              byr:1937 iyr:2017 cid:147 hgt:183cm\n\
    //              \n\
    //              iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
    //              hcl:#cfa07d byr:1929\n\
    //              \n\
    //              hcl:#ae17e1 iyr:2013\n\
    //              eyr:2024\n\
    //              ecl:brn pid:760753108 byr:1931\n\
    //              hgt:179cm\n\
    //              \n\
    //              hcl:#cfa07d eyr:2025 pid:166559648\n\
    //              iyr:2011 ecl:brn hgt:59in"
    //         ),
    //         2
    //     );
    // }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(
            part_2(
                "eyr:1972 cid:100\n\
                 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
                 \n\
                 iyr:2019\n\
                 hcl:#602927 eyr:1967 hgt:170cm\n\
                 ecl:grn pid:012533040 byr:1946\n\
                 \n\
                 hcl:dab227 iyr:2012\n\
                 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
                 \n\
                 hgt:59cm ecl:zzz\n\
                 eyr:2038 hcl:74454a iyr:2023\n\
                 pid:3556412378 byr:2007"
            ),
            0
        );

        assert_eq!(
            part_2(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
                 hcl:#623a2f\n
                 \n\
                 eyr:2029 ecl:blu cid:129 byr:1989\n\
                 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
                 \n\
                 hcl:#888785\n\
                 hgt:164cm byr:2001 iyr:2015 cid:88\n\
                 pid:545766238 ecl:hzl\n\
                 eyr:2022\n\
                 \n\
                 iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            ),
            4
        );
    }
}
