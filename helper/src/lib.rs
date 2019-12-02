use anyhow::Result;
use std::io::{self, BufRead};

pub fn read_usizes() -> Result<Vec<usize>> {
    io::stdin().lock().lines().map(|l| parse_line(l?)).collect()
}

fn parse_line(line: String) -> Result<usize> {
    Ok(line.parse()?)
}
