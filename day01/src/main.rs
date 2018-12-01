use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    let file = File::open("./input.txt")?;
    let mut part1_freq = 0;

    for line in BufReader::new(file).lines() {
        part1_freq += line?.parse::<i32>().unwrap();
    }

    let mut seen: HashSet<i32> = HashSet::new();
    let mut found = false;

    let mut part2_freq = 0;
    while !found {
        let file = File::open("./input.txt")?;

        for line in BufReader::new(file).lines() {
            part2_freq += line?.parse::<i32>().unwrap();
            if !seen.insert(part2_freq) {
                found = true;
                break;
            }
        }
    }

    println!("Part 1: {}", part1_freq);
    println!("Part 2: {}", part2_freq);
    Ok(())
}