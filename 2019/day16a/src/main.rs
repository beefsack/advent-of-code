use anyhow::{anyhow, Result};

use std::io::{stdin, Read};
use std::iter;

const BASE_PATTERN: &[isize] = &[0, 1, 0, -1];

struct Pattern(Vec<isize>);

impl Pattern {
    fn nth(&self, n: usize) -> Vec<isize> {
        if n == 0 {
            panic!("n must be non-zero");
        }
        self.0
            .iter()
            .map(|v| iter::repeat(*v).take(n).collect::<Vec<isize>>())
            .flatten()
            .collect()
    }

    fn apply(&self, input: Vec<isize>) -> Vec<isize> {
        let mut output = vec![0isize; input.len()];
        for (i, output_item) in output.iter_mut().enumerate().take(input.len()) {
            let p = self.nth(i + 1);
            *output_item = input
                .iter()
                .enumerate()
                .map(|(k, v)| v * p[(k + 1) % p.len()])
                .sum::<isize>()
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as isize;
        }
        output
    }

    fn apply_n(&self, mut input: Vec<isize>, n: usize) -> Vec<isize> {
        for _ in 0..n {
            input = self.apply(input);
        }
        input
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;
    let ints: Vec<isize> = input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as isize)
                .ok_or_else(|| anyhow!("not a digit"))
        })
        .collect::<Result<Vec<isize>>>()?;
    let pattern = Pattern(BASE_PATTERN.to_vec());
    println!(
        "{}",
        pattern
            .apply_n(ints, 100)
            .into_iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_nth() {
        let pattern = Pattern(BASE_PATTERN.to_vec());
        assert_eq!(pattern.nth(1), vec![0, 1, 0, -1]);
        assert_eq!(pattern.nth(2), vec![0, 0, 1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn test_pattern_apply() {
        let input: Vec<isize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let pattern = Pattern(BASE_PATTERN.to_vec());
        assert_eq!(
            pattern.apply_n(input.clone(), 1),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            pattern.apply_n(input.clone(), 2),
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        );
        assert_eq!(
            pattern.apply_n(input.clone(), 3),
            vec![0, 3, 4, 1, 5, 5, 1, 8]
        );
        assert_eq!(
            pattern.apply_n(input.clone(), 4),
            vec![0, 1, 0, 2, 9, 4, 9, 8]
        );
    }
}
