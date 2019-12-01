use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::cmp::max;

// Specifically, to find the fuel required for a module
// take its mass, divide by three, round down, and subtract 2.
fn calculate_fuel_for_mass(mass: u32) -> u32 {
    return mass / 3 - 2;
}

fn calculate_fuel_for_mass_total(mass: u32) -> u32 {
    let mut fuel = max(mass/3, 2) - 2;
    if fuel > 0 {
        fuel += calculate_fuel_for_mass_total(fuel)
    }

    return fuel;
}

fn part_1(file_name: String) -> Result<u32> {
    let file = File::open(file_name)?;
    let mut fuel_required = 0;

    for line in BufReader::new(file).lines() {
        let mass = line?.parse::<u32>().unwrap();
        fuel_required += calculate_fuel_for_mass(mass);
    }

    return Ok(fuel_required);
}

fn part_2(file_name: String) -> Result<u32> {
    let file = File::open(file_name)?;
    let mut fuel_required = 0;

    for line in BufReader::new(file).lines() {
        let mass = line?.parse::<u32>().unwrap();
        fuel_required += calculate_fuel_for_mass_total(mass);
    }

    return Ok(fuel_required);
}

fn main() -> Result<()> {
    let part1_fuel = part_1(String::from("./input.txt"));
    let part2_fuel = part_2(String::from("./input.txt"));

    println!("Part 1: {}", part1_fuel?);
    println!("Part 2: {}", part2_fuel?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_mass_calculations() {
        // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
        assert_eq!(calculate_fuel_for_mass(12), 2);
        // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
        assert_eq!(calculate_fuel_for_mass(14), 2);
        // For a mass of 1969, the fuel required is 654.
        assert_eq!(calculate_fuel_for_mass(1969), 654);
        // For a mass of 100756, the fuel required is 33583.
        assert_eq!(calculate_fuel_for_mass(100756), 33583);
    }

    #[test]
    fn it_solves_part2_total_mass_calculations() {
        // Make sure negative calculations work
        assert_eq!(calculate_fuel_for_mass_total(1), 0);

        // A module of mass 14 requires 2 fuel. This fuel requires no further fuel
        // /(2 divided by 3 and rounded down is 0, which would call for a negative fuel),
        // so the total fuel required is still just 2.
        assert_eq!(calculate_fuel_for_mass_total(14), 2);

        // At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2).
        // 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel.
        // So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
        assert_eq!(calculate_fuel_for_mass_total(1969), 966);

        // The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
        assert_eq!(calculate_fuel_for_mass_total(100756), 50346);
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(part_1(String::from("./test/example_input.txt")).unwrap(), 2+2+654+33583);
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(part_2(String::from("./test/example_input.txt")).unwrap(), 2+2+966+50346);
    }
}
