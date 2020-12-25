const INPUT_FILE: &str = include_str!("../../inputs/day25.txt");

fn transform_subject_num(subject_number: u64, loop_size: u64) -> u64 {
    // The handshake used by the card and the door involves an operation that
    // transforms a subject number. To transform a subject number, start with
    // the value 1. Then, a number of times called the loop size, perform the following steps:
    let mut value = 1;

    for _i in 0..loop_size {
        // Set the value to itself multiplied by the subject number.
        value *= subject_number;
        // Set the value to the remainder after dividing the value by 20201227.
        value %= 20201227;
    }

    value
}

fn part_1(input: &str) -> u64 {
    let mut iter = input.lines();

    let pub_key_1: u64 = iter.next().unwrap().parse().unwrap();
    let pub_key_2: u64 = iter.next().unwrap().parse().unwrap();

    let mut value = 1;
    let mut secret_loop_size = 0;

    let other_pub_key = loop {
        secret_loop_size += 1;
        value *= 7;
        value %= 20201227;

        if value == pub_key_1 {
            break pub_key_2;
        }

        if value == pub_key_2 {
            break pub_key_1;
        }
    };
    
    let encryption_key = transform_subject_num(other_pub_key, secret_loop_size);

    return encryption_key;
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part_1_example() {
        assert_eq!(part_1("17807724\n5764801"), 14897079);
    }
}
