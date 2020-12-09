use std::collections::VecDeque;

const INPUT_FILE: &str = include_str!("../../inputs/day09.txt");

fn part_1(input: &str, preamble_len: usize) -> Option<usize> {
    let mut checksum: VecDeque<usize> = input
        .lines()
        .take(preamble_len)
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .collect();

    let first_invalid = input
        .lines()
        .skip(preamble_len)
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .find(|number| {
            for first in 0..(checksum.len() - 1) {
                for second in (first + 1)..(checksum.len()) {
                    if checksum[first] != checksum[second]
                        && checksum[first] + checksum[second] == *number
                    {
                        // Found two numbers that sum up to the current number.
                        // Push current to checksum and pop from the front
                        checksum.pop_front();
                        checksum.push_back(*number);

                        return false;
                    }
                }
            }

            return true;
        });

    first_invalid
}

fn part_2(input: &str, preamble_len: usize) -> Option<usize> {
    let first_invalid = part_1(input, preamble_len).unwrap();

    let list: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .collect();

    for (index, number) in list.iter().enumerate() {
        let mut sum = *number;
        for other in (index + 1)..(list.len()) {
            if sum > first_invalid {
                break;
            }

            sum += list[other];

            if sum == first_invalid {
                // To find the encryption weakness, add together the smallest and largest number
                // in this contiguous range; in this example, these are 15 and 47, producing 62.
                let min = list[index..=other].iter().min().unwrap();
                let max = list[index..=other].iter().max().unwrap();

                return Some(min + max);
            }
        }
    }

    None
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE, 25).unwrap();
    let part_2_result = part_2(INPUT_FILE, 25).unwrap();

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_examples() {
        assert_eq!(
            part_1(
                "35\n\
                 20\n\
                 15\n\
                 25\n\
                 47\n\
                 40\n\
                 62\n\
                 55\n\
                 65\n\
                 95\n\
                 102\n\
                 117\n\
                 150\n\
                 182\n\
                 127\n\
                 219\n\
                 299\n\
                 277\n\
                 309\n\
                 576",
                5
            ),
            Some(127)
        );
    }

    #[test]
    fn it_solves_part2_examples() {
        assert_eq!(
            part_2(
                "35\n\
                 20\n\
                 15\n\
                 25\n\
                 47\n\
                 40\n\
                 62\n\
                 55\n\
                 65\n\
                 95\n\
                 102\n\
                 117\n\
                 150\n\
                 182\n\
                 127\n\
                 219\n\
                 299\n\
                 277\n\
                 309\n\
                 576",
                5
            ),
            Some(62)
        );
    }
}
