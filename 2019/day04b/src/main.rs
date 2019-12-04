const START: usize = 168_630;
const END: usize = 718_098;

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
    let mut consecutives: Vec<usize> = vec![];
    let mut cur_consecutive: usize = 0;
    for c in pass.to_string().chars() {
        if let Some(last_c) = last {
            if c < last_c {
                // No decreasing
                return false;
            }
            if c == last_c {
                cur_consecutive += 1;
            } else {
                consecutives.push(cur_consecutive);
                cur_consecutive = 1;
            }
        } else {
            cur_consecutive += 1;
        }
        last = Some(c);
    }
    consecutives.push(cur_consecutive);
    consecutives.iter().any(|&c| c == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(112233), true);
        assert_eq!(is_valid(123444), false);
        assert_eq!(is_valid(111122), true);
    }
}
