use std::iter::FromIterator;

const INPUT_FILE: &str = include_str!("../input.txt");

fn fft(input: String, phases: u32) -> Vec<i32> {
    let sequence: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let base_pattern: [i32; 4] = [0, 1, 0, -1];

    let mut signal = sequence.clone();

    for phase in 0..phases {
        // println!("[PHASE{}]: signal {:?}", phase, signal);
        for digit_ix in 0..signal.len() {
            let mut repeated_pattern = Vec::new();

            for value in &base_pattern {
                for _repeat in 0..(digit_ix+1) {
                    // repeat each value in the pattern a number of times equal
                    // to the position in the output list being considered
                    repeated_pattern.push(value);
                }
            }
            // println!("[PHASE{}]: pattern {:?}", phase, repeated_pattern);

            let mut pattern_index = 1; // Offset the pattern by one
            let mut sum = 0;
            for digit in signal.clone() {
                sum += digit * repeated_pattern[pattern_index];
                pattern_index = (pattern_index + 1) % repeated_pattern.len();
            }

            signal[digit_ix] = sum.abs() % 10;
        }
    }

    return Vec::from_iter(signal[0..8].iter().cloned());
}

fn main() -> () {
    let message_part1 = fft(String::from(INPUT_FILE), 100);
    // let message_part2 = fft(String::from(INPUT_FILE).repeat(10000), 100);

    println!("[INFO]: Part 1: {:?}", message_part1);
    // println!("[INFO]: Part 2: {:?}", message_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_correct_digits_example_1() {
        assert_eq!(fft(String::from("12345678"), 1), vec![4, 8, 2, 2, 6, 1, 5, 8]);
        assert_eq!(fft(String::from("12345678"), 2), vec![3, 4, 0, 4, 0, 4, 3, 8]);
        assert_eq!(fft(String::from("12345678"), 3), vec![0, 3, 4, 1, 5, 5, 1, 8]);
        assert_eq!(fft(String::from("12345678"), 4), vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn it_calculates_correct_digits_example_2() {
        assert_eq!(fft(String::from("80871224585914546619083218645595"), 100), vec![2, 4, 1, 7, 6, 1, 7, 6]);
        assert_eq!(fft(String::from("19617804207202209144916044189917"), 100), vec![7, 3, 7, 4, 5, 4, 1, 8]);
        assert_eq!(fft(String::from("69317163492948606335995924319873"), 100), vec![5, 2, 4, 3, 2, 1, 3, 3]);
    }
}
