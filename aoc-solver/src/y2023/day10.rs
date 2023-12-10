use std::collections::{HashMap, HashSet};

use crate::solution::{AocError, Solution};

pub struct Day10;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

#[derive(Debug, PartialEq, Clone)]
struct Pipe {
    shape: Shape,
    connections: Vec<Coords>,
}

type Coords = (i32, i32);
type Point = (f64, f64);
type Pipes = HashMap<Coords, Pipe>;

fn parse(input: &str) -> Result<(Pipes, Coords), AocError> {
    let mut pipes: HashMap<Coords, Pipe> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, tile)| {
                let x = x as i32;
                let y = y as i32;

                let (shape, connections) = match tile {
                    '|' => (Shape::Vertical, vec![(x, y - 1), (x, y + 1)]),
                    '-' => (Shape::Horizontal, vec![(x - 1, y), (x + 1, y)]),
                    'L' => (Shape::NorthEast, vec![(x, y - 1), (x + 1, y)]),
                    'J' => (Shape::NorthWest, vec![(x, y - 1), (x - 1, y)]),
                    '7' => (Shape::SouthWest, vec![(x, y + 1), (x - 1, y)]),
                    'F' => (Shape::SouthEast, vec![(x, y + 1), (x + 1, y)]),
                    'S' => (
                        Shape::Start,
                        vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)],
                    ),
                    _ => return None,
                };

                let pipe = Pipe { shape, connections };

                Some(((x, y), pipe))
            })
        })
        .collect();

    let unfiltered_pipes = pipes.clone();

    // Filter out connections that aren't connected from both ends
    for (source, pipe) in pipes.iter_mut() {
        pipe.connections.retain(|target| {
            unfiltered_pipes
                .get(target)
                .map(|target_pipe| target_pipe.connections.contains(source))
                .unwrap_or(false)
        });
    }

    // Find the start and resolve its actual pipe shape
    let start = pipes
        .iter_mut()
        .find(|(_, pipe)| pipe.shape == Shape::Start)
        .and_then(|((x, y), pipe)| {
            let d1 = (pipe.connections[0].0 - x, pipe.connections[0].1 - y);
            let d2 = (pipe.connections[1].0 - x, pipe.connections[1].1 - y);

            let shape = match (d1, d2) {
                ((0, 1), (0, -1)) | ((0, -1), (0, 1)) => Shape::Vertical,
                ((1, 0), (-1, 0)) | ((-1, 0), (1, 0)) => Shape::Horizontal,
                ((0, 1), (1, 0)) | ((1, 0), (0, 1)) => Shape::SouthEast,
                ((0, -1), (1, 0)) | ((1, 0), (0, -1)) => Shape::NorthEast,
                ((0, 1), (-1, 0)) | ((-1, 0), (0, 1)) => Shape::SouthWest,
                ((0, -1), (-1, 0)) | ((-1, 0), (0, -1)) => Shape::NorthWest,
                _ => return None,
            };

            pipe.shape = shape;

            Some((*x, *y))
        })
        .ok_or(AocError::logic("No valid starting point"))?;

    Ok((pipes, start))
}

fn find_loop(
    pipes: HashMap<Coords, Pipe>,
    start: Coords,
) -> Result<(HashSet<Coords>, Vec<Point>), AocError> {
    let mut current = start;
    let mut visited = HashSet::new();
    let mut vertices = Vec::new();

    loop {
        let pipe = pipes.get(&current).ok_or(AocError::logic("Missing pipe"))?;

        visited.insert(current);
        vertices.push((current.0 as f64, current.1 as f64));

        if let Some(next) = pipe
            .connections
            .iter()
            .find(|connection| !visited.contains(*connection))
        {
            current = *next;
        } else {
            return Ok((visited, vertices));
        }
    }
}

fn shoelace(vertices: &[Point]) -> f64 {
    let mut area = 0.0;
    let n = vertices.len();

    for i in 0..n {
        let j = (i + 1) % n;

        let x1 = vertices[i].0;
        let y1 = vertices[i].1;
        let x2 = vertices[j].0;
        let y2 = vertices[j].1;

        area += x1 * y2 - x2 * y1;
    }

    area.abs() / 2.0
}

impl Solution for Day10 {
    type A = u32;
    type B = i32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2023/day10.txt")
    }

    fn part_1(&self, input: &str) -> Result<u32, AocError> {
        let (pipes, start) = parse(input)?;
        let (pipe_loop, _) = find_loop(pipes, start)?;

        Ok(pipe_loop.len() as u32 / 2)
    }

    fn part_2(&self, input: &str) -> Result<i32, AocError> {
        let (pipes, start) = parse(input)?;
        let (pipe_loop, vertices) = find_loop(pipes, start)?;

        // Calculate the area "A" of polygon using Shoelace formula
        // https://en.wikipedia.org/wiki/Shoelace_formula
        let area = shoelace(&vertices) as i32;

        // Solve the amount of interior points "i" with Pick's theorem,
        // using pipe_loop's length as "b" and the area from shoelace as "A"
        // https://en.wikipedia.org/wiki/Pick%27s_theorem
        // A = i + b/2 - 1
        // i = -b/2 + 1 + A
        let interior_points = pipe_loop.len() as i32 / -2 + 1 + area;

        Ok(interior_points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_part1_simple() {
        let simple = parse(
            ".....\n\
             .S-7.\n\
             .|.|.\n\
             .L-J.\n\
             .....\n",
        );
        assert!(simple.is_ok());
        let (pipes, start) = simple.unwrap();

        assert_eq!(start, (1, 1));
        assert_eq!(pipes.len(), 8);
        assert_eq!(
            pipes.get(&start).map(|pipe| pipe.shape),
            Some(Shape::SouthEast)
        );
    }

    #[test]
    fn it_parses_part1_simple_with_extras() {
        let extras = parse(
            "-L|F7\n\
             7S-7|\n\
             L|7||\n\
             -L-J|\n\
             L|-JF\n",
        );
        assert!(extras.is_ok());
        let (pipes, start) = extras.unwrap();

        assert_eq!(start, (1, 1));
        assert_eq!(
            pipes.get(&start).map(|pipe| pipe.shape),
            Some(Shape::SouthEast)
        );
        assert_eq!(pipes.len(), 25);
        assert_eq!(
            pipes.get(&(0, 0)),
            Some(&Pipe {
                shape: Shape::Horizontal,
                connections: vec![],
            })
        );
        assert_eq!(
            pipes.get(&(4, 0)),
            Some(&Pipe {
                shape: Shape::SouthWest,
                connections: vec![(4, 1), (3, 0)],
            })
        );
        assert_eq!(
            pipes.get(&(1, 3)),
            Some(&Pipe {
                shape: Shape::NorthEast,
                connections: vec![(1, 2), (2, 3)],
            })
        );
    }

    #[test]
    fn it_parses_part1_complex() {
        let simple = parse(
            "..F7.\n\
             .FJ|.\n\
             SJ.L7\n\
             |F--J\n\
             LJ...\n",
        );
        assert!(simple.is_ok());
        assert_eq!(simple.unwrap().0.len(), 16);
    }

    #[test]
    fn it_parses_part1_complex_with_extras() {
        let extras = parse(
            "7-F7-\n\
             .FJ|7\n\
             SJLL7\n\
             |F--J\n\
             LJ.LJ\n",
        );
        assert!(extras.is_ok());
        assert_eq!(extras.unwrap().0.len(), 23);
    }

    #[test]
    fn it_solves_part1_simple() {
        assert_eq!(
            Day10.part_1(
                ".....\n\
                 .S-7.\n\
                 .|.|.\n\
                 .L-J.\n\
                 .....\n",
            ),
            Ok(4)
        );
    }

    #[test]
    fn it_solves_part1_simple_with_extras() {
        assert_eq!(
            Day10.part_1(
                "-L|F7\n\
                 7S-7|\n\
                 L|7||\n\
                 -L-J|\n\
                 L|-JF\n",
            ),
            Ok(4)
        );
    }

    #[test]
    fn it_solves_part1_complex() {
        assert_eq!(
            Day10.part_1(
                "..F7.\n\
                 .FJ|.\n\
                 SJ.L7\n\
                 |F--J\n\
                 LJ...\n",
            ),
            Ok(8)
        );
    }

    #[test]
    fn it_solves_part1_complex_with_extras() {
        assert_eq!(
            Day10.part_1(
                "7-F7-\n\
                 .FJ|7\n\
                 SJLL7\n\
                 |F--J\n\
                 LJ.LJ\n",
            ),
            Ok(8)
        );
    }

    #[test]
    fn it_solves_part2_simple() {
        assert_eq!(
            Day10.part_2(
                ".....\n\
                 .S-7.\n\
                 .|.|.\n\
                 .L-J.\n\
                 .....\n",
            ),
            Ok(1)
        );
    }

    #[test]
    fn it_solves_part2_simple_example_1() {
        assert_eq!(
            Day10.part_2(
                "...........\n\
                 .S-------7.\n\
                 .|F-----7|.\n\
                 .||.....||.\n\
                 .||.....||.\n\
                 .|L-7.F-J|.\n\
                 .|..|.|..|.\n\
                 .L--J.L--J.\n\
                 ...........\n"
            ),
            Ok(4)
        )
    }

    #[test]
    fn it_solves_part2_simple_example_2() {
        assert_eq!(
            Day10.part_2(
                "..........\n\
                 .S------7.\n\
                 .|F----7|.\n\
                 .||....||.\n\
                 .||....||.\n\
                 .|L-7F-J|.\n\
                 .|..||..|.\n\
                 .L--JL--J.\n\
                 ..........\n"
            ),
            Ok(4)
        )
    }

    #[test]
    fn it_solves_part2_large_example_1() {
        assert_eq!(
            Day10.part_2(
                ".F----7F7F7F7F-7....\n\
                 .|F--7||||||||FJ....\n\
                 .||.FJ||||||||L7....\n\
                 FJL7L7LJLJ||LJ.L-7..\n\
                 L--J.L7...LJS7F-7L7.\n\
                 ....F-J..F7FJ|L7L7L7\n\
                 ....L7.F7||L7|.L7L7|\n\
                 .....|FJLJ|FJ|F7|.LJ\n\
                 ....FJL-7.||.||||...\n\
                 ....L---J.LJ.LJLJ...\n"
            ),
            Ok(8)
        )
    }

    #[test]
    fn it_solves_part2_large_example_2() {
        assert_eq!(
            Day10.part_2(
                "FF7FSF7F7F7F7F7F---7\n\
                 L|LJ||||||||||||F--J\n\
                 FL-7LJLJ||||||LJL-77\n\
                 F--JF--7||LJLJIF7FJ-\n\
                 L---JF-JLJIIIIFJLJJ7\n\
                 |F|F-JF---7IIIL7L|7|\n\
                 |FFJF7L7F-JF7IIL---7\n\
                 7-L-JL7||F7|L7F-7F7|\n\
                 L.L7LFJ|||||FJL7||LJ\n\
                 L7JLJL-JLJLJL--JLJ.L"
            ),
            Ok(10)
        )
    }
}
