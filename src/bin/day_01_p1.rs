use std::fs;

fn main() {
    let mut current_dial: u32 = 50;
    let mut zeros_counter: u32 = 0;

    let inputs = read_inputs("res/day_01_input.txt");

    for input in &inputs {
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
            Input::R(n) => (current_dial + n) % 100,
            Input::L(n) => {
                let n = *n % 100;
                if current_dial >= n {
                    current_dial - n
                } else {
                    100 - (n - current_dial)
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
        let (direction, shift) = input_line.split_at(1);
        let shift: u32 = shift
            .parse()
            .expect("Invalid shift `{shift}`: can't convert it to u32.");

        match direction {
            "L" => inputs.push(Input::L(shift)),
            "R" => inputs.push(Input::R(shift)),
            c => panic!("Invalid starting character `{c}` for line `{input_line}`."),
        }
    }

    inputs
}
