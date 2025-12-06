use std::fs;

fn main() {
    let mut current_dial: u32 = 50;
    let mut zeros_counter: u32 = 0;

    let inputs = read_inputs("res/day_01_input.txt");

    for input in inputs {
        current_dial = input.apply_to(current_dial);
        if current_dial == 0 {
            zeros_counter += 1;
        }
    }

    println!("Number of zeros: {zeros_counter}.");
}

enum Input {
    R(u32),
    L(u32),
}

impl Input {
    fn apply_to(&self, current_dial: u32) -> u32 {
        match self {
            Input::R(i) => (current_dial + i) % 100,
            Input::L(i) => {
                let i = *i % 100;
                if current_dial >= i {
                    current_dial - i
                } else {
                    100 - (i - current_dial)
                }
            }
        }
    }
}

fn read_inputs(path: &str) -> Vec<Input> {
    let input_file_content = fs::read_to_string(path).expect("Should be able to read file input.");
    let input_lines = input_file_content.trim().lines();

    let mut inputs: Vec<Input> = Vec::new();

    for input_line in input_lines {
        let mut input_chars = input_line.chars();
        match input_chars.next() {
            Some('L') => {
                let shift: Vec<char> = input_chars.collect();
                let shift = String::from_iter(shift).parse::<u32>().unwrap();
                inputs.push(Input::L(shift));
            }
            Some('R') => {
                let shift: Vec<char> = input_chars.collect();
                let shift = String::from_iter(shift).parse::<u32>().unwrap();
                inputs.push(Input::R(shift));
            }
            Some(c) => panic!("Invalid starting character `{c}` for line `{input_line}`."),
            None => panic!("Empty line."),
        }
    }
    inputs
}
