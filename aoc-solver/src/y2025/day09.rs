use crate::solution::{AocError, Solution};
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    start: Point,
    end: Point,
}

pub struct Day09;

fn parse(input: &str) -> Result<Vec<Point>, AocError> {
    let corners = input
        .trim()
        .lines()
        .map(|line| match line.split_once(",") {
            Some((x, y)) => {
                let x = x.parse::<i64>().map_err(|err| AocError::parse(x, err))?;
                let y = y.parse::<i64>().map_err(|err| AocError::parse(y, err))?;

                Ok(Point { x, y })
            }
            _ => Err(AocError::parse(line, "invalid input")),
        })
        .try_collect()?;

    Ok(corners)
}

fn area(a: Point, b: Point) -> u64 {
    ((a.x - b.x).unsigned_abs() + 1) * ((a.y - b.y).unsigned_abs() + 1)
}

fn is_on_edge(p: Point, segment: &Segment) -> bool {
    let Segment { start, end } = segment;

    let is_vertical = start.y == end.y;
    let is_horizontal = start.x == end.x;

    if is_vertical {
        if p.x != start.x {
            return false;
        }

        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);

        return p.y >= min_y && p.y <= max_y;
    }

    if is_horizontal {
        if p.y != start.y {
            return false;
        }

        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);

        return p.x >= min_x && p.x <= max_x;
    }

    false
}

fn is_inside(p: Point, segments: &[Segment]) -> bool {
    if segments.iter().any(|s| is_on_edge(p, s)) {
        return true;
    }

    // https://en.wikipedia.org/wiki/Even–odd_rule
    let mut hits = 0;

    for segment in segments {
        let Segment { start, end } = segment;

        let is_vertical = start.x == end.x;
        if !is_vertical {
            continue;
        }

        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);

        // Cast a horizontal ray to right and check if it hits vertical segment
        //
        //     SEGMENT
        //        max_y
        //        |
        //        <start.x
        // ^------+---------> RAY
        // p(x,y) |
        //        min_y
        //
        if p.x < start.x && p.y >= min_y && p.y < max_y {
            hits += 1;
        }
    }

    hits % 2 == 1
}

fn is_between(a: i64, b: i64, c: i64) -> bool {
    let low = a.min(b);
    let high = a.max(b);
    c > low && c < high
}

fn is_intersecting(a: &Segment, b: &Segment) -> bool {
    let a_vertical = a.start.x == a.end.x;
    let a_horizontal = a.start.y == a.end.y;

    let b_vertical = b.start.x == b.end.x;
    let b_horizontal = b.start.y == b.end.y;

    // Skip parallel segments
    if (a_horizontal && b_horizontal) || (a_vertical && b_vertical) {
        return false;
    }

    //    | (b_x, a_y)
    // ---+-- a
    //    |
    //    b
    if a_horizontal && b_vertical {
        return is_between(a.start.x, a.end.x, b.start.x)
            && is_between(b.start.y, b.end.y, a.start.y);
    }

    //    | (a_x, b_y)
    // ---+-- b
    //    |
    //    a
    if a_vertical && b_horizontal {
        return is_between(b.start.x, b.end.x, a.start.x)
            && is_between(a.start.y, a.end.y, b.start.y);
    }

    false
}

fn build_segments(points: &[Point]) -> Vec<Segment> {
    let mut segments = Vec::new();

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        segments.push(Segment { start, end });
    }

    segments
}

impl Solution for Day09 {
    type Part1 = u64;
    type Part2 = u64;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2025/day09.txt")
    }

    fn part_1(&self, input: &str) -> Result<u64, AocError> {
        let corners = parse(input)?;

        let max = corners
            .into_iter()
            .tuple_combinations()
            .map(|(a, b)| area(a, b))
            .max()
            .ok_or(AocError::logic("couldn't find largest rectangel"))?;

        Ok(max)
    }

    fn part_2(&self, input: &str) -> Result<u64, AocError> {
        let corners = parse(input)?;
        let segments = build_segments(&corners);

        let areas = corners
            .iter()
            .copied()
            .tuple_combinations()
            .map(|(a, b)| (a, b, area(a, b)))
            .sorted_by(|(_, _, a), (_, _, b)| b.cmp(a));

        for (a, b, size) in areas {
            let c = Point { x: a.x, y: b.y };
            let d = Point { x: b.x, y: a.y };

            // For a rectangle to fit inside...
            // 1) All of its vertices must be inside the polygon
            // 2) None of the edges of the rectangle can cross any line

            if ![a, b, c, d].into_iter().all(|p| is_inside(p, &segments)) {
                continue;
            }

            let edges = [
                Segment { start: a, end: c },
                Segment { start: c, end: b },
                Segment { start: b, end: d },
                Segment { start: d, end: a },
            ];

            let fits_inside = edges.iter().all(|edge| {
                segments
                    .iter()
                    .all(|segment| !is_intersecting(edge, segment))
            });

            if fits_inside {
                return Ok(size);
            }
        }

        Err(AocError::logic("nothing fits"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1\n\
                           11,1\n\
                           11,7\n\
                           9,7\n\
                           9,5\n\
                           2,5\n\
                           2,3\n\
                           7,3";

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(Day09.part_1(EXAMPLE), Ok(50));
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(Day09.part_2(EXAMPLE), Ok(24));
    }

    #[test]
    fn it_checks_for_intersection_1() {
        let a = Segment {
            start: Point { x: 0, y: 0 },
            end: Point { x: 4, y: 0 },
        };
        let b = Segment {
            start: Point { x: 0, y: 1 },
            end: Point { x: 4, y: 1 },
        };

        assert!(!is_intersecting(&a, &b));
    }

    #[test]
    fn it_checks_for_intersection_2() {
        let a = Segment {
            start: Point { x: 2, y: 3 },
            end: Point { x: 6, y: 3 },
        };
        let b = Segment {
            start: Point { x: 4, y: 5 },
            end: Point { x: 4, y: 1 },
        };

        assert!(is_intersecting(&a, &b));
    }

    #[test]
    fn it_checks_for_intersection_3() {
        let a = Segment {
            start: Point { x: 1, y: 0 },
            end: Point { x: 1, y: 4 },
        };
        let b = Segment {
            start: Point { x: 1, y: 2 },
            end: Point { x: 4, y: 2 },
        };

        assert!(!is_intersecting(&a, &b));
    }

    #[test]
    fn it_checks_for_intersection_4() {
        let a = Segment {
            start: Point { x: 9, y: 5 },
            end: Point { x: 2, y: 5 },
        };
        let b = Segment {
            start: Point { x: 7, y: 3 },
            end: Point { x: 7, y: 1 },
        };

        assert!(!is_intersecting(&a, &b));
    }

    #[test]
    fn it_checks_for_intersection_5() {
        let a = Segment {
            start: Point { x: 4992, y: 64223 },
            end: Point { x: 4992, y: 50076 },
        };
        let b = Segment {
            start: Point { x: 2484, y: 50076 },
            end: Point { x: 94607, y: 50076 },
        };

        assert!(!is_intersecting(&a, &b));
    }
}
