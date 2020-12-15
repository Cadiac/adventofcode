use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../../inputs/day15.txt");

fn memory_game(input: &str, turns: u32) -> u32 {
    let mut mem: HashMap<u32, u32> = HashMap::new();

    let mut t = 0;
    let mut next_spoken = 0;

    // After the initial words the next spoken is still 0
    for i in input.split(',') {
        let num = i.parse::<u32>().unwrap();
        mem.insert(num, t);
        // println!("Turn {:?}: The number spoken is a starting number, {:?}.", t+1, num);
        t += 1;
    }

    while t < turns - 1 {
        if let Some(last_spoken_at) = mem.insert(next_spoken, t) {
            // println!("Turn {:?}: The number spoken is old, {:?}, last spoken at {:?}.", t+1, next_spoken, last_spoken_at);
            next_spoken = t - last_spoken_at;
        } else {
            // println!("Turn {:?}: The number spoken is new, {:?}.", t+1, next_spoken);
            next_spoken = 0;
        }

        t += 1;
    }

    return next_spoken;
}

fn main() -> () {
    let part_1_result = memory_game(INPUT_FILE, 2020);
    let part_2_result = memory_game(INPUT_FILE, 30000000);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(memory_game("0,3,6", 2020), 436);
        assert_eq!(memory_game("1,3,2", 2020), 1);
        assert_eq!(memory_game("2,1,3", 2020), 10);
        assert_eq!(memory_game("1,2,3", 2020), 27);
        assert_eq!(memory_game("2,3,1", 2020), 78);
        assert_eq!(memory_game("3,2,1", 2020), 438);
        assert_eq!(memory_game("3,1,2", 2020), 1836);
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(memory_game("0,3,6", 30000000), 175594);
        assert_eq!(memory_game("1,3,2", 30000000), 2578);
        assert_eq!(memory_game("2,1,3", 30000000), 3544142);
        assert_eq!(memory_game("1,2,3", 30000000), 261214);
        assert_eq!(memory_game("2,3,1", 30000000), 6895259);
        assert_eq!(memory_game("3,2,1", 30000000), 18);
        assert_eq!(memory_game("3,1,2", 30000000), 362);
    }
}
