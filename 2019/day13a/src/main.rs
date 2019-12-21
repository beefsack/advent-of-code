use anyhow::{anyhow, Context, Result};

use std::io::{stdin, Read};

use intcptr::{run, Program};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn parse(input: isize) -> Result<Self> {
        match input {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err(anyhow!("invalid tile {}", input)),
        }
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
    let output = run(&mut prog, &[2])?.output;

    let mut block_count = 0;
    for (index, _x) in output.iter().enumerate().step_by(3) {
        if output.len() <= index + 2 {
            break;
        }
        let _y = output[index + 1];
        let tile = Tile::parse(output[index + 2])?;
        if tile == Tile::Block {
            block_count += 1;
        }
    }
    println!("{}", block_count);

    Ok(())
}
