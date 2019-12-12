use anyhow::{anyhow, Context, Error, Result};

use intcptr;

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;
use std::io::{stdin, Read};
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Colour {
    Black,
    White,
}

impl Default for Colour {
    fn default() -> Self {
        Colour::Black
    }
}

impl Into<isize> for Colour {
    fn into(self) -> isize {
        match self {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }
}

impl TryFrom<isize> for Colour {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self> {
        match value {
            0 => Ok(Colour::Black),
            1 => Ok(Colour::White),
            _ => Err(anyhow!("{} is not a valid colour", value)),
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Colour::Black => " ",
                Colour::White => "#",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn unit_vector(&self) -> Point {
        match *self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
        }
    }
}

fn paint(mut prog: &mut intcptr::Program) -> Result<()> {
    let mut ship: HashMap<Point, Colour> = HashMap::new();
    ship.insert(Point { x: 0, y: 0 }, Colour::White);
    let mut painted: HashSet<Point> = HashSet::new();
    let mut loc = Point { x: 0, y: 0 };
    let mut dir: Direction = Direction::Up;

    loop {
        let cur_colour = ship.entry(loc).or_default();
        let halt = intcptr::run(&mut prog, &[(*cur_colour).into()])?;
        if halt.output.len() != 2 {
            return Err(anyhow!("Expected output len 2, got {}", halt.output.len()));
        }
        let colour = Colour::try_from(halt.output[0])?;
        if colour != *cur_colour {
            *cur_colour = colour;
            painted.insert(loc);
        }
        match halt.output[1] {
            0 => dir = dir.turn_left(),
            1 => dir = dir.turn_right(),
            n => return Err(anyhow!("Expected direction 0 or 1, got {}", n)),
        }
        loc = loc + dir.unit_vector();
        if halt.cause == intcptr::HaltCause::Exit {
            break;
        }
    }

    println!("{}", render(&ship));
    Ok(())
}

fn render(ship: &HashMap<Point, Colour>) -> String {
    let mut min_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;

    for loc in ship.keys() {
        if loc.x < min_x {
            min_x = loc.x;
        }
        if loc.x > max_x {
            max_x = loc.x;
        }
        if loc.y < min_y {
            min_y = loc.y;
        }
        if loc.y > max_y {
            max_y = loc.y;
        }
    }

    (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .map(|x| format!("{}", ship.get(&Point { x, y }).cloned().unwrap_or_default()))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() -> Result<()> {
    let mut raw_input = String::new();
    stdin().lock().read_to_string(&mut raw_input)?;
    let input: Vec<isize> = raw_input
        .trim()
        .split(',')
        .map(|w| w.parse::<isize>().context("failed parsing number"))
        .collect::<Result<Vec<isize>>>()?;
    let mut prog = intcptr::Program::with_memory(input);
    paint(&mut prog)?;
    Ok(())
}
