use std::str::FromStr;
use std::fs;

/// The value is only from 0 to 100 so it'll fit on a u8, but I chose to use
/// u32 for simpler operations with the input (that isn't limited).
type Dial = u32;
const DIAL_SIZE: u32 = 100;

fn main() {
    let mut current_dial: Dial = 50;
    let mut zeros_counter: u32 = 0;

    let inputs = read_inputs("res/day_01_input.txt");

    for input in &inputs {
        let new_zeros;
        (current_dial, new_zeros) = input.apply_to(current_dial);
        zeros_counter += new_zeros;
    }

    println!("Number of zeros: {zeros_counter}.");
}

enum Input {
    R(u32),
    L(u32),
}

impl Input {
    /// Apply the input to the dial and return the new dial plus the number of
    /// zeros encountered.
    fn apply_to(&self, dial: Dial) -> (Dial, u32) {
        match self {
            Input::R(n) => ((dial + n) % DIAL_SIZE, (dial + n) / 100),
            Input::L(n) => {
                let n = *n;
                if dial > n {
                    (dial - n, 0)
                } else {
                    let zeros = (n - dial) / DIAL_SIZE + 1;
                    (
                        (DIAL_SIZE * zeros + dial - n) % 100,
                        if dial != 0 { zeros } else { zeros - 1 },
                    )
                }
            }
        }
    }
}

impl FromStr for Input {
    type Err = String;

    /// Parse `text` that has the format `L{shift}` or `R{shift}`.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (direction, shift) = text.split_at(1);
        let shift: u32 = shift
            .parse()
            .expect("Invalid shift `{shift}`: can't convert it to u32.");

        match direction {
            "L" => Ok(Self::L(shift)),
            "R" => Ok(Self::R(shift)),
            c => Err(format!(
                "Invalid starting character `{c}` for direction in text `{text}`."
            )),
        }
    }
}

fn read_inputs(path: &str) -> Vec<Input> {
    let input_file_content = fs::read_to_string(path).expect("Should be able to read file input.");

    input_file_content
        .trim()
        .lines()
        .map(|line| Input::from_str(line).expect("Invalid input"))
        .collect()
}
