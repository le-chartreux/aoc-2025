use std::ops::RangeInclusive;

use std::fs;

type Id = u64;
type IdRange = RangeInclusive<Id>;
type IdSum = Id;

fn main() {
    let ranges = read_inputs("res/day_02_input.txt");
    let mut sum: IdSum = 0;
    for range in ranges {
        sum += get_sum_of_invalid_ids(range);
    }
    println!("Sum of invalid IDs: {sum}");
}

fn get_sum_of_invalid_ids(range: IdRange) -> IdSum {
    range.filter(|&id| id.is_invalid()).sum()
}

trait Invalidity {
    fn is_invalid(&self) -> bool;
}

trait CountDigits {
    fn get_number_of_digits(&self) -> u32;
}

trait Repeat {
    fn repeat(&self, times: u32) -> Self;
}

trait FirstDigits {
    fn get_nth_first_digits(&self, n: u32) -> Self;
}

impl Invalidity for Id {
    fn is_invalid(&self) -> bool {
        let number_of_digits = self.get_number_of_digits();

        for n in 1..number_of_digits {
            // TODO: que sur les multiples
            if number_of_digits.is_multiple_of(n)
                && self.get_nth_first_digits(n).repeat(number_of_digits / n) == *self
            {
                return true;
            }
        }
        false
    }
}

impl CountDigits for Id {
    fn get_number_of_digits(&self) -> u32 {
        self.ilog10() + 1
    }
}

impl Repeat for Id {
    fn repeat(&self, times: u32) -> Self {
        let multiplicator: Id = (0..(self.get_number_of_digits() * times))
            .step_by(self.get_number_of_digits() as usize)
            .map(|n| 10_u64.pow(n))
            .sum();
        multiplicator * self
    }
}

impl FirstDigits for Id {
    fn get_nth_first_digits(&self, n: u32) -> Id {
        self / (10 as Id).pow(self.get_number_of_digits() - n)
    }
}

fn read_inputs(path: &str) -> Vec<IdRange> {
    let input_file_content = fs::read_to_string(path).expect("Should be able to read file input.");

    input_file_content
        .trim()
        .split(',')
        .map(read_input)
        .collect()
}

fn read_input(input: &str) -> IdRange {
    let input: Vec<&str> = input.split('-').collect();
    if input.len() != 2 {
        panic!("Invalid input {:?}: range should be 2 elements.", input);
    }
    let start: Id = input[0].parse().expect("Can't convert start of ID");
    let end: Id = input[1].parse().expect("Can't convert end of ID");
    start..=end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_case_invalid() {
        for id in [
            11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824,
            2121212121,
        ] {
            assert!(id.is_invalid());
        }
    }

    #[test]
    fn test_is_invalid_case_valid() {
        for id in [4, 12, 23, 100, 110, 998, 1011, 118851855, 23456, 44646] {
            assert!(!id.is_invalid());
        }
    }

    #[test]
    fn test_get_nth_first_digits() {
        let test_cases = [
            (12345, 1, 1),
            (12345, 2, 12),
            (12345, 3, 123),
            (12345, 4, 1234),
            (12345, 5, 12345),
            (123456789, 7, 1234567),
        ];
        for (id, number_of_digits, expected) in test_cases {
            assert_eq!(id.get_nth_first_digits(number_of_digits), expected);
        }
    }

    #[test]
    fn test_repeat() {
        let test_cases = [(1, 1, 1), (12, 1, 12), (12, 2, 1212), (123, 3, 123123123)];
        for (id, times, expected) in test_cases {
            assert_eq!(id.repeat(times), expected);
        }
    }

    #[test]
    fn test_get_number_of_digits() {
        let test_cases = [(3, 1), (37, 2), (481, 3), (1234567, 7)];
        for (id, expected) in test_cases {
            assert_eq!(id.get_number_of_digits(), expected);
        }
    }
}
