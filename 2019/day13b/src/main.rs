use anyhow::{anyhow, Context, Result};

use std::cmp::Ordering;
use std::collections::HashMap;
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
    // Free play, no quarters required
    prog.set(0, 2);

    let mut joystick_next: Option<isize> = None;
    let mut screen: HashMap<(isize, isize), Tile> = HashMap::new();
    let mut score = 0;

    loop {
        let input = match joystick_next {
            Some(j) => vec![j],
            None => vec![],
        };
        let output = run(&mut prog, &input)?.output;

        for (index, x) in output.iter().enumerate().step_by(3) {
            if output.len() <= index + 2 {
                break;
            }
            let y = output[index + 1];
            if *x == -1 && y == 0 {
                // Score
                score = output[index + 2];
            } else {
                // Tile
                screen.insert((*x, y), Tile::parse(output[index + 2])?);
            }
        }

        let mut block_count = 0;
        let mut ball_x = 0;
        let mut paddle_x = 0;

        for ((x, _y), tile) in &screen {
            match tile {
                Tile::Block => block_count += 1,
                Tile::Ball => ball_x = *x,
                Tile::Paddle => paddle_x = *x,
                _ => {}
            }
        }

        if block_count == 0 {
            println!("Game over");
            // Game over!
            break;
        }

        match paddle_x.cmp(&ball_x) {
            Ordering::Greater => joystick_next = Some(-1),
            Ordering::Less => joystick_next = Some(1),
            Ordering::Equal => joystick_next = Some(0),
        }
    }
    println!("{}", score);

    Ok(())
}
