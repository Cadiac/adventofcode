const INPUT_FILE: &str = include_str!("../../inputs/day02.txt");

fn part_1(input: &str) -> i32 {
    let mut position: (i32, i32) = (0, 0);

    for command in input.lines() {
        let parts: Vec<&str> = command.split(' ').collect();
        let steps = parts[1].parse::<i32>().unwrap();

        match parts[0] {
            "forward" => position.0 += steps,
            "down" => position.1 += steps,
            "up" => position.1 -= steps,
            _ => unimplemented!(),
        }
    }

    position.0 * position.1
}

fn part_2(input: &str) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    let mut aim = 0;

    for command in input.lines() {
        let parts: Vec<&str> = command.split(' ').collect();
        let steps = parts[1].parse::<i32>().unwrap();

        match parts[0] {
            "forward" => {
                position.0 += steps;
                position.1 += steps * aim;
            }
            "down" => aim += steps,
            "up" => aim -= steps,
            _ => unimplemented!(),
        }
    }

    position.0 * position.1
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "forward 5\n\
                down 5\n\
                forward 8\n\
                up 3\n\
                down 8\n\
                forward 2"
            ),
            150
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "forward 5\n\
                down 5\n\
                forward 8\n\
                up 3\n\
                down 8\n\
                forward 2"
            ),
            900
        );
    }
}
