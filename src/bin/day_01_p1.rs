use std::{fs, str::FromStr};

/// The value is only from 0 to 100 so it'll fit on a u8, but I chose to use
/// u32 for simpler operations with the input (that isn't limited).
type Dial = u32;
const DIAL_SIZE: u32 = 100;

fn main() {
    let mut current_dial: Dial = 50;
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
    fn apply_to(&self, current_dial: Dial) -> Dial {
        match self {
            Input::R(n) => (current_dial + n) % DIAL_SIZE,
            Input::L(n) => {
                let n = *n % DIAL_SIZE;
                if current_dial >= n {
                    current_dial - n
                } else {
                    DIAL_SIZE - (n - current_dial)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_to() {
        let mut dial = 50;

        dial = Input::L(68).apply_to(dial);
        assert_eq!(dial, 82);

        dial = Input::L(30).apply_to(dial);
        assert_eq!(dial, 52);

        dial = Input::R(48).apply_to(dial);
        assert_eq!(dial, 0);

        dial = Input::L(5).apply_to(dial);
        assert_eq!(dial, 95);

        dial = Input::R(60).apply_to(dial);
        assert_eq!(dial, 55);

        dial = Input::L(55).apply_to(dial);
        assert_eq!(dial, 0);

        dial = Input::L(1).apply_to(dial);
        assert_eq!(dial, 99);

        dial = Input::L(99).apply_to(dial);
        assert_eq!(dial, 0);

        dial = Input::R(14).apply_to(dial);
        assert_eq!(dial, 14);

        dial = Input::L(82).apply_to(dial);
        assert_eq!(dial, 32);
    }
}
