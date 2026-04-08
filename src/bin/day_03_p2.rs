use std::fs;

// Only a single digit so could be a u8 but set to u64 to ease sum.
type BatteryJoltage = u64;
type Bank = Vec<BatteryJoltage>;

fn main() {
    let banks = read_inputs("res/day_03_input.txt");
    let total_output_joltage: BatteryJoltage = banks.iter().map(largest_possible_joltage).sum();
    println!("Total ouput joltage: {total_output_joltage}");
}

fn largest_possible_joltage(bank: &Bank) -> BatteryJoltage {
    const NUMBER_OF_DIGITS: usize = 12;
    assert!(
        bank.len() >= NUMBER_OF_DIGITS,
        "can't get largest possible joltage on a bank of less than {NUMBER_OF_DIGITS} elements"
    );

    let mut result = [0; NUMBER_OF_DIGITS];
    for (pos_bank, &joltage_bank) in bank.iter().enumerate() {
        let mut changed = false;
        for (pos_result, joltage_result) in result.iter_mut().enumerate() {
            if changed {
                *joltage_result = 0;
            } else if NUMBER_OF_DIGITS - pos_result <= bank.len() - pos_bank
                && joltage_bank > *joltage_result
            {
                *joltage_result = joltage_bank;
                changed = true;
            }
        }
    }

    result
        .iter()
        .enumerate()
        .map(|(pos, digit)| 10_u64.pow((NUMBER_OF_DIGITS - pos - 1) as u32) * digit)
        .sum()
}

fn read_inputs(path: &str) -> Vec<Bank> {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    let mut inputs = Vec::new();
    for line in input_file_content.lines() {
        let mut input = Vec::new();
        for char in line.chars() {
            input.push(
                char.to_digit(10)
                    .expect("failed to convert input char to digit")
                    .into(),
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
        assert_eq!(largest_possible_joltage(&bank), 987654321111);
    }

    #[test]
    fn largest_possible_joltage_on_811111111111119() {
        let bank = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(largest_possible_joltage(&bank), 811111111119);
    }

    #[test]
    fn largest_possible_joltage_on_234234234234278() {
        let bank = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(largest_possible_joltage(&bank), 434234234278);
    }

    #[test]
    fn largest_possible_joltage_on_818181911112111() {
        let bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(largest_possible_joltage(&bank), 888911112111);
    }
}
