use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
struct Planet {
    name: String,
    depth: u32,
    children: Vec<String>,
}

fn recursive_set_depth(mut system: HashMap<String, Planet>, planet: String, depth: u32) -> HashMap<String, Planet> {
    let current_planet = system
        .entry(planet.clone())
        .and_modify(|planet| { planet.depth = depth })
        .or_insert(Planet{ name: planet, depth: depth, children: Vec::new() });

    for child in current_planet.clone().children {
        system = recursive_set_depth(system, child, depth + 1);
    }

    return system;
}

fn build_tree(orbits: Vec<(String,String)>) -> HashMap<String, Planet> {
    let mut system: HashMap<String, Planet> = HashMap::new();

    for (center, child) in orbits {
        system
            .entry(center.clone())
            .or_insert(Planet{ name: center, depth: 0, children: Vec::new() })
            .children.push(child.clone());
        
        // Make sure we also create the planets that don't have anything orbiting them
        system.entry(child.clone()).or_insert(Planet{ name: child, depth: 0, children: Vec::new() });
    }

    // Loop down from root, setting the depth of each leaf of the tree
    system = recursive_set_depth(system, String::from("COM"), 0);

    return system;
}

fn is_descendant(system: HashMap<String, Planet>, parent: &Planet, descendant: &Planet) -> bool {
    if parent.name == descendant.name {
        return true;
    }

    if parent.children.contains(&descendant.name) {
        return true;
    }

    return parent.children.iter().any(|child| {
        let current_child = system.get(child).expect("Child missing");

        return is_descendant(system.clone(), current_child, descendant);
    });
}

fn lowest_common_ancestor(system: HashMap<String, Planet>, planet_1: Planet, planet_2: Planet) -> Planet {
    println!("Finding LCA between {:?} and {:?}", planet_1, planet_2);

    if planet_1.name == planet_2.name {
        return planet_1;
    }

    if planet_1.children.contains(&planet_2.name) || planet_2.children.contains(&planet_1.name) {
        return planet_1;
    }

    // This is slow, changing the tree to know its parents would boost performance a lot
    let mut current_parent = system.iter().find(|(_, planet)| { planet.children.contains(&planet_1.name) });

    while let Some((_parent_key, parent_planet)) = current_parent {
        // If parent has the planet 2 as its descendant we have found the LCA
        if is_descendant(system.clone(), parent_planet, &planet_2) {
            println!("LCA is {:?}", parent_planet);
            return parent_planet.clone();
        }

        // Otherwise keep traversing up
        current_parent = system.iter().find(|(_, planet)| { planet.children.contains(&parent_planet.name) });
    }

    return planet_1;
}

fn part_1(orbits: Vec<(String, String)>) -> u32 {
    let system: HashMap<String, Planet> = build_tree(orbits);

    println!("System: {:?}", system);

    let mut total_orbits = 0;

    for (_name, planet) in system {
        total_orbits += planet.depth;
    }

    return total_orbits;
}

fn part_2(orbits: Vec<(String, String)>) -> u32 {
    let system: HashMap<String, Planet> = build_tree(orbits);

    println!("System: {:?}", system);

    let you = system.get(&String::from("YOU")).expect("YOU missing");
    let santa = system.get(&String::from("SAN")).expect("SANTA missing");
    let lca = lowest_common_ancestor(system.clone(), you.clone(), santa.clone());

    // Count the first child-parent link as zero distance, since it represents us already orbiting that planet
    let total_distance = you.depth - lca.depth + santa.depth - lca.depth - 2;

    return total_distance;
}

fn main() -> () {
    let orbits: Vec<(String, String)> = INPUT_FILE
        .lines()
        .map(|orbit| {
            let planets: Vec<&str> = orbit.split(')').collect();
            return (String::from(planets[0]), String::from(planets[1]));
        })
        .collect();

    let part1 = part_1(orbits.clone());
    let part2 = part_2(orbits.clone());

    println!("Part 1: {}", part1);
    println!("Part 1: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_correct_checksum() {
        let orbits = vec![
            (String::from("COM"), String::from("B")),
            (String::from("B"), String::from("C")),
            (String::from("C"), String::from("D")),
            (String::from("D"), String::from("E")),
            (String::from("E"), String::from("F")),
            (String::from("B"), String::from("G")),
            (String::from("G"), String::from("H")),
            (String::from("D"), String::from("I")),
            (String::from("E"), String::from("J")),
            (String::from("J"), String::from("K")),
            (String::from("K"), String::from("L"))
        ];
        assert_eq!(part_1(orbits), 42);
    }

    #[test]
    fn it_solves_part2_example() {
        let orbits = vec![
            (String::from("COM"), String::from("B")),
            (String::from("B"), String::from("C")),
            (String::from("C"), String::from("D")),
            (String::from("D"), String::from("E")),
            (String::from("E"), String::from("F")),
            (String::from("B"), String::from("G")),
            (String::from("G"), String::from("H")),
            (String::from("D"), String::from("I")),
            (String::from("E"), String::from("J")),
            (String::from("J"), String::from("K")),
            (String::from("K"), String::from("L")),
            (String::from("K"), String::from("YOU")),
            (String::from("I"), String::from("SAN")),
        ];
        assert_eq!(part_2(orbits), 4);
    }
}
