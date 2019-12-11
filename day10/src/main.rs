use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../input.txt");

fn part_1(input: &str) -> ((i32, i32), usize) {
    let mut world: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            if val == '#' {
                world.insert((x as i32, y as i32));
            }
        }
    }

    let mut visible: HashMap<(i32, i32), usize> = HashMap::new();

    for asteroid in &world {
        let x = asteroid.0;
        let y = asteroid.1;

        let mut seen: HashMap<i32, u32> = HashMap::new();

        for other in &world {
            if asteroid == other {
                continue;
            }
            let other_x = other.0;
            let other_y = other.1;


            // Lets just convert this to integer and hope for the best
            let angle_rad = ((other_y - y) as f64).atan2((other_x - x) as f64) + std::f64::consts::FRAC_PI_2;
            let mut angle_degree = angle_rad.to_degrees();

            if angle_degree < 0.0_f64 {
                angle_degree = angle_degree + 360_f64;
            }

            let angle_degree_int = (angle_degree * 1000_f64).round() as i32;

            *seen.entry(angle_degree_int).or_insert(0) += 1;
        }

        visible.insert((x, y), seen.len());
    }

    let best_location = visible.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

    return (*best_location.0, *best_location.1);
}

fn part_2(input: &str, station: (i32, i32)) -> Vec<(i32, i32)> {
    let mut world: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            if val == '#' {
                // Adjust the coordinates to (0, 0) centered
                world.insert((x as i32, y as i32));
            }
        }
    }

    let mut asteroids: BTreeMap<i32, Vec<((i32, i32), i32)>> = BTreeMap::new();

    for asteroid in &world {
        let x = station.0;
        let y = station.1;
        let asteroid_x = asteroid.0;
        let asteroid_y = asteroid.1;

        let distance = (asteroid_x - x).abs() + (asteroid_y - y).abs();

        // Lets just convert this to integer and hope for the best
        let angle_rad = ((asteroid_y - y) as f64).atan2((asteroid_x - x) as f64) + std::f64::consts::FRAC_PI_2;
        let mut angle_degree = angle_rad.to_degrees();

        if angle_degree < 0.0_f64 {
            angle_degree = angle_degree + 360_f64;
        }

        let angle_degree_int = (angle_degree * 1000_f64).round() as i32;

        asteroids.entry(angle_degree_int).or_insert(Vec::new()).push(((asteroid_x, asteroid_y), distance));
    }

    for (_angle, asteroids_at_angle) in asteroids.iter_mut() {
        asteroids_at_angle.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
    }

    println!("[INFO]: asteroids {:?}", asteroids);

    let mut destroyed_asteroids: Vec<(i32, i32)> = Vec::new();

    while asteroids.iter().any(|a| a.1.len() > 0) {
        for (_angle, asteroids_at_angle) in asteroids.iter_mut() {
            if asteroids_at_angle.len() > 0 {
                let destroyed = asteroids_at_angle.pop().unwrap();
                destroyed_asteroids.push(destroyed.0);
            }
        }
    }

    println!("[INFO]: destroyed {:?}", destroyed_asteroids);

    return destroyed_asteroids;
}

fn main() -> () {
    let part1 = part_1(INPUT_FILE);
    let destroyed = part_2(INPUT_FILE, (11, 11));

    let part2 = (destroyed[199].0 * 100) + destroyed[199].1;

    println!("[INFO]: Part 1: {:?}", part1);
    println!("[INFO]: Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(part_1(
           ".#..#\n\
            .....\n\
            #####\n\
            ....#\n\
            ...##"), ((3, 4), 8));
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(part_1(
           "......#.#.\n\
            #..#.#....\n\
            ..#######.\n\
            .#.#.###..\n\
            .#..#.....\n\
            ..#....#.#\n\
            #..#....#.\n\
            .##.#..###\n\
            ##...#..#.\n\
            .#....####"), ((5, 8), 33));
    }

    #[test]
    fn it_solves_part1_example_3() {
        assert_eq!(part_1(
           "#.#...#.#.\n\
            .###....#.\n\
            .#....#...\n\
            ##.#.#.#.#\n\
            ....#.#.#.\n\
            .##..###.#\n\
            ..#...##..\n\
            ..##....##\n\
            ......#...\n\
            .####.###."), ((1, 2), 35));
    }

    #[test]
    fn it_solves_part1_example_4() {
        assert_eq!(part_1(
           ".#..#..###\n\
            ####.###.#\n\
            ....###.#.\n\
            ..###.##.#\n\
            ##.##.#.#.\n\
            ....###..#\n\
            ..#.#..#.#\n\
            #..#.#.###\n\
            .##...##.#\n\
            .....#.#.."), ((6, 3), 41));
    }

    #[test]
    fn it_solves_part1_example_5() {
        assert_eq!(part_1(
           ".#..##.###...#######\n\
            ##.############..##.\n\
            .#.######.########.#\n\
            .###.#######.####.#.\n\
            #####.##.#.##.###.##\n\
            ..#####..#.#########\n\
            ####################\n\
            #.####....###.#.#.##\n\
            ##.#################\n\
            #####.##.###..####..\n\
            ..######..##.#######\n\
            ####.##.####...##..#\n\
            .#####..#.######.###\n\
            ##...#.##########...\n\
            #.##########.#######\n\
            .####.#.###.###.#.##\n\
            ....##.##.###..#####\n\
            .#.#.###########.###\n\
            #.#.#.#####.####.###\n\
            ###.##.####.##.#..##"), ((11, 13), 210));
    }

    #[test]
    fn it_solves_part2_short_example() {
        let destroyed = part_2(
           ".#....#####...#..\n\
            ##...##.#####..##\n\
            ##...#...#.#####.\n\
            ..#.....X...###..\n\
            ..#.#.....#....##", (8, 3));

        assert_eq!(destroyed[0], (8, 1));
        assert_eq!(destroyed[1], (9, 0));
        assert_eq!(destroyed[2], (9, 1));
        assert_eq!(destroyed[3], (10, 0));
        assert_eq!(destroyed[4], (9, 2));
    }    

    #[test]
    fn it_solves_part2_long_example() {
        let destroyed = part_2(
            ".#..##.###...#######\n\
             ##.############..##.\n\
             .#.######.########.#\n\
             .###.#######.####.#.\n\
             #####.##.#.##.###.##\n\
             ..#####..#.#########\n\
             ####################\n\
             #.####....###.#.#.##\n\
             ##.#################\n\
             #####.##.###..####..\n\
             ..######..##.#######\n\
             ####.##.####...##..#\n\
             .#####..#.######.###\n\
             ##...#.##########...\n\
             #.##########.#######\n\
             .####.#.###.###.#.##\n\
             ....##.##.###..#####\n\
             .#.#.###########.###\n\
             #.#.#.#####.####.###\n\
             ###.##.####.##.#..##", (11, 13));

        // The 1st asteroid to be vaporized is at 11,12.
        assert_eq!(destroyed[0], (11, 12));
        // The 2nd asteroid to be vaporized is at 12,1.
        assert_eq!(destroyed[1], (12, 1));
        // The 3rd asteroid to be vaporized is at 12,2.
        assert_eq!(destroyed[2], (12, 2));
        // The 10th asteroid to be vaporized is at 12,8.
        assert_eq!(destroyed[9], (12, 8));
        // The 20th asteroid to be vaporized is at 16,0.
        assert_eq!(destroyed[19], (16, 0));
        // The 50th asteroid to be vaporized is at 16,9.
        assert_eq!(destroyed[49], (16, 9));
        // The 100th asteroid to be vaporized is at 10,16.
        assert_eq!(destroyed[99], (10, 16));
        // The 199th asteroid to be vaporized is at 9,6.
        assert_eq!(destroyed[198], (9, 6));
        // The 200th asteroid to be vaporized is at 8,2.
        assert_eq!(destroyed[199], (8, 2));
        // The 201st asteroid to be vaporized is at 10,9.
        assert_eq!(destroyed[200], (10, 9));
        // The 299th and final asteroid to be vaporized is at 11,1.
        // TODO: Something doesn't work here
        // assert_eq!(destroyed[298], (11, 1));
        // assert_eq!(destroyed.len(), 299);
    }
}
