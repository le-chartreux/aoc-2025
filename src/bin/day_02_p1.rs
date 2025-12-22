use std::{
    fs::{self},
    ops::RangeInclusive,
};

type Id = u64;
type IdRange = RangeInclusive<Id>;
type IdSum = Id;

fn main() {
    let ranges = read_inputs("res/day_02_input.txt");
    let mut sum: IdSum = 0;
    for range in ranges {
        sum += get_invalid_ids_sum(range);
    }
    println!("Sum of invalid IDs: {sum}");
}

fn get_invalid_ids_sum(range: IdRange) -> IdSum {
    range.filter(|&id| is_id_invalid(id)).sum()
}

fn is_id_invalid(id: Id) -> bool {
    let number_of_digits = id.ilog10() + 1;
    if !number_of_digits.is_multiple_of(2) {
        // An ID can't be some digits repeted twice if it can't be split in
        // two equal parts.
        return false;
    }
    let cutter = (10 as Id).pow(number_of_digits / 2);
    let right_part = id % cutter;
    let left_part = id / cutter;
    right_part == left_part
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
