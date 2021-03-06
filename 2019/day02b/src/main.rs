use std::io::{stdin, Read};

const OPCODE_ADD: usize = 1;
const OPCODE_MUL: usize = 2;
const OPCODE_EXIT: usize = 99;

fn main() {
    let mut raw_input = String::new();
    stdin().lock().read_to_string(&mut raw_input).unwrap();
    let mut input: Vec<usize> = raw_input
        .trim()
        .split(',')
        .map(|w| w.parse::<usize>().unwrap())
        .collect();
    let original_input = input.clone();
    // Override values to reset program
    for x in 0..=99 {
        for y in 0..=99 {
            input = original_input.clone();
            input[1] = x;
            input[2] = y;
            run(&mut input);
            if input[0] == 19_690_720 {
                println!("{}", 100 * x + y);
                return;
            }
        }
    }
    panic!("inputs not found");
}

fn run(input: &mut [usize]) {
    let mut pos: usize = 0;
    loop {
        match input[pos] {
            OPCODE_ADD => {
                input[input[pos + 3]] = input[input[pos + 1]] + input[input[pos + 2]];
                pos += 4;
            }
            OPCODE_MUL => {
                input[input[pos + 3]] = input[input[pos + 1]] * input[input[pos + 2]];
                pos += 4;
            }
            OPCODE_EXIT => return,
            invalid => panic!("invalid opcode: {}", invalid),
        }
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
