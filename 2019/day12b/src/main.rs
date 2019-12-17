use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;

use helper::point::ipoint3::Field;
use helper::point::IPoint3;

use std::collections::HashMap;
use std::hash::Hash;
use std::io::{stdin, BufRead};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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

    fn axis(&self, axis: Field) -> Body1 {
        Body1 {
            pos: self.pos[axis],
            vel: self.vel[axis],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct System(Vec<Body>);

impl System {
    fn loop_steps(&self) -> usize {
        let (x_start, x_len) = self.axis(Field::X).loop_steps();
        let (y_start, y_len) = self.axis(Field::Y).loop_steps();
        let (z_start, z_len) = self.axis(Field::Z).loop_steps();

        if x_start > 0 || y_start > 0 || z_start > 0 {
            unimplemented!("loop_steps not implemented for loop start > 0");
        }

        // Have only implemented for all starts = 0, as that produces the correct answer
        lcm(lcm(x_len, y_len), lcm(y_len, z_len))
    }

    fn axis(&self, axis: Field) -> System1 {
        System1(self.0.iter().map(|b| b.axis(axis)).collect())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Body1 {
    pos: isize,
    vel: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct System1(Vec<Body1>);

impl System1 {
    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn apply_gravity(&mut self) {
        for a_index in 0..self.0.len() - 1 {
            for b_index in a_index + 1..self.0.len() {
                let (new_a, new_b) = gravity1(self.0[a_index], self.0[b_index]);
                self.0[a_index] = new_a;
                self.0[b_index] = new_b;
            }
        }
    }

    fn apply_velocity(&mut self) {
        self.0.iter_mut().for_each(|body| body.pos += body.vel);
    }

    fn loop_steps(&self) -> (usize, usize) {
        let mut iter = self.clone();
        let mut previous: HashMap<System1, usize> = HashMap::new();
        let mut i = 0;
        previous.insert(iter.clone(), i);
        loop {
            i += 1;
            iter.step();
            if let Some(pos) = previous.get(&iter) {
                return (*pos, i - pos);
            }
            previous.insert(iter.clone(), i);
        }
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

fn gravity_change1(a: isize, b: isize) -> isize {
    if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    }
}

fn gravity1(mut a: Body1, mut b: Body1) -> (Body1, Body1) {
    let change = gravity_change1(a.pos, b.pos);
    a.vel += change;
    b.vel -= change;
    (a, b)
}

fn main() -> Result<()> {
    let system: System = System(
        stdin()
            .lock()
            .lines()
            .map(|l| Ok(Body::with_pos(parse_ipoint3(&l?)?)))
            .collect::<Result<Vec<Body>>>()?,
    );
    println!("{}", system.loop_steps());
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
        assert_eq!(gravity_change1(1, 3), 1);
        assert_eq!(gravity_change1(3, 1), -1);
        assert_eq!(gravity_change1(1, 1), 0);
    }

    #[test]
    fn test_system_loop_steps() {
        let system = System(vec![
            Body::with_pos(IPoint3 { x: -1, y: 0, z: 2 }),
            Body::with_pos(IPoint3 {
                x: 2,
                y: -10,
                z: -7,
            }),
            Body::with_pos(IPoint3 { x: 4, y: -8, z: 8 }),
            Body::with_pos(IPoint3 { x: 3, y: 5, z: -1 }),
        ]);

        assert_eq!(system.loop_steps(), 2772);
    }
}
