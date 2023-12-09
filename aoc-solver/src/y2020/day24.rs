use std::collections::HashMap;

type CubeCoords = (i32, i32, i32);
type Tiles = HashMap<CubeCoords, bool>;

use crate::solution::{AocError, Solution};

pub struct Day24;

const ADJACENT_AND_SELF_TILES: [CubeCoords; 7] = [
    (0, 0, 0),
    (1, -1, 0),
    (0, -1, 1),
    (-1, 0, 1),
    (-1, 1, 0),
    (0, 1, -1),
    (1, 0, -1),
];

#[derive(Debug, Default)]
pub struct GameOfTiles {
    pub tiles: Tiles,
    pub step: u32,
}

impl GameOfTiles {
    fn new(input: &str) -> Self {
        let mut tiles: Tiles = HashMap::new();

        for coords in input.lines().map(Self::parse_hex_tile) {
            tiles.entry(coords).and_modify(|t| *t = !*t).or_insert(true);
        }

        Self { tiles, step: 0 }
    }

    fn parse_hex_tile(input: &str) -> CubeCoords {
        let directions: HashMap<&str, CubeCoords> = [
            ("e", (1, -1, 0)),
            ("se", (0, -1, 1)),
            ("sw", (-1, 0, 1)),
            ("w", (-1, 1, 0)),
            ("nw", (0, 1, -1)),
            ("ne", (1, 0, -1)),
        ]
        .iter()
        .cloned()
        .collect();

        let mut tile = (0, 0, 0);
        let mut direction = (0, 0, 0);
        let instructions = input.chars().collect::<Vec<char>>();
        let mut iter = instructions.iter();
        while let Some(c) = iter.next() {
            // e, se, sw, w, nw, and ne.
            if *c == 'e' {
                direction = *directions.get("e").unwrap();
            } else if *c == 'w' {
                direction = *directions.get("w").unwrap();
            } else if *c == 's' {
                let c2 = iter.next().unwrap();
                if *c2 == 'e' {
                    direction = *directions.get("se").unwrap();
                } else if *c2 == 'w' {
                    direction = *directions.get("sw").unwrap();
                }
            } else if *c == 'n' {
                let c2 = iter.next().unwrap();
                if *c2 == 'e' {
                    direction = *directions.get("ne").unwrap();
                } else if *c2 == 'w' {
                    direction = *directions.get("nw").unwrap();
                }
            }
            tile = (
                tile.0 + direction.0,
                tile.1 + direction.1,
                tile.2 + direction.2,
            );
        }

        tile
    }

    fn is_black(&self, coords: CubeCoords) -> bool {
        match self.tiles.get(&coords) {
            Some(state) => *state,
            _ => false,
        }
    }

    fn adjacent_active_count(&self, coords: &CubeCoords) -> usize {
        let mut count = 0;
        for (x, y, z) in ADJACENT_AND_SELF_TILES.iter() {
            if *x == 0 && *y == 0 && *z == 0 {
                continue;
            }

            if self.is_black((coords.0 + x, coords.1 + y, coords.2 + z)) {
                count += 1;
            }
        }

        count
    }

    fn simulate(&mut self, steps: u32) {
        while self.step < steps {
            let mut new_tiles = self.tiles.clone();
            self.step += 1;

            // The only meaningful tiles that can change colors are the ones within
            // distance of max one from now black tiles
            for (coords, _s) in self.tiles.iter().filter(|(_c, is_black)| **is_black) {
                for (x, y, z) in ADJACENT_AND_SELF_TILES.iter() {
                    let coords = (coords.0 + x, coords.1 + y, coords.2 + z);
                    let state = self.is_black(coords);
                    let adjacent_count = self.adjacent_active_count(&coords);

                    // Any black tile with zero or more than 2 black tiles immediately
                    // adjacent to it is flipped to white.
                    if state && adjacent_count == 0 || adjacent_count > 2 {
                        new_tiles.insert(coords, false);
                    }
                    // Any white tile with exactly 2 black tiles immediately adjacent
                    // to it is flipped to black.
                    else if !state && adjacent_count == 2 {
                        new_tiles.insert(coords, true);
                    }
                }
            }

            self.tiles = new_tiles;
        }
    }

    fn active_tiles_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|(_coords, is_black)| **is_black)
            .count()
    }
}

impl Solution for Day24 {
    type A = usize;
    type B = usize;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2020/day24.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let game = GameOfTiles::new(input);

        Ok(game.active_tiles_count())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut game = GameOfTiles::new(input);
        game.simulate(100);

        Ok(game.active_tiles_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_hex_tiles_correctly() {
        assert_eq!(GameOfTiles::parse_hex_tile("esenee"), (3, -3, 0));
        assert_eq!(GameOfTiles::parse_hex_tile("esew"), (0, -1, 1));
        assert_eq!(GameOfTiles::parse_hex_tile("nwwswee"), (0, 0, 0));
    }

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day24.part_1(
                "sesenwnenenewseeswwswswwnenewsewsw\n\
                 neeenesenwnwwswnenewnwwsewnenwseswesw\n\
                 seswneswswsenwwnwse\n\
                 nwnwneseeswswnenewneswwnewseswneseene\n\
                 swweswneswnenwsewnwneneseenw\n\
                 eesenwseswswnenwswnwnwsewwnwsene\n\
                 sewnenenenesenwsewnenwwwse\n\
                 wenwwweseeeweswwwnwwe\n\
                 wsweesenenewnwwnwsenewsenwwsesesenwne\n\
                 neeswseenwwswnwswswnw\n\
                 nenwswwsewswnenenewsenwsenwnesesenew\n\
                 enewnwewneswsewnwswenweswnenwsenwsw\n\
                 sweneswneswneneenwnewenewwneswswnese\n\
                 swwesenesewenwneswnwwneseswwne\n\
                 enesenwswwswneneswsenwnewswseenwsese\n\
                 wnwnesenesenenwwnenwsewesewsesesew\n\
                 nenewswnwewswnenesenwnesewesw\n\
                 eneswnwswnwsenenwnwnwwseeswneewsenese\n\
                 neswnwewnwnwseenwseesewsenwsweewe\n\
                 wseweeenwnesenwwwswnew"
            ),
            Ok(10)
        );
    }

    #[ignore]
    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day24.part_2(
                "sesenwnenenewseeswwswswwnenewsewsw\n\
                 neeenesenwnwwswnenewnwwsewnenwseswesw\n\
                 seswneswswsenwwnwse\n\
                 nwnwneseeswswnenewneswwnewseswneseene\n\
                 swweswneswnenwsewnwneneseenw\n\
                 eesenwseswswnenwswnwnwsewwnwsene\n\
                 sewnenenenesenwsewnenwwwse\n\
                 wenwwweseeeweswwwnwwe\n\
                 wsweesenenewnwwnwsenewsenwwsesesenwne\n\
                 neeswseenwwswnwswswnw\n\
                 nenwswwsewswnenenewsenwsenwnesesenew\n\
                 enewnwewneswsewnwswenweswnenwsenwsw\n\
                 sweneswneswneneenwnewenewwneswswnese\n\
                 swwesenesewenwneswnwwneseswwne\n\
                 enesenwswwswneneswsenwnewswseenwsese\n\
                 wnwnesenesenenwwnenwsewesewsesesew\n\
                 nenewswnwewswnenesenwnesewesw\n\
                 eneswnwswnwsenenwnwnwwseeswneewsenese\n\
                 neswnwewnwnwseenwseesewsenwsweewe\n\
                 wseweeenwnesenwwwswnew"
            ),
            Ok(2208)
        );
    }
}
