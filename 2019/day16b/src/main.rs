use anyhow::Result;

use std::io::{stdin, Read};

fn sum_of_following(input: &mut Vec<i8>) {
    let mut tally: isize = 0;
    input.iter_mut().rev().for_each(|v| {
        tally += *v as isize;
        *v = (tally % 10) as i8;
    })
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;
    let trimmed = input.trim();
    let offset: usize = trimmed[0..7].parse()?;
    let mut ints: Vec<i8> = trimmed
        .bytes()
        .map(|b| (b - b'0') as i8)
        .cycle()
        .take(10_000 * trimmed.len())
        .collect();

    // We now work on the assumption that the offset it in the second half of output, and the second
    // half of the pattern is just the sum of itself plus the following.
    for _ in 0..100 {
        sum_of_following(&mut ints);
    }
    println!(
        "{}",
        &ints[offset..offset + 8]
            .iter()
            .map(|v| format!("{}", v))
            .collect::<String>()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_following() {
        let mut ints: Vec<i8> = vec![0, 1, 5, 9];
        sum_of_following(&mut ints);
        assert_eq!(ints, vec![5, 5, 4, 9]);
    }
}
