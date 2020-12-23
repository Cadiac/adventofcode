use std::collections::VecDeque;

const INPUT: &str = "643719258";

fn part_1(input: &str, moves: u32) -> String {
    let mut cups: VecDeque<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // Treat the last cup as current cup, and the first elements of the
    // queue as the ones clockwise next to them
    cups.rotate_left(1);

    let max_cup = cups.iter().max().cloned().unwrap();
    let min_cup = cups.iter().min().cloned().unwrap();

    for _mov in 0..moves {
        // println!("cups: {:?}", cups);

        // The crab picks up the three cups that are immediately clockwise of the current cup.
        // They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let cup_1 = cups.pop_front().unwrap();
        let cup_2 = cups.pop_front().unwrap();
        let cup_3 = cups.pop_front().unwrap();
        // println!("pick up: ({:?}, {:?}, {:?})", cup_1, cup_2, cup_3);
        // The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.
        // If this would select one of the cups that was just picked up, the crab will keep subtracting one
        // until it finds a cup that wasn't just picked up. If at any point in this process the value goes
        // below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
        let mut dest_val = cups.back().unwrap().clone();

        let dest_idx = 'outer: loop {
            dest_val = if dest_val <= min_cup {
                max_cup
            } else {
                dest_val - 1
            };

            // println!("searching destination val {:?}", dest_val);
            for (idx, cup) in cups.iter().enumerate() {
                if *cup == dest_val {
                    // println!("dest_idx: {:?}", idx);
                    break 'outer idx;
                }
            }
        };

        // The crab places the cups it just picked up so that they are immediately clockwise of the destination cup.
        // They keep the same order as when they were picked up.
        cups.insert(dest_idx + 1, cup_3);
        cups.insert(dest_idx + 1, cup_2);
        cups.insert(dest_idx + 1, cup_1);

        // The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
        cups.rotate_left(1);
    }

    while cups.front() != Some(&1) {
        cups.rotate_left(1);
    }

    cups.into_iter()
        .skip(1)
        .map(|cup| cup.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn main() -> () {
    let part_1_result = part_1(INPUT, 100);

    println!("[INFO]: Part 1: {:?}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_10() {
        assert_eq!(part_1("389125467", 10), "92658374");
    }

    #[test]
    fn it_solves_part1_example_100() {
        assert_eq!(part_1("389125467", 100), "67384529");
    }
}
