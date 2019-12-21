use anyhow::{Context, Result};

use std::io::{stdin, Read};

use intcptr::{run, Program};

fn main() -> Result<()> {
    let mut raw_input = String::new();
    stdin().lock().read_to_string(&mut raw_input)?;
    let input: Vec<isize> = raw_input
        .trim()
        .split(',')
        .map(|w| w.parse::<isize>().context("failed parsing number"))
        .collect::<Result<Vec<isize>>>()?;
    let mut prog = Program::with_memory(input);
    println!("{}", run(&mut prog, &[2])?.output[0]);
    Ok(())
}
