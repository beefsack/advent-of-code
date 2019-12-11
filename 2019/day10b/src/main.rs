use anyhow::{anyhow, Result};
use num;

use std::collections::{HashMap, HashSet};
use std::f64::consts::{FRAC_PI_2, PI};
use std::io::{stdin, Read};
use std::iter::FromIterator;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq)]
enum MapItem {
    Empty,
    Asteroid,
}

const MAP_ITEM_EMPTY_CHAR: char = '.';
const MAP_ITEM_ASTEROID_CHAR: char = '#';

impl MapItem {
    fn from_char(c: char) -> Result<Self> {
        match c {
            MAP_ITEM_EMPTY_CHAR => Ok(MapItem::Empty),
            MAP_ITEM_ASTEROID_CHAR => Ok(MapItem::Asteroid),
            _ => Err(anyhow!("'{}' not a valid map item", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map(Vec<Vec<MapItem>>);

impl Map {
    fn parse(input: &str) -> Result<Self> {
        Ok(Self(
            input
                .trim()
                .lines()
                .map(Map::parse_line)
                .collect::<Result<Vec<Vec<MapItem>>>>()?,
        ))
    }

    fn parse_line(input: &str) -> Result<Vec<MapItem>> {
        input.chars().map(MapItem::from_char).collect()
    }

    fn find(&self, kind: &MapItem) -> Vec<Point> {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, item)| {
                        if item == kind {
                            Some(Point {
                                x: x as isize,
                                y: y as isize,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .flatten()
            .collect()
    }

    fn visible_pairs(&self) -> Vec<[Point; 2]> {
        let asteroids = self.find(&MapItem::Asteroid);
        let asteroids_set: HashSet<Point> = HashSet::from_iter(asteroids.to_owned());
        let mut pairs: Vec<[Point; 2]> = vec![];
        for (index, a1) in asteroids.iter().enumerate().take(asteroids.len() - 1) {
            'a2: for a2 in asteroids.iter().skip(index + 1) {
                let step = (*a2 - *a1).smallest_whole_vector();
                let mut ptr = *a1 + step;
                while ptr != *a2 {
                    if asteroids_set.contains(&ptr) {
                        // There is an asteroid in the way, skip this pair
                        continue 'a2;
                    }
                    ptr = ptr + step;
                }
                pairs.push([*a1, *a2]);
            }
        }
        pairs
    }

    fn most_visible(&self) -> (Vec<Point>, usize) {
        let mut asteroid_visible: HashMap<Point, usize> = HashMap::new();
        let mut most = 0;
        for pair in self.visible_pairs() {
            for a in pair.iter() {
                let e = asteroid_visible.entry(*a).or_default();
                *e += 1;
                if *e > most {
                    most = *e;
                }
            }
        }
        (
            asteroid_visible
                .into_iter()
                .filter_map(|(a, n)| if n == most { Some(a) } else { None })
                .collect(),
            most,
        )
    }

    fn laser_targets(&self, from: Point) -> Vec<Point> {
        // Find the angles of all other points
        let mut asteroids: Vec<RelativeAsteroid> = self
            .find(&MapItem::Asteroid)
            .into_iter()
            .filter_map(|a| {
                if a != from {
                    let diff = a - from;
                    let angle_diff = diff.smallest_whole_vector();
                    Some(RelativeAsteroid {
                        at: a,
                        angle: (-FRAC_PI_2 - (angle_diff.y as f64).atan2(-angle_diff.x as f64)
                            + 2f64 * PI)
                            % (2f64 * PI),
                        dist: a.length(),
                    })
                } else {
                    None
                }
            })
            .collect();
        asteroids.sort_by(|a, b| {
            a.angle
                .partial_cmp(&b.angle)
                .unwrap()
                .then(a.dist.partial_cmp(&b.dist).unwrap())
        });
        let mut targets: Vec<Point> = vec![];
        let mut index = 0;
        let mut last_angle: Option<f64> = None;
        loop {
            if Some(asteroids[index].angle) != last_angle {
                last_angle = Some(asteroids[index].angle);
                targets.push(asteroids[index].at);
                asteroids.remove(index);
                // We don't increment index as we removed the current item anyway
                if asteroids.is_empty() {
                    break;
                }
            } else {
                index += 1;
            }
            index %= asteroids.len();
        }
        targets
    }
}

#[derive(Debug)]
struct RelativeAsteroid {
    at: Point,
    angle: f64,
    dist: f64,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
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

impl Point {
    fn smallest_whole_vector(self) -> Point {
        match self {
            Point { x: 0, y: 0 } => self,
            Point { x, y: 0 } => Point {
                x: x.signum(),
                y: 0,
            },
            Point { x: 0, y } => Point {
                x: 0,
                y: y.signum(),
            },
            Point { x, y } => {
                let gcd = num::integer::gcd(x, y);
                Point {
                    x: x / gcd,
                    y: y / gcd,
                }
            }
        }
    }

    fn length(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;
    let map = Map::parse(&input)?;
    let (asteroids, _) = map.most_visible();
    let asteroid = asteroids
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("could not find asteroid"))?;
    println!("{:?}", map.laser_targets(asteroid).get(199));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_map_item_from_char() -> Result<()> {
        assert_eq!(MapItem::from_char(MAP_ITEM_EMPTY_CHAR)?, MapItem::Empty);
        assert_eq!(
            MapItem::from_char(MAP_ITEM_ASTEROID_CHAR)?,
            MapItem::Asteroid
        );
        assert_eq!(MapItem::from_char('z').is_err(), true);
        Ok(())
    }

    #[test]
    fn test_map_parse_line() -> Result<()> {
        assert_eq!(
            Map::parse_line(".#..#")?,
            vec![
                MapItem::Empty,
                MapItem::Asteroid,
                MapItem::Empty,
                MapItem::Empty,
                MapItem::Asteroid,
            ]
        );
        Ok(())
    }

    #[test]
    fn test_map_parse() -> Result<()> {
        assert_eq!(
            Map::parse(
                "
.#..#
.....
#####
....#
...##
"
            )?,
            Map(vec![
                vec![
                    MapItem::Empty,
                    MapItem::Asteroid,
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Asteroid,
                ],
                vec![
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Empty,
                ],
                vec![
                    MapItem::Asteroid,
                    MapItem::Asteroid,
                    MapItem::Asteroid,
                    MapItem::Asteroid,
                    MapItem::Asteroid,
                ],
                vec![
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Asteroid,
                ],
                vec![
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Empty,
                    MapItem::Asteroid,
                    MapItem::Asteroid,
                ],
            ]),
        );
        Ok(())
    }

    #[test]
    fn test_map_find() -> Result<()> {
        let map = Map::parse(
            "
.#.
...
###
",
        )?;
        assert_eq!(
            map.find(&MapItem::Asteroid),
            vec![
                Point { x: 1, y: 0 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
            ]
        );
        Ok(())
    }

    #[test]
    fn test_map_visible_pairs() -> Result<()> {
        let map = Map::parse(
            "
##.
.#.
###
",
        )?;
        assert_eq!(
            map.visible_pairs(),
            vec![
                [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }],
                [Point { x: 0, y: 0 }, Point { x: 1, y: 1 }],
                [Point { x: 0, y: 0 }, Point { x: 0, y: 2 }],
                [Point { x: 0, y: 0 }, Point { x: 1, y: 2 }],
                [Point { x: 1, y: 0 }, Point { x: 1, y: 1 }],
                [Point { x: 1, y: 0 }, Point { x: 0, y: 2 }],
                [Point { x: 1, y: 0 }, Point { x: 2, y: 2 }],
                [Point { x: 1, y: 1 }, Point { x: 0, y: 2 }],
                [Point { x: 1, y: 1 }, Point { x: 1, y: 2 }],
                [Point { x: 1, y: 1 }, Point { x: 2, y: 2 }],
                [Point { x: 0, y: 2 }, Point { x: 1, y: 2 }],
                [Point { x: 1, y: 2 }, Point { x: 2, y: 2 }],
            ]
        );
        Ok(())
    }

    #[test]
    fn test_point_smallest_whole_vector() {
        assert_eq!(
            Point { x: 0, y: 0 }.smallest_whole_vector(),
            Point { x: 0, y: 0 }
        );
        assert_eq!(
            Point { x: 0, y: 5 }.smallest_whole_vector(),
            Point { x: 0, y: 1 }
        );
        assert_eq!(
            Point { x: 0, y: -5 }.smallest_whole_vector(),
            Point { x: 0, y: -1 }
        );
        assert_eq!(
            Point { x: 5, y: 0 }.smallest_whole_vector(),
            Point { x: 1, y: 0 }
        );
        assert_eq!(
            Point { x: -5, y: 0 }.smallest_whole_vector(),
            Point { x: -1, y: 0 }
        );
        assert_eq!(
            Point { x: 3, y: 3 }.smallest_whole_vector(),
            Point { x: 1, y: 1 }
        );
        assert_eq!(
            Point { x: 4, y: 6 }.smallest_whole_vector(),
            Point { x: 2, y: 3 }
        );
        assert_eq!(
            Point { x: 7, y: -10 }.smallest_whole_vector(),
            Point { x: 7, y: -10 }
        );
    }

    #[test]
    fn test_map_most_visible() -> Result<()> {
        let mut map = Map::parse(
            "
.#..#
.....
#####
....#
...##
",
        )?;
        assert_eq!(map.most_visible(), (vec![Point { x: 3, y: 4 }], 8));

        map = Map::parse(
            "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
",
        )?;
        assert_eq!(map.most_visible(), (vec![Point { x: 5, y: 8 }], 33));

        map = Map::parse(
            "
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
",
        )?;
        assert_eq!(map.most_visible(), (vec![Point { x: 1, y: 2 }], 35));

        map = Map::parse(
            "
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
",
        )?;
        assert_eq!(map.most_visible(), (vec![Point { x: 6, y: 3 }], 41));

        map = Map::parse(
            "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
",
        )?;
        assert_eq!(map.most_visible(), (vec![Point { x: 11, y: 13 }], 210));

        Ok(())
    }

    #[test]
    fn test_map_laser_targets() -> Result<()> {
        let map = Map::parse(
            "
###
###
###
",
        )?;
        assert_eq!(
            map.laser_targets(Point { x: 1, y: 1 }),
            vec![
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 0 },
            ]
        );
        assert_eq!(
            map.laser_targets(Point { x: 0, y: 1 }),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 0, y: 2 },
                Point { x: 2, y: 1 },
            ]
        );
        Ok(())
    }
}
