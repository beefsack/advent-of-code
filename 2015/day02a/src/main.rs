use anyhow::Result;
use std::io::{stdin, BufRead};

fn main() {
    println!(
        "{}",
        stdin()
            .lock()
            .lines()
            .map(|l| parse_box(l.unwrap()).unwrap())
            .map(|b| wrapping_paper(&b))
            .sum::<usize>(),
    );
}

fn parse_box(input: String) -> Result<Vec<usize>> {
    input.split('x').map(|d| Ok(d.parse::<usize>()?)).collect()
}

fn area(b: &[usize]) -> usize {
    let mut area: usize = 0;
    for (index, x) in b.iter().take(b.len() - 1).enumerate() {
        for y in b.iter().skip(index + 1) {
            area += x * y;
        }
    }
    area * 2
}

fn smallest_side(b: &[usize]) -> usize {
    let mut b = b.to_vec();
    b.sort();
    b.iter().take(2).product()
}

fn wrapping_paper(b: &[usize]) -> usize {
    area(b) + smallest_side(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping_paper() {
        assert_eq!(wrapping_paper(&[2, 3, 4]), 58);
        assert_eq!(wrapping_paper(&[1, 1, 10]), 43);
    }

    #[test]
    fn test_smallest_side() {
        assert_eq!(smallest_side(&[5, 4, 1, 2]), 2);
    }
}
