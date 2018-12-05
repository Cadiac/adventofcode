use std::collections::VecDeque;

const INPUT_FILE: &str = include_str!("../input.txt");

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

const ASCII_CASE_DIFFERENCE: u8 = 32;

fn cascade(original_characters: VecDeque<char>) -> usize {
    let mut processed_index = 0;
    let mut characters = original_characters.clone();
    loop {
        let mut removed_something = false;
        for index in processed_index..characters.len()-1 {
            let diff = (characters[index] as i32 - characters[index+1] as i32).abs() as u8;

            if diff == ASCII_CASE_DIFFERENCE {
                characters.remove(index);
                characters.remove(index);
                removed_something = true;
                if index != 0 {
                    processed_index = index - 1;
                }
                break;
            }
        }
        if !removed_something {
            return characters.len();
        }
    }
}

fn part_1(file: &str) -> usize {
    let characters = file.chars().collect::<VecDeque<char>>();
    cascade(characters)
}

fn part_2(file: &str) -> usize {
    let characters = file.chars().collect::<VecDeque<char>>();
    
    let mut results = Vec::new();

    for c in ALPHABET.iter() {
        let characters_without_letter = characters
            .clone()   
            .into_iter()
            .filter(|letter| letter != c && (*letter as u8 + ASCII_CASE_DIFFERENCE) as char != *c)
            .collect();

        results.push(cascade(characters_without_letter));
    };

    *results.iter().min().expect("No results")
}

fn main() {
    let part1_result = part_1(INPUT_FILE);
    let part2_result = part_2(INPUT_FILE);
 
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../test/example.txt");

    #[test]
    fn it_cascades_correctly() {
        assert_eq!(cascade("dbcCCBcCcD".chars().collect::<VecDeque<char>>()), 6);
        assert_eq!(cascade("daAcCaCAcCcaDA".chars().collect::<VecDeque<char>>()), 8);
        assert_eq!(cascade("dabAaBAaDA".chars().collect::<VecDeque<char>>()), 4);
        assert_eq!(cascade("abAcCaCBAcCcaA".chars().collect::<VecDeque<char>>()), 6);
    }

    #[test]
    fn it_solves_day05_part1_example() {
        assert_eq!(part_1(TEST_FILE), 10);
    }

    #[test]
    fn it_solves_day05_part2_example() {
        assert_eq!(part_2(TEST_FILE), 4);
    }
}
