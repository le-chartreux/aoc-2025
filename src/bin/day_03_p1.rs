use std::fs;

// Only a single digit so could be a u8 but set to u32 to ease sum.
type BatteryJoltage = u32;
type Bank = Vec<BatteryJoltage>;

fn main() {
    let banks = read_inputs("res/day_03_input.txt");
    let total_output_joltage: BatteryJoltage = banks
        .iter()
        .map(|bank| bank.get_largest_possible_joltage())
        .sum();
    println!("Total ouput joltage: {total_output_joltage}");
}

trait LargestPossibleJoltage {
    fn get_largest_possible_joltage(&self) -> BatteryJoltage;
}

impl LargestPossibleJoltage for Bank {
    fn get_largest_possible_joltage(&self) -> BatteryJoltage {
        // It can't be the last digit as there's nothing to its right.
        let first_digit = self[..(self.len() - 1)]
            .iter()
            .max()
            .expect("failed to get max of Bank except last digit");

        let first_digit_position = self
            .iter()
            .position(|digit| digit == first_digit)
            .expect("failed to find position of max in Bank");
        
        // Only to the right of the max.
        let second_digit = self[(first_digit_position + 1)..]
            .iter()
            .max()
            .expect("failed to find second max digit in Bank");

        first_digit * 10 + second_digit
    }
}

fn read_inputs(path: &str) -> Vec<Bank> {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    let mut inputs = Vec::new();
    for line in input_file_content.lines() {
        let mut input = Vec::new();
        for char in line.chars() {
            input.push(
                char.to_digit(10)
                    .expect("failed to convert input char to digit"),
            );
        }
        inputs.push(input);
    }
    inputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_possible_joltage_on_987654321111111() {
        let input: Bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(input.get_largest_possible_joltage(), 98);
    }

    #[test]
    fn largest_possible_joltage_on_811111111111119() {
        let input: Bank = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(input.get_largest_possible_joltage(), 89);
    }

    #[test]
    fn largest_possible_joltage_on_234234234234278() {
        let input: Bank = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(input.get_largest_possible_joltage(), 78);
    }

    #[test]
    fn largest_possible_joltage_on_818181911112111() {
        let input: Bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(input.get_largest_possible_joltage(), 92);
    }
}
