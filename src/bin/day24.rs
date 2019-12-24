extern crate regex;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day24.txt");
const ADJACENT_TILES: [(i64, i64); 4] = [(0,1), (0,-1), (1,0), (-1,0)];

fn parse_world(input: String) -> HashMap<(i64, i64), bool> {
    let mut world: HashMap<(i64, i64), bool> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            if tile == '#' {
                world.insert((x as i64, y as i64), true);
            } else if tile == '.' {
                world.insert((x as i64, y as i64), false);
            }
        };
    };

    return world;
}

fn calculate_biodiversity(world: &HashMap<(i64, i64), bool>) -> i64 {
    world.iter().fold(0, |mut acc, ((x,y), bug)| {
        if *bug {
            let i = x + y*5;
            acc += 2_i64.pow(i as u32);
        }
        return acc;
    })
}

fn part_1(input: String) -> i64 {
    let mut world = parse_world(input);
    let mut biodiversities: HashSet<i64> = HashSet::new();

    loop {
        let mut new_world = world.clone();

        for ((x,y), bug) in &world {
            let mut adjacent_bugs = 0;
            for adjacent in &ADJACENT_TILES {
                if let Some(tile) = world.get(&(x + adjacent.0, y + adjacent.1)) {
                    if *tile {
                        adjacent_bugs += 1;
                    }
                }
            }

            if *bug {
                new_world.insert((*x, *y), adjacent_bugs == 1);
            } else {
                new_world.insert((*x, *y), adjacent_bugs == 1 || adjacent_bugs == 2);
            }
        }

        let biodiversity = calculate_biodiversity(&world);
        if !biodiversities.insert(biodiversity) {
            return biodiversity;
        }
        world = new_world;
    }
}

fn main() -> () {
    let biodiversity = part_1(String::from(INPUT_FILE));
    
    println!("[INFO]: Part 1: {:?}", biodiversity);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_calculate_biodiversity_correctly() {
        let world = parse_world(
           String::from(
           ".....\n\
            .....\n\
            .....\n\
            #....\n\
            .#..."));
        let biodiversity = calculate_biodiversity(&world);
        assert_eq!(biodiversity, 2129920);
    }
}
