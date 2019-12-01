use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

// Specifically, to find the fuel required for a module
// take its mass, divide by three, round down, and subtract 2.
fn calculalte_fuel_for_mass(mass: u32) -> u32 {
    return (mass / 3) - 2;
}

fn part_1(file_name: String) -> Result<u32> {
    let file = File::open(file_name)?;
    let mut fuel_required = 0;

    for line in BufReader::new(file).lines() {
        let mass = line?.parse::<u32>().unwrap();
        fuel_required += calculalte_fuel_for_mass(mass);
    }

    return Ok(fuel_required);
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
    let part1_sum = part_1(String::from("./input.txt"));
    // let part2_freq = part_2(String::from("./input.txt"));

    println!("Part 1: {}", part1_sum?);
    // println!("Part 2: {}", part2_freq?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_day01_part1_mass_calculations() {
        // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
        assert_eq!(calculalte_fuel_for_mass(12), 2);
        // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
        assert_eq!(calculalte_fuel_for_mass(14), 2);
        // For a mass of 1969, the fuel required is 654.
        assert_eq!(calculalte_fuel_for_mass(1969), 654);
        // For a mass of 100756, the fuel required is 33583.
        assert_eq!(calculalte_fuel_for_mass(100756), 33583);
    }

    #[test]
    fn it_solves_day01_part1_example() {
        assert_eq!(part_1(String::from("./test/part1_1.txt")).unwrap(), 34241);
    }

    // #[test]
    // fn it_solves_day01_part2_examples() {
    //     assert_eq!(part_2(String::from("./test/part2_1.txt")).unwrap(), 0);
    //     assert_eq!(part_2(String::from("./test/part2_2.txt")).unwrap(), 10);
    //     assert_eq!(part_2(String::from("./test/part2_3.txt")).unwrap(), 5);
    //     assert_eq!(part_2(String::from("./test/part2_4.txt")).unwrap(), 14);
    // }
}
