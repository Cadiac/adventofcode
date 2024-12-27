use crate::solution::{AocError, Solution};

pub struct Day25;

type Locks = Vec<Vec<u8>>;
type Keys = Vec<Vec<u8>>;

fn parse(input: &str) -> Result<(Locks, Keys), AocError> {
    let mut locks: Vec<Vec<u8>> = Vec::new();
    let mut keys: Vec<Vec<u8>> = Vec::new();

    for chunk in input.split("\n\n") {
        let is_lock = chunk.starts_with("#####") && chunk.ends_with(".....");

        let item = chunk
            .lines()
            .map(|line| {
                line.chars()
                    .map(|column| if column == '#' { 1 } else { 0 })
                    .collect::<Vec<u8>>()
            })
            .fold(vec![0, 0, 0, 0, 0], |mut acc: Vec<u8>, current: Vec<u8>| {
                for (i, column) in current.iter().enumerate() {
                    acc[i] += column;
                }
                acc
            });

        if is_lock {
            locks.push(item);
        } else {
            keys.push(item);
        }
    }

    Ok((locks, keys))
}

impl Solution for Day25 {
    type Part1 = u32;
    type Part2 = String;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2024/day25.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (locks, keys) = parse(input)?;

        let mut count = 0;

        for lock in locks.iter() {
            for key in keys.iter() {
                let is_fit = lock
                    .iter()
                    .zip(key.iter())
                    .map(|(pin_height, key_height)| pin_height + key_height)
                    .all(|height| height <= 7);

                if is_fit {
                    count += 1
                }
            }
        }

        Ok(count)
    }

    fn part_2(&self, _input: &str) -> Result<String, AocError> {
        Ok([
            "",
            "                               ",
            "               *               ",
            "               ^^              ",
            "              ^^o              ",
            "              o^^              ",
            "              ^^o^             ",
            "             o^^^^o            ",
            "             ^^o^^^^           ",
            "        _______||_______       ",
            "            AoC 2024           ",
        ]
        .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day25.part_1(
                "#####\n\
                 .####\n\
                 .####\n\
                 .####\n\
                 .#.#.\n\
                 .#...\n\
                 .....\n\
                 \n\
                 #####\n\
                 ##.##\n\
                 .#.##\n\
                 ...##\n\
                 ...#.\n\
                 ...#.\n\
                 .....\n\
                 \n\
                 .....\n\
                 #....\n\
                 #....\n\
                 #...#\n\
                 #.#.#\n\
                 #.###\n\
                 #####\n\
                 \n\
                 .....\n\
                 .....\n\
                 #.#..\n\
                 ###..\n\
                 ###.#\n\
                 ###.#\n\
                 #####\n\
                 \n\
                 .....\n\
                 .....\n\
                 .....\n\
                 #....\n\
                 #.#..\n\
                 #.#.#\n\
                 #####"
            ),
            Ok(3)
        );
    }

    #[test]
    fn it_parses_correctly() {
        assert_eq!(
            parse(
                "#####\n\
                 .####\n\
                 .####\n\
                 .####\n\
                 .#.#.\n\
                 .#...\n\
                 .....\n\
                 \n\
                 .....\n\
                 #....\n\
                 #....\n\
                 #...#\n\
                 #.#.#\n\
                 #.###\n\
                 #####"
            ),
            Ok((vec![vec![1, 6, 4, 5, 4]], vec![vec![6, 1, 3, 2, 4]]))
        )
    }
}
