use std::{
    fs::{self},
};

use aoc_2025::day_02_p2_lib::{Id, IdRange, IdSum, get_invalid_ids_sum};

fn main() {
    let ranges = read_inputs("res/day_02_input.txt");
    let mut sum: IdSum = 0;
    for range in ranges {
        sum += get_invalid_ids_sum(range);
    }
    println!("Sum of invalid IDs: {sum}");
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
