use anyhow::{anyhow, Context, Error, Result};

use helper::point::IPoint2;
use intcptr::{run, Program};

use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn variants() -> Vec<Self> {
        vec![Dir::North, Dir::South, Dir::East, Dir::West]
    }
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

impl From<IPoint2> for Dir {
    fn from(p: IPoint2) -> Self {
        if p.x.abs() > p.y.abs() {
            if p.x > 0 {
                Dir::East
            } else {
                Dir::West
            }
        } else {
            if p.y > 0 {
                Dir::South
            } else {
                Dir::North
            }
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

impl Tile {
    fn is_blocking(&self) -> bool {
        match *self {
            Tile::Empty | Tile::OxygenSystem => false,
            Tile::Wall => true,
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct World {
    tiles: HashMap<IPoint2, Tile>,
    at: IPoint2,
}

impl Default for World {
    fn default() -> Self {
        let origin = IPoint2::default();
        World {
            tiles: vec![(origin, Tile::Empty)].into_iter().collect(),
            at: origin,
        }
    }
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

    fn unexplored(&self) -> Vec<IPoint2> {
        self.tiles
            .iter()
            .filter_map(|(loc, tile)| match tile {
                Tile::Empty => Some(Dir::variants().into_iter().filter_map(move |dir| {
                    let neighbour = *loc + dir.into();
                    if !self.tiles.contains_key(&neighbour) {
                        Some(neighbour)
                    } else {
                        None
                    }
                })),
                _ => None,
            })
            .flatten()
            .collect()
    }

    fn explore(&mut self, prog: &mut Program) -> Result<()> {
        let mut unexplored: HashSet<IPoint2> = self.unexplored().into_iter().collect();
        while !unexplored.is_empty() {
            // Find the closest unexplored tile to the current position
            let next = *unexplored
                .iter()
                .min_by_key(|x| (self.at - **x).manhattan())
                .unwrap();
            unexplored.remove(&next);
            self.move_to(next, prog)?;
            for dir in Dir::variants() {
                let neighbour = self.at + dir.into();
                if !self.tiles.contains_key(&neighbour) {
                    unexplored.insert(neighbour);
                }
            }
        }
        Ok(())
    }

    fn find_path(&self, from: IPoint2, to: IPoint2) -> Option<Vec<IPoint2>> {
        if from == to {
            return Some(vec![from]);
        }
        let from_state = PathState {
            at: from,
            dist: 0,
            prev: None,
        };
        // Simple implementation of Dijkstra's algo for an unweighted ortho grid
        let mut dists: HashMap<IPoint2, PathState> = vec![(from, from_state)].into_iter().collect();
        // Use Reverse to create a min-heap instead of a max-heap
        let mut priority_queue: BinaryHeap<Reverse<PathState>> =
            vec![Reverse(from_state)].into_iter().collect();

        while let Some(Reverse(PathState { at, dist, prev })) = priority_queue.pop() {
            if at == to {
                // Path found
                let mut path = vec![at];
                let mut path_prev = prev;
                while let Some(p) = path_prev {
                    path.push(p);
                    path_prev = dists.get(&p).cloned().and_then(|v| v.prev);
                }
                path.reverse();
                return Some(path);
            }
            // Add the empty unvisited neighbours
            for dir in Dir::variants() {
                let neighbour_loc = at + dir.into();
                if !dists.contains_key(&neighbour_loc)
                    && (neighbour_loc == to
                        || self
                            .tiles
                            .get(&neighbour_loc)
                            .map(|t| !t.is_blocking())
                            .unwrap_or(false))
                {
                    let next = PathState {
                        at: neighbour_loc,
                        dist: dist + 1,
                        prev: Some(at),
                    };
                    dists.insert(neighbour_loc, next);
                    priority_queue.push(Reverse(next));
                }
            }
        }
        None
    }

    fn move_to(&mut self, to: IPoint2, mut prog: &mut Program) -> Result<usize> {
        let path = match self.find_path(self.at, to) {
            Some(p) => p,
            None => return Err(anyhow!("could not find path")),
        };
        for move_dir in points_to_dirs(&path) {
            let target_loc = self.at + move_dir.into();
            match run(&mut prog, &[move_dir.into()])?
                .output
                .into_iter()
                .next()
                .map(DroneStatus::try_from)
            {
                Some(Ok(DroneStatus::HitWall)) => {
                    self.tiles.insert(target_loc, Tile::Wall);
                }
                Some(Ok(DroneStatus::Moved)) => {
                    self.tiles.insert(target_loc, Tile::Empty);
                    self.at = target_loc;
                }
                Some(Ok(DroneStatus::FoundOxygenSystem)) => {
                    self.tiles.insert(target_loc, Tile::OxygenSystem);
                    self.at = target_loc;
                }
                Some(Err(e)) => return Err(e).context("invalid output from intcptr"),
                None => {}
            };
        }
        Ok(path.len())
    }
}

fn points_to_dirs(points: &[IPoint2]) -> Vec<Dir> {
    (1..points.len())
        .map(|i| Dir::from(points[i] - points[i - 1]))
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PathState {
    at: IPoint2,
    dist: usize,
    prev: Option<IPoint2>,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

    let mut prog = Program::with_memory(input);
    let mut world = World::default();
    world.explore(&mut prog)?;
    println!("{}", world);
    let oxy_loc = *world
        .tiles
        .iter()
        .find(|&(_loc, t)| *t == Tile::OxygenSystem)
        .ok_or_else(|| anyhow!("could not find oxygen system"))?
        .0;
    println!(
        "{}",
        world.find_path(IPoint2::default(), oxy_loc).unwrap().len() - 1
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_from_ipoint2() {
        for dir in Dir::variants() {
            let p: IPoint2 = dir.into();
            assert_eq!(Dir::from(p), dir);
        }
    }
}
