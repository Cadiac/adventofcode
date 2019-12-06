use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../input.txt");

fn part_1(orbits: Vec<(&str, &str)>) -> u32 {
    let mut system: HashMap<&str, Vec<&str>> = HashMap::new();

    for (center, child) in orbits {
        system
            .entry(center)
            .or_insert(Vec::new())
            .push(child);
        
        // Make sure we also create the planets that don't have anything orbiting them
        system.entry(child).or_insert(Vec::new());
    }

    let mut total_orbits: u32 = 0;

    for (planet, _) in system.clone() {
        let mut depth = 0;

        let mut parent = system
            .iter()
            .find(|(_sys_planet, sys_orbits)| sys_orbits.contains(&planet));

        while let Some((parent_planet, _)) = parent {
            depth += 1;

            parent = system
                .iter()
                .find(|(_sys_planet, sys_orbits)| sys_orbits.contains(&parent_planet));
        }

        total_orbits += depth;
    }

    println!("System: {:?}", system);

    return total_orbits;
}

fn main() -> () {
    let orbits: Vec<(&str, &str)> = INPUT_FILE.lines().map(|orbit| {
        let planets: Vec<&str> = orbit.split(')').collect();
        return (planets[0], planets[1]);
    }).collect();

    let part1 = part_1(orbits);

    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_correct_checksum() {
        let orbits = vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L")
        ];
        assert_eq!(part_1(orbits), 42);
    }
}
