use std::ops::RangeInclusive;

pub type Id = u64;
pub type IdRange = RangeInclusive<Id>;
pub type IdSum = Id;

pub fn get_invalid_ids_sum(range: IdRange) -> IdSum {
    range.filter(|&id| is_id_invalid(id)).sum()
}

fn is_id_invalid(id: Id) -> bool {
    // TODO: put in dedicated function
    let number_of_digits = id.ilog10() + 1;

    for n in 1..number_of_digits {
        // TODO: que sur les multiples
        if number_of_digits.is_multiple_of(n) {
            // si les n premiers number répétés number_of_digits/n fois donnent id
            if repeat(get_nth_first_digits(id, n), number_of_digits / n) == id {
                return true
            }
        }
    }
    false
}

fn get_nth_first_digits(id: Id, n: u32) -> Id {
    let number_of_digits = id.ilog10() + 1;
    id / (10 as Id).pow(number_of_digits - n)
}

fn repeat(id: Id, times: u32) -> Id {
    let number_of_digits = id.ilog10() + 1;
    // id * (sum(10^n for n in 0..(number_of_digits*rep) avec un pas de number_of_digits))
    let multiplicator: Id = (0..(number_of_digits*times))
        .step_by(number_of_digits as usize)
        .map(|n| 10_u64.pow(n))
        .sum();
    multiplicator * id
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_id_invalid_case_invalid() {
        assert!(is_id_invalid(11));
        assert!(is_id_invalid(22));
        assert!(is_id_invalid(99));
        assert!(is_id_invalid(111));
        assert!(is_id_invalid(999));
        assert!(is_id_invalid(1010));
        assert!(is_id_invalid(1188511885));
        assert!(is_id_invalid(222222));
        assert!(is_id_invalid(446446));
        assert!(is_id_invalid(38593859));
        assert!(is_id_invalid(565656));
        assert!(is_id_invalid(824824824));
        assert!(is_id_invalid(2121212121));
    }

    #[test]
    fn is_id_invalid_case_valid() {
        assert!(!is_id_invalid(12));
        assert!(!is_id_invalid(23));
        assert!(!is_id_invalid(100));
        assert!(!is_id_invalid(110));
        assert!(!is_id_invalid(998));
        assert!(!is_id_invalid(1011));
        assert!(!is_id_invalid(118851885));
        assert!(!is_id_invalid(23456));
        assert!(!is_id_invalid(44646));
    }

    #[test]
    fn test_get_nth_first_digits() {
        assert_eq!(get_nth_first_digits(12345, 1), 1);
        assert_eq!(get_nth_first_digits(12345, 2), 12);
        assert_eq!(get_nth_first_digits(12345, 3), 123);
        assert_eq!(get_nth_first_digits(12345, 4), 1234);
        assert_eq!(get_nth_first_digits(12345, 5), 12345);
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat(1, 1), 1);
        assert_eq!(repeat(12, 1), 12);
        assert_eq!(repeat(12, 2), 1212);
        assert_eq!(repeat(123, 3), 123123123);
    }
}
