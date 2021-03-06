use helper;
use std::io::stdin;

fn main() {
    print!(
        "{}",
        helper::parse_lines::<usize, _>(stdin())
            .map(|l| required_fuel(l.unwrap()))
            .sum::<usize>()
    );
}

fn required_fuel(mass: usize) -> usize {
    if mass >= 6 {
        let fuel = mass / 3 - 2;
        fuel + required_fuel(fuel)
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
        assert_eq!(required_fuel(1969), 966);
        assert_eq!(required_fuel(100756), 50346);
    }
}
