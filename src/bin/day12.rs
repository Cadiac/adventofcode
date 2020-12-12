const INPUT_FILE: &str = include_str!("../../inputs/day12.txt");

#[derive(Debug, Default)]
pub struct Ship {
    pub rotation: i32,
    pub position: (i32, i32),
    pub waypoint: (i32, i32),
}

fn part_1(input: &str) -> i32 {
    let mut ship = Ship {
        rotation: 90,
        position: (0, 0),
        waypoint: (10, -1),
    };

    for instruction in input.lines() {
        let action: String = instruction.chars().take(1).collect();
        let value: i32 = instruction.chars().skip(1).collect::<String>().parse().unwrap();

        match action.as_str() {
            // Action N means to move north by the given value.
            "N" => ship.position.1 -= value,
            // Action S means to move south by the given value.
            "S" => ship.position.1 += value,
            // Action E means to move east by the given value.
            "E" => ship.position.0 += value,
            // Action W means to move west by the given value.
            "W" => ship.position.0 -= value,
            // Action L means to turn left the given number of degrees.
            "L" => ship.rotation -= value,
            // Action R means to turn right the given number of degrees.
            "R" => ship.rotation += value,
            // Action F means to move forward by the given value in the direction the ship is currently facing.
            "F" => {
                match ship.rotation % 360 {
                    90 | -270 => ship.position.0 += value,
                    180 | -180 => ship.position.1 += value,
                    270 | -90 => ship.position.0 -= value,
                    0 => ship.position.1 -= value,
                    rot => panic!("Unhandled rotation {}", rot)
                }
            },
            act => panic!("Unhandled action {}", act)
        }
    }

    return ship.position.0.abs() + ship.position.1.abs();
}

fn part_2(input: &str) -> i32 {
    let mut ship = Ship {
        rotation: 90,
        position: (0, 0),
        waypoint: (10, -1),
    };

    for instruction in input.lines() {
        let action: String = instruction.chars().take(1).collect();
        let value: i32 = instruction.chars().skip(1).collect::<String>().parse().unwrap();

        match action.as_str() {
            // Action N means to move the waypoint north by the given value.
            "N" => ship.waypoint.1 -= value,
            // Action S means to move the waypoint south by the given value.
            "S" => ship.waypoint.1 += value,
            // Action E means to move the waypoint east by the given value.
            "E" => ship.waypoint.0 += value,
            // Action W means to move the waypoint west by the given value.
            "W" => ship.waypoint.0 -= value,

            // https://en.wikipedia.org/wiki/Rotation_matrix#In_two_dimensions
            // New coordinates (x′, y′) of a point (x, y) after rotation are
            // x' = x*cos(angle) - y*sin(angle)
            // y' = x*sin(angle) + y*cos(angle)

            // Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
            "L" => {
                let x = ship.waypoint.0 as f64;
                let y = ship.waypoint.1 as f64;
                let angle = (-value as f64).to_radians();

                ship.waypoint.0 = (x * angle.cos() - y * angle.sin()).round() as i32;
                ship.waypoint.1 = (x * angle.sin() + y * angle.cos()).round() as i32;
            },
            // Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
            "R" => {
                let x = ship.waypoint.0 as f64;
                let y = ship.waypoint.1 as f64;
                let angle = (value as f64).to_radians();

                ship.waypoint.0 = (x * angle.cos() - y * angle.sin()).round() as i32;
                ship.waypoint.1 = (x * angle.sin() + y * angle.cos()).round() as i32;
            },
            // Action F means to move forward to the waypoint a number of times equal to the given value.
            "F" => {
                ship.position.0 += value * ship.waypoint.0;
                ship.position.1 += value * ship.waypoint.1;
            },
            act => panic!("Unhandled action {}", act)
        }
    }

    return ship.position.0.abs() + ship.position.1.abs();
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "F10\n\
                 N3\n\
                 F7\n\
                 R90\n\
                 F11"
            ),
            25
        )
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "F10\n\
                 N3\n\
                 F7\n\
                 R90\n\
                 F11"
            ),
            286
        )
    }
}
