use anyhow::{anyhow, Context, Result};

use helper::point::IPoint2;
use intcptr::{run, Program};

use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, Read};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Scaffold,
    Space,
}

impl Tile {
    fn parse(input: char) -> Result<Self> {
        match input {
            '#' => Ok(Tile::Scaffold),
            '.' => Ok(Tile::Space),
            _ => Err(anyhow!("invalid tile: {}", input)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Scaffold => '#',
                Tile::Space => '.',
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn parse(input: char) -> Result<Self> {
        match input {
            '^' => Ok(Dir::North),
            'v' => Ok(Dir::South),
            '<' => Ok(Dir::West),
            '>' => Ok(Dir::East),
            _ => Err(anyhow!("invalid dir: {}", input)),
        }
    }
}

impl Dir {
    fn unit(self) -> IPoint2 {
        match self {
            Dir::North => IPoint2 { x: 0, y: -1 },
            Dir::South => IPoint2 { x: 0, y: 1 },
            Dir::West => IPoint2 { x: -1, y: 0 },
            Dir::East => IPoint2 { x: 1, y: 0 },
        }
    }

    fn variants() -> Vec<Dir> {
        vec![Dir::North, Dir::South, Dir::West, Dir::East]
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::North => '^',
                Dir::South => 'v',
                Dir::West => '<',
                Dir::East => '>',
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct World {
    tiles: HashMap<IPoint2, Tile>,
    at: IPoint2,
    dir: Dir,
}

impl Default for World {
    fn default() -> Self {
        World {
            tiles: HashMap::new(),
            at: IPoint2::default(),
            dir: Dir::North,
        }
    }
}

impl World {
    fn parse(input: &str) -> Result<Self> {
        let mut world = World::default();
        for (y, line) in input.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                let loc = IPoint2 {
                    x: x as isize,
                    y: y as isize,
                };
                if let Ok(dir) = Dir::parse(tile) {
                    world.tiles.insert(loc, Tile::Scaffold);
                    world.at = loc;
                    world.dir = dir;
                } else if let Ok(t) = Tile::parse(tile) {
                    world.tiles.insert(loc, t);
                } else {
                    return Err(anyhow!("invalid tile at ({},{}): {}", x, y, tile));
                }
            }
        }
        Ok(world)
    }

    fn bounds(&self) -> (IPoint2, IPoint2) {
        if self.tiles.is_empty() {
            return (IPoint2::default(), IPoint2::default());
        }
        let mut locs = self.tiles.keys();
        let first = locs.next().unwrap();
        let mut min = *first;
        let mut max = *first;
        for l in locs {
            min.x = isize::min(min.x, l.x);
            min.y = isize::min(min.y, l.y);
            max.x = isize::max(max.x, l.x);
            max.y = isize::max(max.y, l.y);
        }
        (min, max)
    }

    fn intersections(&self) -> Vec<IPoint2> {
        self.tiles
            .iter()
            .filter_map(|(loc, tile)| {
                if tile == &Tile::Scaffold
                    && Dir::variants()
                        .into_iter()
                        .filter(|dir| self.tiles.get(&(*loc + dir.unit())) == Some(&Tile::Scaffold))
                        .count()
                        > 2
                {
                    Some(*loc)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self.bounds();
        write!(
            f,
            "{}",
            (min.y..=max.y)
                .map(|y| (min.x..=max.x)
                    .map(|x| {
                        let loc = IPoint2 { x, y };
                        if loc == self.at {
                            self.dir.to_string()
                        } else {
                            self.tiles
                                .get(&loc)
                                .cloned()
                                .unwrap_or(Tile::Space)
                                .to_string()
                        }
                    })
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn main() -> Result<()> {
    let mut raw_input = String::new();
    stdin().lock().read_to_string(&mut raw_input)?;
    let input: Vec<isize> = raw_input
        .trim()
        .split(',')
        .map(|w| w.parse::<isize>().context("failed parsing number"))
        .collect::<Result<Vec<isize>>>()?;
    let mut prog = Program::with_memory(input);
    let map: String = run(&mut prog, &[])?
        .output
        .into_iter()
        .map(|c| (c as u8) as char)
        .collect();
    let world = World::parse(&map)?;
    println!(
        "{}",
        world
            .intersections()
            .into_iter()
            .map(|loc| loc.x * loc.y)
            .sum::<isize>()
    );
    Ok(())
}
