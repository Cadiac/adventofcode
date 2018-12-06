use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../input.txt");

struct Coord {
    x: i32,
    y: i32
}

struct Point {
    closest: i32,
    distance_to_closest: i32,
    multiple_closest: bool
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn read_anomalities(file: &str) -> (HashMap<(i32, i32), i32>, Coord, Coord) {
    let mut anomalities: HashMap<(i32, i32), i32> = HashMap::new();

    let mut min = Coord{x: std::i32::MAX, y: std::i32::MAX};
    let mut max = Coord{x: 0, y: 0};

    for (i, line) in file.lines().enumerate() {
        let coords: Vec<i32> = line
            .split(", ")
            .map(|c| c.parse().expect("an i32"))
            .collect();

        if coords[0] < min.x {
            min.x = coords[0];
        }
        if coords[0] > max.x {
            max.x = coords[0];
        }
        if coords[1] < min.y {
            min.y = coords[1];
        }
        if coords[1] > max.y {
            max.y = coords[1];
        }

        anomalities.insert((coords[0], coords[1]), i as i32);
    };

    (anomalities, min, max)
}

fn part_1(file: &str) -> i32 {
    let (anomalities, min, max) = read_anomalities(file);
    let mut area_sizes: HashMap<i32, i32> = HashMap::new();

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let mut closest: Option<Point> = None;
            for (coords, id) in &anomalities {
                let dist = distance(coords.clone(), (x, y));

                let mut new_closest;
                match closest {
                    None => {
                        new_closest = Some(Point{
                            closest: id.clone(),
                            distance_to_closest: dist,
                            multiple_closest: false
                        })
                    },
                    Some(current_closest) => {
                        if current_closest.distance_to_closest > dist {
                            new_closest = Some(Point{
                                closest: id.clone(),
                                distance_to_closest: dist,
                                multiple_closest: false
                            });
                        } else if current_closest.distance_to_closest == dist {
                            new_closest = Some(Point{
                                closest: id.clone(),
                                distance_to_closest: dist,
                                multiple_closest: true
                            });
                        } else {
                            new_closest = Some(current_closest);
                        }
                    }
                }

                closest = new_closest;
            };

            let result = closest.expect("Should have closest");

            if !result.multiple_closest {
                *area_sizes.entry(result.closest).or_insert(0) += 1;
            }
        }
    }

    let largest_area = area_sizes.iter().max_by_key(|a| a.1);

    *largest_area.expect("Should have largest area").1
}

fn part_2(file: &str, total_distance_limit: i32) -> i32 {
    let (anomalities, min, max) = read_anomalities(file);
    let mut safe_area = 0;

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let mut total_distances = 0;
            for (coords, _id) in &anomalities {
                total_distances += distance(coords.clone(), (x, y));
            };

            if total_distances < total_distance_limit {
                safe_area += 1;
            }
        }
    }

    safe_area
}

fn main() {
    let part1_result = part_1(INPUT_FILE);
    let part2_result = part_2(INPUT_FILE, 10000);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../test/example.txt");

    #[test]
    fn it_solves_day06_part1_example() {
        assert_eq!(part_1(TEST_FILE), 17);
    }

    #[test]
    fn it_solves_day06_part2_example() {
        assert_eq!(part_2(TEST_FILE, 32), 16);
    }

    #[test]
    fn it_calculates_distances_correctly() {
        assert_eq!(distance((0, 0), (1, 1)), 2);
        assert_eq!(distance((0, 0), (4, 4)), 8);
        assert_eq!(distance((4, 4), (4, 4)), 0);
        assert_eq!(distance((4, 4), (0, 4)), 4);
        assert_eq!(distance((4, 4), (0, 0)), 8);
    }
}
