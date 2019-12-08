const INPUT_FILE: &str = include_str!("../input.txt");

fn part_1(input: &str, width: usize, height: usize) -> i32 {
    let chars: Vec<u32> = input.chars().map(|c| c.to_digit(10).expect("an u32")).collect();

    let chunks: Vec<&[u32]> = chars.chunks(width).collect();
    let layers: Vec<&[&[u32]]> = chunks.chunks(height).collect();

    let result = layers.iter().fold((std::i32::MAX, 0), |(fewest, checksum), layer| {
        let mut zero_digits = 0;        
        let mut one_digits = 0;
        let mut two_digits = 0;

        for chunk in *layer {
            for digit in *chunk {
                if *digit == 0 {
                    zero_digits += 1;
                }
                if *digit == 1 {
                    one_digits += 1;
                }
                if *digit == 2 {
                    two_digits += 2;
                }
            }
        }

        if zero_digits < fewest {
            return (zero_digits, one_digits * two_digits);
        }

        return (fewest, checksum);
    });

    println!("[DEBUG]: result {:?}", result);
    
    return result.1;
}


fn main() -> () {
    let part1 = part_1(INPUT_FILE, 25, 6);

    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_day08_part1_example() {
        assert_eq!(part_1("123456789012", 3, 2), 2);
    }

    #[test]
    fn it_solves_day08_another_example() {
        // Layer 1:
        //  1210
        //  0100
        // 4 x 0 digits
        // 3 x 1 digits
        // 2 x 2 digits
        // checksum == 6

        // Layer 2:
        //  1012
        //  0102
        // 3 x 0 digits
        // 3 x 1 digits
        // 2 x 2 digits
        // checksum = 12
        assert_eq!(part_1("1210010010120102", 4, 2), 12);
    }
}
