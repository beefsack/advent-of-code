const START: usize = 168630;
const END: usize = 718098;

fn main() {
    let mut valid: usize = 0;
    for i in START..=END {
        if is_valid(i) {
            valid += 1
        }
    }
    println!("{}", valid);
}

fn is_valid(pass: usize) -> bool {
    if pass < 100_000 || pass > 999_999 {
        // 6 digits only
        return false;
    }
    let mut last: Option<char> = None;
    let mut has_consecutive = false;
    for c in pass.to_string().chars() {
        if let Some(last_c) = last {
            if c < last_c {
                // No decreasing
                return false;
            }
            if c == last_c {
                has_consecutive = true;
            }
        }
        last = Some(c);
    }
    has_consecutive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(111111), true);
        assert_eq!(is_valid(223450), false);
        assert_eq!(is_valid(123789), false);
    }
}
