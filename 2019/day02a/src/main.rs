use std::io::{stdin, Read};

const OPCODE_ADD: usize = 1;
const OPCODE_MUL: usize = 2;
const OPCODE_EXIT: usize = 99;
const STEP: usize = 4;

fn main() {
    let mut raw_input = String::new();
    stdin().lock().read_to_string(&mut raw_input).unwrap();
    let mut input: Vec<usize> = raw_input
        .trim()
        .split(',')
        .map(|w| w.parse::<usize>().unwrap())
        .collect();
    // Override values to reset program
    input[1] = 12;
    input[2] = 2;
    run(&mut input);
    println!("{}", input[0]);
}

fn run(input: &mut [usize]) {
    let mut pos: usize = 0;
    loop {
        match input[pos] {
            OPCODE_ADD => input[input[pos + 3]] = input[input[pos + 1]] + input[input[pos + 2]],
            OPCODE_MUL => input[input[pos + 3]] = input[input[pos + 1]] * input[input[pos + 2]],
            OPCODE_EXIT => return,
            invalid => panic!("invalid opcode: {}", invalid),
        }
        pos += STEP;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut input: Vec<usize> = vec![1, 0, 0, 0, 99];
        run(&mut input);
        assert_eq!(input, vec![2, 0, 0, 0, 99]);

        input = vec![2, 3, 0, 3, 99];
        run(&mut input);
        assert_eq!(input, vec![2, 3, 0, 6, 99]);

        input = vec![2, 4, 4, 5, 99, 0];
        run(&mut input);
        assert_eq!(input, vec![2, 4, 4, 5, 99, 9801]);

        input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut input);
        assert_eq!(input, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
