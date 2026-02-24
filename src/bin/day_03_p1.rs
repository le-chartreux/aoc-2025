use std::fs;

// Only a single digit so could be a u8 but set to u32 to ease sum.
type BatteryJoltage = u32;
type Bank = Vec<BatteryJoltage>;

fn main() {
    let banks = read_inputs("res/day_03_input.txt");
    let total_output_joltage: BatteryJoltage = banks.iter().map(largest_possible_joltage).sum();
    println!("Total ouput joltage: {total_output_joltage}");
}

fn largest_possible_joltage(bank: &Bank) -> BatteryJoltage {
    let mut left_digit = 0;
    let mut right_digit = 0;

    // The left digit can't be the last digit as there's nothing to its right.
    for i in bank[..(bank.len() - 1)].iter() {
        let i = *i;
        if i > left_digit {
            left_digit = i;
            right_digit = 0;
        } else if i > right_digit {
            right_digit = i
        }
    }

    let last_elem = *bank.last().expect("failed to get last element of Bank");
    if last_elem > right_digit {
        right_digit = last_elem;
    }

    left_digit * 10 + right_digit
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
        let bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(largest_possible_joltage(&bank), 98);
    }

    #[test]
    fn largest_possible_joltage_on_811111111111119() {
        let bank = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(largest_possible_joltage(&bank), 89);
    }

    #[test]
    fn largest_possible_joltage_on_234234234234278() {
        let bank = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(largest_possible_joltage(&bank), 78);
    }

    #[test]
    fn largest_possible_joltage_on_818181911112111() {
        let bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(largest_possible_joltage(&bank), 92);
    }
}
