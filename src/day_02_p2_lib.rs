use std::ops::RangeInclusive;

pub type Id = u64;
pub type IdRange = RangeInclusive<Id>;
pub type IdSum = Id;

pub fn get_sum_of_invalid_ids(range: IdRange) -> IdSum {
    range.filter(|&id| is_id_invalid(id)).sum()
}

fn is_id_invalid(id: Id) -> bool {
    let number_of_digits = get_number_of_digits(id);

    for n in 1..number_of_digits {
        // TODO: que sur les multiples
        if number_of_digits.is_multiple_of(n) {
            if repeat(get_nth_first_digits(id, n), number_of_digits / n) == id {
                return true;
            }
        }
    }
    false
}

fn get_nth_first_digits(id: Id, n: u32) -> Id {
    id / (10 as Id).pow(get_number_of_digits(id) - n)
}

fn repeat(id: Id, times: u32) -> Id {
    let number_of_digits = get_number_of_digits(id);
    // id * (sum(10^n for n in 0..(number_of_digits*rep) avec un pas de number_of_digits))
    let multiplicator: Id = (0..(number_of_digits * times))
        .step_by(number_of_digits as usize)
        .map(|n| 10_u64.pow(n))
        .sum();
    multiplicator * id
}

fn get_number_of_digits(id: Id) -> u32 {
    id.ilog10() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_id_invalid_case_invalid() {
        for id in [
            11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824,
            2121212121,
        ] {
            assert!(is_id_invalid(id))
        }
    }

    #[test]
    fn test_is_id_invalid_case_valid() {
        for id in [4, 12, 23, 100, 110, 998, 1011, 118851855, 23456, 44646] {
            assert!(!is_id_invalid(id));
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
            assert_eq!(get_nth_first_digits(id, number_of_digits), expected);
        }
    }

    #[test]
    fn test_repeat() {
        let test_cases = [(1, 1, 1), (12, 1, 12), (12, 2, 1212), (123, 3, 123123123)];
        for (id, times, expected) in test_cases {
            assert_eq!(repeat(id, times), expected);
        }
    }

    #[test]
    fn test_get_number_of_digits() {
        let test_cases = [(3, 1), (37, 2), (481, 3), (1234567, 7)];
        for (id, expected) in test_cases {
            assert_eq!(get_number_of_digits(id), expected);
        }
    }
}
