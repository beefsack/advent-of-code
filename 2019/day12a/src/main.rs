use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

use helper::point::IPoint3;

use std::cmp::Ordering;
use std::io::{stdin, BufRead};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Body {
    pos: IPoint3,
    vel: IPoint3,
}

impl Body {
    fn with_pos(pos: IPoint3) -> Self {
        Self {
            pos,
            vel: Default::default(),
        }
    }

    fn potential_energy(&self) -> isize {
        self.pos.to_vec().iter().map(|p| p.abs()).sum()
    }

    fn kinetic_energy(&self) -> isize {
        self.vel.to_vec().iter().map(|p| p.abs()).sum()
    }

    fn total_energy(&self) -> isize {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct System(Vec<Body>);

impl System {
    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn apply_gravity(&mut self) {
        for a_index in 0..self.0.len() - 1 {
            for b_index in a_index + 1..self.0.len() {
                let (new_a, new_b) = gravity(self.0[a_index], self.0[b_index]);
                self.0[a_index] = new_a;
                self.0[b_index] = new_b;
            }
        }
    }

    fn apply_velocity(&mut self) {
        self.0.iter_mut().for_each(|body| body.pos += body.vel);
    }

    fn total_energy(&self) -> isize {
        self.0.iter().map(|b| b.total_energy()).sum()
    }
}

fn parse_ipoint3(input: &str) -> Result<IPoint3> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<x=(-?\d+),y=(-?\d+),z=(-?\d+)>").unwrap();
    }
    let trimmed: String = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let capture = RE
        .captures_iter(&trimmed)
        .next()
        .ok_or_else(|| anyhow!("could not find point"))?;
    Ok(IPoint3 {
        x: capture[1].parse()?,
        y: capture[2].parse()?,
        z: capture[3].parse()?,
    })
}

fn gravity_change_1d(a: isize, b: isize) -> isize {
    match a.cmp(&b) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

fn gravity_change_3d(a: IPoint3, b: IPoint3) -> IPoint3 {
    IPoint3 {
        x: gravity_change_1d(a.x, b.x),
        y: gravity_change_1d(a.y, b.y),
        z: gravity_change_1d(a.z, b.z),
    }
}

fn gravity(mut a: Body, mut b: Body) -> (Body, Body) {
    let change = gravity_change_3d(a.pos, b.pos);
    a.vel += change;
    b.vel -= change;
    (a, b)
}

fn main() -> Result<()> {
    let mut system: System = System(
        stdin()
            .lock()
            .lines()
            .map(|l| Ok(Body::with_pos(parse_ipoint3(&l?)?)))
            .collect::<Result<Vec<Body>>>()?,
    );
    for _ in 0..1000 {
        system.step();
    }
    println!("{}", system.total_energy());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_ipoint3() -> Result<()> {
        assert_eq!(
            parse_ipoint3("<x=-2, y=0,z=3>")?,
            IPoint3 { x: -2, y: 0, z: 3 }
        );
        assert_eq!(
            parse_ipoint3("<x=3, y=5, z=-1>")?,
            IPoint3 { x: 3, y: 5, z: -1 }
        );
        assert_eq!(
            parse_ipoint3("<x= -8, y=-10, z=  0>")?,
            IPoint3 {
                x: -8,
                y: -10,
                z: 0
            }
        );
        Ok(())
    }

    #[test]
    fn test_gravity_change_1d() {
        assert_eq!(gravity_change_1d(1, 3), 1);
        assert_eq!(gravity_change_1d(3, 1), -1);
        assert_eq!(gravity_change_1d(1, 1), 0);
    }

    #[test]
    fn test_system_next_n2() {
        let mut system = System(vec![
            Body::with_pos(IPoint3 { x: -1, y: 0, z: 2 }),
            Body::with_pos(IPoint3 { x: 2, y: -10, z: 2 }),
        ]);

        system.step();
        assert_eq!(
            system.0,
            vec![
                Body {
                    pos: IPoint3 { x: 0, y: -1, z: 2 },
                    vel: IPoint3 { x: 1, y: -1, z: 0 },
                },
                Body {
                    pos: IPoint3 { x: 1, y: -9, z: 2 },
                    vel: IPoint3 { x: -1, y: 1, z: 0 },
                },
            ]
        );

        system.step();
        assert_eq!(
            system.0,
            vec![
                Body {
                    pos: IPoint3 { x: 2, y: -3, z: 2 },
                    vel: IPoint3 { x: 2, y: -2, z: 0 },
                },
                Body {
                    pos: IPoint3 { x: -1, y: -7, z: 2 },
                    vel: IPoint3 { x: -2, y: 2, z: 0 },
                },
            ]
        );
    }

    #[test]
    fn test_system_next_n4() {
        let mut system = System(vec![
            Body::with_pos(IPoint3 { x: -1, y: 0, z: 2 }),
            Body::with_pos(IPoint3 {
                x: 2,
                y: -10,
                z: -7,
            }),
            Body::with_pos(IPoint3 { x: 4, y: -8, z: 8 }),
            Body::with_pos(IPoint3 { x: 3, y: 5, z: -1 }),
        ]);

        system.step();
        assert_eq!(
            system.0,
            vec![
                Body {
                    pos: IPoint3 { x: 2, y: -1, z: 1 },
                    vel: IPoint3 { x: 3, y: -1, z: -1 },
                },
                Body {
                    pos: IPoint3 { x: 3, y: -7, z: -4 },
                    vel: IPoint3 { x: 1, y: 3, z: 3 },
                },
                Body {
                    pos: IPoint3 { x: 1, y: -7, z: 5 },
                    vel: IPoint3 { x: -3, y: 1, z: -3 },
                },
                Body {
                    pos: IPoint3 { x: 2, y: 2, z: 0 },
                    vel: IPoint3 { x: -1, y: -3, z: 1 },
                },
            ]
        );
    }
}
