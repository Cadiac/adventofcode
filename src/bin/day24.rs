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

fn initialize_world_depth(world: &mut HashMap<(i64, i64, i64), bool>, depth: i64) {
    for y in 0..5 {
        for x in 0..5 {
            if x == 2 && y == 2 {
                continue;
            }
            world.insert((x, y, depth), false);
        }
    }
}

fn parse_world_part_2(input: String) -> HashMap<(i64, i64, i64), bool> {
    let mut bugs: HashMap<(i64, i64, i64), bool> = HashMap::new();

    initialize_world_depth(&mut bugs, 0);
    // Also initialize the world below and above
    initialize_world_depth(&mut bugs, 1);
    initialize_world_depth(&mut bugs, -1);

    // Initialize the starting bugs
    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            if tile == '#' {
                bugs.insert((x as i64, y as i64, 0), true);
            }
        };
    };

    return bugs;
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

fn part_2(input: String, minutes: i64) -> usize {
    let mut world = parse_world_part_2(input);

    for _minute in 0..minutes {
        let mut new_world = world.clone();

        for ((x,y,depth), bug) in &world {
            let mut adjacent_bugs = 0;

            // Check tiles within this world
            for adjacent in &ADJACENT_TILES {
                if let Some(tile) = world.get(&(x + adjacent.0, y + adjacent.1, *depth)) {
                    if *tile {
                        adjacent_bugs += 1;
                    }
                }
            }

            // Check the tiles in world containing this one
            if *x == 0 {
                if let Some(tile) = world.get(&(1, 2, depth - 1)) {
                    if *tile {
                        adjacent_bugs += 1;
                    }
                }
            }
            if *x == 4 {
                if let Some(tile) = world.get(&(3, 2, depth - 1)) {
                    if *tile {
                        adjacent_bugs += 1;
                    }
                }
            }
            if *y == 0 {
                if let Some(tile) = world.get(&(2, 1, depth - 1)) {
                    if *tile {
                        adjacent_bugs += 1;
                    }
                }
            }
            if *y == 4 {
                if let Some(tile) = world.get(&(2, 3, depth - 1)) {
                    if *tile {
                        adjacent_bugs += 1;
                    }
                }
            }

            // Check the tiles in world contained within this
            if *x == 2 && *y == 1 {
                for xx in 0..5 {
                    if let Some(tile) = world.get(&(xx, 0, depth + 1)) {
                        if *tile {
                            adjacent_bugs += 1;
                        }
                    }
                }
            }
            if *x == 2 && *y == 3 {
                for xx in 0..5 {
                    if let Some(tile) = world.get(&(xx, 4, depth + 1)) {
                        if *tile {
                            adjacent_bugs += 1;
                        }
                    }
                }
            }
            if *x == 1 && *y == 2 {
                for yy in 0..5 {
                    if let Some(tile) = world.get(&(0, yy, depth + 1)) {
                        if *tile {
                            adjacent_bugs += 1;
                        }
                    }
                }
            }
            if *x == 3 && *y == 2 {
                for yy in 0..5 {
                    if let Some(tile) = world.get(&(4, yy, depth + 1)) {
                        if *tile {
                            adjacent_bugs += 1;
                        }
                    }
                }
            }

            if *bug {
                new_world.insert((*x, *y, *depth), adjacent_bugs == 1);
            } else {
                new_world.insert((*x, *y, *depth), adjacent_bugs == 1 || adjacent_bugs == 2);
            }
        }

        // Check if the out most worlds have one or more bugs
        // If yes, create a new world
        let max_depth = (new_world.iter().max_by_key(|((_x,_y,depth), _)| depth).unwrap().0).2;
        let max_has_bugs = new_world.iter().any(|((_x,_y,depth), bug)| *depth == max_depth && *bug);
        if max_has_bugs {
            initialize_world_depth(&mut new_world, max_depth + 1);
        }

        let min_depth = (new_world.iter().min_by_key(|((_x,_y,depth), _)| depth).unwrap().0).2;
        let min_has_bugs = new_world.iter().any(|((_x,_y,depth), bug)| *depth == min_depth && *bug);
        if min_has_bugs {
            initialize_world_depth(&mut new_world, min_depth - 1);
        }

        world = new_world;
    }

    return world.iter().filter(|(_key, bug)| **bug).count();
}

fn main() -> () {
    let biodiversity = part_1(String::from(INPUT_FILE));
    let bugs = part_2(String::from(INPUT_FILE), 200);

    println!("[INFO]: Part 1: {:?}", biodiversity);
    println!("[INFO]: Part 2: {:?}", bugs);
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

    #[test]
    fn it_simulates_part2_example() {
        let bugs = part_2(
            String::from(
                "....#\n\
                #..#.\n\
                #.?##\n\
                ..#..\n\
                #...."), 10);

         assert_eq!(bugs, 99);
    }
}
