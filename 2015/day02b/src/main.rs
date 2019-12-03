use anyhow::Result;
use std::io::{stdin, BufRead};

fn main() {
    println!(
        "{}",
        stdin()
            .lock()
            .lines()
            .map(|l| parse_box(l.unwrap()).unwrap())
            .map(|b| ribbon(&b))
            .sum::<usize>(),
    );
}

fn parse_box(input: String) -> Result<Vec<usize>> {
    input.split('x').map(|d| Ok(d.parse::<usize>()?)).collect()
}

fn smallest_permimeter(b: &[usize]) -> usize {
    let mut b = b.to_vec();
    b.sort();
    b.iter().take(2).sum::<usize>() * 2
}

fn volume(b: &[usize]) -> usize {
    b.iter().product()
}

fn ribbon(b: &[usize]) -> usize {
    smallest_permimeter(b) + volume(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ribbon() {
        assert_eq!(ribbon(&[2, 3, 4]), 34);
        assert_eq!(ribbon(&[1, 1, 10]), 14);
    }
}
