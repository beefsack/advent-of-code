use anyhow::{anyhow, Context, Error, Result};

use helper::point::IPoint2;
use intcptr::{run, Program};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::io::{stdin, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Into<IPoint2> for Dir {
    fn into(self) -> IPoint2 {
        match self {
            Dir::North => IPoint2 { x: 0, y: -1 },
            Dir::South => IPoint2 { x: 0, y: 1 },
            Dir::East => IPoint2 { x: 1, y: 0 },
            Dir::West => IPoint2 { x: -1, y: 0 },
        }
    }
}

impl Into<isize> for Dir {
    fn into(self) -> isize {
        match self {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DroneStatus {
    HitWall,
    Moved,
    FoundOxygenSystem,
}

impl TryFrom<isize> for DroneStatus {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self> {
        match value {
            0 => Ok(DroneStatus::HitWall),
            1 => Ok(DroneStatus::Moved),
            2 => Ok(DroneStatus::FoundOxygenSystem),
            _ => Err(anyhow!("invalid status {}", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    OxygenSystem,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::OxygenSystem => 'O',
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct World {
    tiles: HashMap<IPoint2, Tile>,
    at: IPoint2,
}

impl World {
    fn bounds(&self) -> (IPoint2, IPoint2) {
        let mut min = IPoint2 { x: 0, y: 0 };
        let mut max = min;
        for p in self.tiles.keys() {
            min.x = isize::min(p.x, min.x);
            min.y = isize::min(p.y, min.y);
            max.x = isize::max(p.x, max.x);
            max.y = isize::max(p.y, max.y);
        }
        (min, max)
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self.bounds();
        write!(
            f,
            "{}",
            (min.y..=max.y)
                .map(|y| {
                    (min.x..=max.x)
                        .map(|x| {
                            let p = IPoint2 { x, y };
                            if p == self.at {
                                "D".to_string()
                            } else {
                                match self.tiles.get(&p) {
                                    Some(t) => t.to_string(),
                                    None => " ".to_string(),
                                }
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn main() -> Result<()> {
    let input: Vec<isize> = include_str!("../res/input")
        .trim()
        .split(',')
        .map(|w| w.parse::<isize>().context("failed parsing number"))
        .collect::<Result<Vec<isize>>>()?;

    let mut world = World::default();
    let mut prog = Program::with_memory(input);
    let mut chars = stdin().bytes();

    loop {
        println!("{}", world);
        let move_dir = match chars.next() {
            Some(Ok(b'w')) | Some(Ok(b'W')) => Dir::North,
            Some(Ok(b's')) | Some(Ok(b'S')) => Dir::South,
            Some(Ok(b'd')) | Some(Ok(b'D')) => Dir::East,
            Some(Ok(b'a')) | Some(Ok(b'A')) => Dir::West,
            Some(Err(e)) => return Err(e).context("error reading char from input"),
            None | Some(Ok(b'q')) | Some(Ok(b'Q')) => break,
            Some(Ok(_)) => {
                println!("invalid character, use WASD or Q to exit");
                continue;
            }
        };
        let target_loc = world.at + move_dir.into();
        match run(&mut prog, &[move_dir.into()])?
            .output
            .into_iter()
            .next()
            .map(DroneStatus::try_from)
        {
            Some(Ok(DroneStatus::HitWall)) => {
                world.tiles.insert(target_loc, Tile::Wall);
            }
            Some(Ok(DroneStatus::Moved)) => {
                world.tiles.insert(target_loc, Tile::Empty);
                world.at = target_loc;
            }
            Some(Ok(DroneStatus::FoundOxygenSystem)) => {
                world.tiles.insert(target_loc, Tile::OxygenSystem);
                world.at = target_loc;
            }
            Some(Err(e)) => return Err(e).context("invalid output from intcptr"),
            None => {}
        };
    }

    Ok(())
}
