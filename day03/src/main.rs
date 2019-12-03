use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../input.txt");

fn part_1(input: &str) -> i32 {
    let coordinates: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|wire_str| wire_str
            .split(',')
            .map(|vector_str| {
                let vector = vector_str.split_at(1);
                return (vector.0, vector.1.parse::<i32>().unwrap())
            })
            .fold(vec![(0i32, 0i32)], |mut acc, vector| {
                let current: (i32, i32) = *acc.last().unwrap();

                for step in 1..=vector.1 {
                    match vector.0 {
                        "U" => acc.push((current.0, current.1 + step)),
                        "D" => acc.push((current.0, current.1 - step)),
                        "L" => acc.push((current.0 - step, current.1)),
                        "R" => acc.push((current.0 + step, current.1)),
                        _ => ()
                    }
                }

                return acc;
            })
        )
        .collect();

    // println!("Coordinates: {:?}", coordinates);

    let wire_1: HashSet<(i32, i32)> = coordinates[0].iter().cloned().collect();
    let wire_2: HashSet<(i32, i32)> = coordinates[1].iter().cloned().collect();

    let intersections = wire_1.intersection(&wire_2);

    // println!("Coordinates: {:?}", intersections);

    let mut min_distance = std::i32::MAX;

    for intersection in intersections {
        let distance = intersection.0.abs() + intersection.1.abs();
        // println!("Distance {:?}: {:?}", intersection, distance);

        if distance > 0 && distance < min_distance {
            min_distance = distance;
        }
        
    }

    return min_distance;
}

fn main() -> () {
    let part1_distance = part_1(INPUT_FILE);

    println!("Part 1: {}", part1_distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(part_1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(part_1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 159);
    }

    #[test]
    fn it_solves_part1_example_3() {
        assert_eq!(part_1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
    }
}
