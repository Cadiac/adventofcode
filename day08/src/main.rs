const INPUT_FILE: &str = include_str!("../input.txt");

fn part_1(input: &str, width: usize, height: usize) -> i32 {
    let layer_length = width * height;
    let layer_count = input.len() / layer_length;

    let mut image = Vec::new();

    for i in 0..layer_count {
        let layer_input = &input[i * layer_length .. (i+1) * layer_length];
        let layer_chars: Vec<u32> = layer_input.chars().map(|c| c.to_digit(10).expect("an u32")).collect();

        image.push(layer_chars);
    }

    println!("[DEBUG]: width: {}, height: {}, layers_count: {}, layer_length: {}", width, height, layer_count, layer_length);

    let result = image.iter().fold((std::i32::MAX, 0), |(fewest, checksum), layer| {
        let mut zero_digits = 0;        
        let mut one_digits = 0;
        let mut two_digits = 0;

        for digit in layer {
            if *digit == 0 {
                zero_digits += 1;
            }
            if *digit == 1 {
                one_digits += 1;
            }
            if *digit == 2 {
                two_digits += 1;
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
        assert_eq!(part_1("123456789012", 3, 2), 1);
    }

    #[test]
    fn it_solves_day08_another_example() {
        // Layer 1:
        //  1210
        //  0100
        // 4 x 0 digits
        // 3 x 1 digits
        // 1 x 2 digits
        // checksum == 3

        // Layer 2:
        //  1012
        //  0102
        // 3 x 0 digits
        // 3 x 1 digits
        // 2 x 2 digits
        // checksum = 6
        assert_eq!(part_1("1210010010120102", 4, 2), 6);
    }
}
