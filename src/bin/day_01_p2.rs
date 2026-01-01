use std::fs;
use std::str::FromStr;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_to() {
        let mut dial = 50;
        let mut number_of_zeros;

        (dial, number_of_zeros) = Input::L(68).apply_to(dial);
        assert_eq!(dial, 82);
        assert_eq!(number_of_zeros, 1);

        (dial, number_of_zeros) = Input::L(30).apply_to(dial);
        assert_eq!(dial, 52);
        assert_eq!(number_of_zeros, 0);

        (dial, number_of_zeros) = Input::R(48).apply_to(dial);
        assert_eq!(dial, 0);
        assert_eq!(number_of_zeros, 1);

        (dial, number_of_zeros) = Input::L(5).apply_to(dial);
        assert_eq!(dial, 95);
        assert_eq!(number_of_zeros, 0);

        (dial, number_of_zeros) = Input::R(60).apply_to(dial);
        assert_eq!(dial, 55);
        assert_eq!(number_of_zeros, 1);

        (dial, number_of_zeros) = Input::L(55).apply_to(dial);
        assert_eq!(dial, 0);
        assert_eq!(number_of_zeros, 1);

        (dial, number_of_zeros) = Input::L(1).apply_to(dial);
        assert_eq!(dial, 99);
        assert_eq!(number_of_zeros, 0);

        (dial, number_of_zeros) = Input::L(99).apply_to(dial);
        assert_eq!(dial, 0);
        assert_eq!(number_of_zeros, 1);

        (dial, number_of_zeros) = Input::R(14).apply_to(dial);
        assert_eq!(dial, 14);
        assert_eq!(number_of_zeros, 0);

        (dial, number_of_zeros) = Input::L(82).apply_to(dial);
        assert_eq!(dial, 32);
        assert_eq!(number_of_zeros, 1);

        (dial, number_of_zeros) = Input::R(491).apply_to(dial);
        assert_eq!(dial, 23);
        assert_eq!(number_of_zeros, 5);

        (dial, number_of_zeros) = Input::L(319).apply_to(dial);
        assert_eq!(dial, 4);
        assert_eq!(number_of_zeros, 3);
    }
}
