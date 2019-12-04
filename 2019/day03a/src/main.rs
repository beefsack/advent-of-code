use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io::{stdin, BufRead};
use std::ops::{Add, Sub};

fn main() {
    println!(
        "{}",
        match closest_intersection_dist(
            &stdin()
                .lock()
                .lines()
                .map(|l| parse_wire(&l.unwrap()).unwrap())
                .collect::<Vec<Wire>>()
        ) {
            Some(dist) => format!("{}", dist),
            None => "".to_string(),
        }
    );
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn unit(&self) -> Point {
        match self {
            Dir::Up => Point { x: 0, y: -1 },
            Dir::Down => Point { x: 0, y: 1 },
            Dir::Left => Point { x: -1, y: 0 },
            Dir::Right => Point { x: 1, y: 0 },
        }
    }
}

type Wire = Vec<WirePath>;

#[derive(Debug, PartialEq, Eq)]
struct WirePath {
    dir: Dir,
    dist: isize,
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn dist(self, other: Point) -> isize {
        let diff = self - other;
        diff.x.checked_abs().unwrap() + diff.y.checked_abs().unwrap()
    }
}

const ORIGIN: Point = Point { x: 0, y: 0 };

fn intersections(wires: &[Wire]) -> Vec<Point> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut intersections: Vec<Point> = vec![];
    for wire in wires {
        let mut cur: Point = ORIGIN;
        let mut new_visited: HashSet<Point> = HashSet::new();
        for path in wire {
            let unit = path.dir.unit();
            for _ in 0..path.dist {
                cur = cur + unit;
                if visited.contains(&cur) {
                    intersections.push(cur);
                } else {
                    new_visited.insert(cur);
                }
            }
        }
        for p in new_visited {
            visited.insert(p);
        }
    }
    intersections
}

fn closest_point(to: Point, pts: &[Point]) -> Option<isize> {
    pts.iter().map(|p| to.dist(*p)).min()
}

fn closest_intersection_dist(wires: &[Wire]) -> Option<isize> {
    closest_point(ORIGIN, &intersections(wires))
}

fn parse_wire_path(input: &str) -> Result<WirePath> {
    if input.len() < 2 {
        return Err(anyhow!("Expected minimum length 2, got: {}", input.len()));
    }
    let mut chars = input.chars();
    let dir = match chars.next() {
        Some('U') => Dir::Up,
        Some('D') => Dir::Down,
        Some('L') => Dir::Left,
        Some('R') => Dir::Right,
        _ => return Err(anyhow!("")),
    };
    Ok(WirePath {
        dir,
        dist: chars.collect::<String>().parse()?,
    })
}

fn parse_wire(input: &str) -> Result<Wire> {
    input.split(",").map(parse_wire_path).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        assert_eq!(ORIGIN.dist(Point { x: 1, y: 1 }), 2);
        assert_eq!(ORIGIN.dist(Point { x: -1, y: -1 }), 2);
    }

    #[test]
    fn test_parse_wire_path() {
        assert_eq!(
            parse_wire_path("U5").unwrap(),
            WirePath {
                dir: Dir::Up,
                dist: 5,
            },
        );
        assert_eq!(
            parse_wire_path("D4").unwrap(),
            WirePath {
                dir: Dir::Down,
                dist: 4,
            },
        );
        assert_eq!(
            parse_wire_path("L35").unwrap(),
            WirePath {
                dir: Dir::Left,
                dist: 35,
            },
        );
        assert_eq!(
            parse_wire_path("R999").unwrap(),
            WirePath {
                dir: Dir::Right,
                dist: 999,
            },
        );
    }

    #[test]
    fn test_parse_wire() {
        assert_eq!(
            parse_wire("R8,U5,L5,D3").unwrap(),
            vec![
                WirePath {
                    dir: Dir::Right,
                    dist: 8,
                },
                WirePath {
                    dir: Dir::Up,
                    dist: 5,
                },
                WirePath {
                    dir: Dir::Left,
                    dist: 5,
                },
                WirePath {
                    dir: Dir::Down,
                    dist: 3,
                },
            ],
        );
    }

    #[test]
    fn test_intersections() {
        assert_eq!(
            intersections(&vec![
                parse_wire("R8,U5,L5,D3").unwrap(),
                parse_wire("U7,R6,D4,L4").unwrap(),
            ]),
            vec![Point { x: 6, y: -5 }, Point { x: 3, y: -3 }],
        );
    }

    #[test]
    fn test_closest_intersection_dist() {
        assert_eq!(
            closest_intersection_dist(&vec![
                parse_wire("R8,U5,L5,D3").unwrap(),
                parse_wire("U7,R6,D4,L4").unwrap(),
            ]),
            Some(6),
        );
        assert_eq!(
            closest_intersection_dist(&vec![
                parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap(),
                parse_wire("U62,R66,U55,R34,D71,R55,D58,R83").unwrap(),
            ]),
            Some(159),
        );
        assert_eq!(
            closest_intersection_dist(&vec![
                parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap(),
                parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap(),
            ]),
            Some(135),
        );
    }
}
