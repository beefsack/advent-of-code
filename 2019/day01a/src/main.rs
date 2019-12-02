use anyhow::Result;
use std::io::{self, BufRead};

fn main() {
    print!(
        "{}",
        read_usizes()
            .unwrap()
            .into_iter()
            .map(required_fuel)
            .sum::<usize>()
    );
}

fn read_usizes() -> Result<Vec<usize>> {
    io::stdin().lock().lines().map(|l| parse_line(l?)).collect()
}

fn parse_line(line: String) -> Result<usize> {
    Ok(line.parse()?)
}

fn required_fuel(mass: usize) -> usize {
    if mass > 2 {
        mass / 3 - 2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_fuel() {
        assert_eq!(required_fuel(12), 2);
        assert_eq!(required_fuel(14), 2);
        assert_eq!(required_fuel(1969), 654);
        assert_eq!(required_fuel(100756), 33583);
    }
}
