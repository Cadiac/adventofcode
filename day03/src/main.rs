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

    let all_intersections = wire_1.intersection(&wire_2);

    // println!("Coordinates: {:?}", all_intersections);

    let mut min_distance = std::i32::MAX;

    for intersection in all_intersections {
        let distance = intersection.0.abs() + intersection.1.abs();
        // println!("Distance {:?}: {:?}", intersection, distance);

        if distance > 0 && distance < min_distance {
            min_distance = distance;
        }
        
    }

    return min_distance;
}

fn intersections(wire_1: &Vec<(i32, i32, i32)>, wire_2: &Vec<(i32, i32, i32)>) -> Vec<(i32, i32, (i32, i32))> {
    let mut result = Vec::new();

    for coord_1 in wire_1 {
        let closest_intersection = wire_2
            .iter()
            .filter(|coord_2| coord_2.0 == coord_1.0 && coord_2.1 == coord_1.1)
            .min_by(|a, b| a.2.cmp(&b.2));

        match closest_intersection {
            None => (),
            Some(intersection) => {
                result.push((coord_1.0, coord_1.1, (coord_1.2, intersection.2)));
            },
        }
    }

    return result;
}

fn part_2(input: &str) -> i32 {
    let coordinates: Vec<Vec<(i32, i32, i32)>> = input
        .lines()
        .map(|wire_str| wire_str
            .split(',')
            .map(|vector_str| {
                let vector = vector_str.split_at(1);
                return (vector.0, vector.1.parse::<i32>().unwrap())
            })
            .fold(vec![(0i32, 0i32, 0i32)], |mut acc, vector| {
                let current: (i32, i32, i32) = *acc.last().unwrap();

                for step in 1..=vector.1 {
                    match vector.0 {
                        "U" => acc.push((current.0, current.1 + step, current.2 + step)),
                        "D" => acc.push((current.0, current.1 - step, current.2 + step)),
                        "L" => acc.push((current.0 - step, current.1, current.2 + step)),
                        "R" => acc.push((current.0 + step, current.1, current.2 + step)),
                        _ => ()
                    }
                }

                return acc;
            })
        )
        .collect();

    let closest_intersections = intersections(&coordinates[0], &coordinates[1]);

    // println!("Coordinates: {:?}", closest_intersections);

    let mut min_distance = std::i32::MAX;

    for (_x, _y, distances) in closest_intersections {
        let distance = distances.0 + distances.1;
        // println!("Travelled distance ({} {}): {:?}", x, y, distance);

        if distance > 0 && distance < min_distance {
            min_distance = distance;
        }        
    }

    return min_distance;
}

fn main() -> () {
    let part1_distance = part_1(INPUT_FILE);
    let part2_distance = part_2(INPUT_FILE);

    println!("Part 1: {}", part1_distance);
    println!("Part 2: {}", part2_distance);
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

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(part_2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(part_2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 610);
    }

    #[test]
    fn it_solves_part3_example_3() {
        assert_eq!(part_2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
    }
}
