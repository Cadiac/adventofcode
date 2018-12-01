use std::io::{Result};

// use std::fs::File;
// use std::io::{BufRead, BufReader, Result};

fn part_1() -> Result<i32> {
    // let file = File::open(file_name)?;
    // for line in BufReader::new(file).lines() {
    // }
    
    return Ok(0);
}

fn part_2() -> Result<i32> {
    // let file = File::open(file_name)?;
    // for line in BufReader::new(file).lines() {
    // }

    return Ok(0);
}

fn main() -> Result<()> {
    let part1_result = part_1();
    let part2_result = part_2();

    println!("Part 1: {}", part1_result?);
    println!("Part 2: {}", part2_result?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(part_1().unwrap(), 0);
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(part_2().unwrap(), 0);
    }
}
