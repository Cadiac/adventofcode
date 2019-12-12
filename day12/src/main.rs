extern crate regex;
use regex::Regex;

#[derive(PartialEq, Clone)]
struct Entity {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32)
}

impl std::fmt::Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "pos=<x={: >3}, y={: >3}, z={: >3}>, vel=<x={: >3}, y={: >3}, z={: >3}>\n",
            self.position.0, self.position.1, self.position.2,
            self.velocity.0, self.velocity.1, self.velocity.2);
    }
}

const INPUT_FILE: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<(Entity)> {
    let input_regex = Regex::new(r"^<x=(.+), y=(.+), z=(.+)>$").unwrap();
    
    let initial_state: Vec<Entity> = input
        .lines()
        .map(|line| {
            let capture = input_regex.captures(line).unwrap();

            let x = capture[1].parse::<i32>().unwrap();
            let y = capture[2].parse::<i32>().unwrap();
            let z = capture[3].parse::<i32>().unwrap();

            return Entity{ position: (x, y, z), velocity: (0, 0, 0) };
        })
        .collect();

    return initial_state;
}

fn simulate_step(entities: &mut Vec<Entity>) {
    // Calculate velocities
    for current_ix in 0..entities.len() {
        for other_ix in 0..entities.len() {
            if current_ix == other_ix { continue; }

            let current = entities[current_ix].clone();
            let other = entities[other_ix].clone();

            let mut dx = other.position.0 - current.position.0;
            let mut dy = other.position.1 - current.position.1;
            let mut dz = other.position.2 - current.position.2;

            if dx != 0 { dx = dx / dx.abs(); }
            if dy != 0 { dy = dy / dy.abs(); }
            if dz != 0 { dz = dz / dz.abs(); }
        
            entities[current_ix].velocity.0 += dx;
            entities[current_ix].velocity.1 += dy;
            entities[current_ix].velocity.2 += dz;
        }
    }

    // Apply movements
    for entity in entities {
        entity.position.0 += entity.velocity.0;
        entity.position.1 += entity.velocity.1;
        entity.position.2 += entity.velocity.2;
    }
}

fn simulate(mut entities: Vec<Entity>, steps: u32) -> Vec<Entity> {
    for step in 0..steps {
        simulate_step(&mut entities);
        println!("After {} steps:", step + 1);
        println!("{:?}", entities);
    }

    return entities;
}

fn calculate_total_energy(entities: Vec<Entity>) -> i32 {
    let mut total_energy = 0;

    for entity in entities {
        let pot_energy = entity.position.0.abs() + entity.position.1.abs() + entity.position.2.abs();
        let kin_energy = entity.velocity.0.abs() + entity.velocity.1.abs() + entity.velocity.2.abs();
        total_energy += pot_energy * kin_energy;
    }

    return total_energy;
}

fn main() -> () {
    let mut entities = parse_input(INPUT_FILE);
    entities = simulate(entities, 1000);
    let total_energy = calculate_total_energy(entities);

    println!("[INFO]: Part 1: {:?}", total_energy);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(simulate(parse_input(
            "<x=-1, y=0, z=2>\n\
            <x=2, y=-10, z=-7>\n\
            <x=4, y=-8, z=8>\n\
            <x=3, y=5, z=-1>"), 10),
        vec![
            Entity{ position: (2,  1, -3), velocity: (-3, -2,  1) },
            Entity{ position: (1, -8,  0), velocity: (-1,  1,  3) },
            Entity{ position: (3, -6,  1), velocity: ( 3,  2, -3) },
            Entity{ position: (2,  0,  4), velocity: ( 1, -1, -1) }
        ]);
    }

    #[test]
    fn it_solves_part1_example_2_step_10() {
        assert_eq!(simulate(parse_input(
            "<x=-8, y=-10, z=0>\n\
            <x=5, y=5, z=10>\n\
            <x=2, y=-7, z=3>\n\
            <x=9, y=-8, z=-3>"), 10),
        vec![
            Entity{ position: (-9,-10,  1), velocity: (-2, -2, -1) },
            Entity{ position: ( 4, 10,  9), velocity: (-3,  7, -2) },
            Entity{ position: ( 8,-10, -3), velocity: ( 5, -1, -2) },
            Entity{ position: ( 5,-10,  3), velocity: ( 0, -4,  5) }
        ]);
    }

    #[test]
    fn it_solves_part1_example_2_step_100() {
        assert_eq!(simulate(parse_input(
            "<x=-8, y=-10, z=0>\n\
            <x=5, y=5, z=10>\n\
            <x=2, y=-7, z=3>\n\
            <x=9, y=-8, z=-3>"), 100),
        vec![
            Entity{ position: (  8,-12, -9), velocity: ( -7,  3,  0) },
            Entity{ position: ( 13, 16, -3), velocity: (  3,-11, -5) },
            Entity{ position: (-29,-11, -1), velocity: ( -3,  7,  4) },
            Entity{ position: ( 16,-13, 23), velocity: (  7,  1,  1) }
        ]);
    }

    #[test]
    fn it_calculates_total_energy() {
        assert_eq!(calculate_total_energy(
            vec![
                Entity{ position: (2,  1, -3), velocity: (-3, -2,  1) },
                Entity{ position: (1, -8,  0), velocity: (-1,  1,  3) },
                Entity{ position: (3, -6,  1), velocity: ( 3,  2, -3) },
                Entity{ position: (2,  0,  4), velocity: ( 1, -1, -1) }
            ]
        ), 179);
    }
}