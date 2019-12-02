use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut level: isize = 0;
    for (index, c) in input.chars().enumerate() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            x => panic!("unknown character: {}", x),
        }
        if level == -1 {
            println!("{}", index + 1);
            return;
        }
    }
    panic!("didn't find basement");
}
