use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../input.txt");

fn calc_line_signature (line: &str) -> (bool, bool) {
    let mut signature = HashMap::with_capacity(26);

    for c in line.chars() {
        *signature.entry(c).or_insert(0) += 1;
    }

    let twice = signature.values().any(|&count| count == 2);
    let thrice = signature.values().any(|&count| count == 3);

    (twice, thrice)
}

fn part_1(file: &str) -> usize {    
    let signatures = file.lines()
        .map(calc_line_signature);

    let twice_total = signatures.clone().filter(|s| s.0).count();
    let thrice_total = signatures.clone().filter(|s| s.1).count();
    
    return twice_total * thrice_total
}

fn part_2(file: &str) -> String {
    let mut found = false;
    let mut result = String::new();

    for word1 in file.lines() {
        for word2 in file.lines() {
            if word1.len() != word2.len() {
                continue;
            }

            let mut differences = 0;
            let mut diff_index = 0;

            for (index, c) in word1.chars().enumerate() {
                if c != word2.chars().nth(index).expect("Invalid length") {
                    if differences > 0 {
                        differences += 1;
                        // Just skip to next word at this point
                        break;
                    }
                    differences += 1;
                    diff_index = index;
                }
            }

            if differences == 1 {
                found = true;
                // Doesn't work if the last character is different.. whatever
                result = [&word1[..diff_index], &word2[diff_index+1..]].concat();
                break;
            }
        };

        if found {
            break;
        }
    };

    result
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
    const PART1_TEST_FILE: &str = include_str!("../test/part1.txt");
    const PART2_TEST_FILE: &str = include_str!("../test/part2.txt");

    #[test]
    fn it_calculates_day02_part1_signatures() {
        assert_eq!(calc_line_signature("abcdef"), (false, false));
        assert_eq!(calc_line_signature("bababc"), (true, true));
        assert_eq!(calc_line_signature("abbcde"), (true, false));
        assert_eq!(calc_line_signature("abcccd"), (false, true));
        assert_eq!(calc_line_signature("aabcdd"), (true, false));
        assert_eq!(calc_line_signature("abcdee"), (true, false));
        assert_eq!(calc_line_signature("ababab"), (false, true));
    }

    #[test]
    fn it_solves_day02_part1_example() {
        assert_eq!(part_1(PART1_TEST_FILE), 12);
    }

    #[test]
    fn it_solves_day02_part2_example() {
        assert_eq!(part_2(PART2_TEST_FILE), "fgij");
    }
}
