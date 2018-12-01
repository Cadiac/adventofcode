use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

fn part_1(file_name: String) -> Result<i32> {
    let file = File::open(file_name)?;
    let mut part1_freq = 0;

    for line in BufReader::new(file).lines() {
        part1_freq += line?.parse::<i32>().unwrap();
    }

    return Ok(part1_freq);
}

fn part_2(file_name: String) -> Result<i32> {
    let mut part2_freq = 0;
    let mut found = false;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(part2_freq);

    while !found {
        let file = File::open(file_name.clone())?;

        for line in BufReader::new(file).lines() {
            part2_freq += line?.parse::<i32>().unwrap();
            if !seen.insert(part2_freq) {
                found = true;
                break;
            }
        }
    }

    return Ok(part2_freq);
}

fn main() -> Result<()> {
    let part1_freq = part_1(String::from("./input.txt"));
    let part2_freq = part_2(String::from("./input.txt"));

    println!("Part 1: {}", part1_freq?);
    println!("Part 2: {}", part2_freq?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(part_1(String::from("./test/part1_1.txt")).unwrap(), 3);
        assert_eq!(part_1(String::from("./test/part1_2.txt")).unwrap(), 0);
        assert_eq!(part_1(String::from("./test/part1_3.txt")).unwrap(), -6);
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(part_2(String::from("./test/part2_1.txt")).unwrap(), 0);
        assert_eq!(part_2(String::from("./test/part2_2.txt")).unwrap(), 10);
        assert_eq!(part_2(String::from("./test/part2_3.txt")).unwrap(), 5);
        assert_eq!(part_2(String::from("./test/part2_4.txt")).unwrap(), 14);
    }
}
