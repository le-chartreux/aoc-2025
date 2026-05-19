use std::{fs, ops::RangeInclusive};

type Id = u64;
type IdRange = RangeInclusive<Id>;

fn main() {
    let ranges = read_input("res/day_05_input.txt");
    let total_fresh_ingredients_ids = count_fresh_ingredients_ids(&ranges);
    println!("Total fresh ingredients IDs: {total_fresh_ingredients_ids}");
}

fn count_fresh_ingredients_ids(ranges: &[IdRange]) -> usize {
    let min = ranges
        .iter()
        .map(RangeInclusive::start)
        .min()
        .expect("failed to get min of ranges");
    let max = ranges
        .iter()
        .map(RangeInclusive::end)
        .max()
        .expect("failed to get max of ranges");
    (*min..=*max).filter(|id| is_fresh(*id, ranges)).count()
}

fn is_fresh(id: Id, ranges: &[IdRange]) -> bool {
    ranges.iter().any(|range| range.contains(&id))
}

fn read_input(path: &str) -> Vec<IdRange> {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    let mut input_file_lines = input_file_content.lines();

    let mut ranges = Vec::new();
    while let Some(line) = input_file_lines.next()
        && !line.is_empty()
    {
        let mut split = line.split("-");
        if let (Some(inf), Some(sup)) = (split.next(), split.next()) {
            ranges.push(
                inf.parse().expect("failed to parse inf of range")
                    ..=sup.parse().expect("failed to parse sup of range"),
            );
        }
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_fresh_ingredients_ids_on_example() {
        let ranges = [3..=5, 10..=14, 16..=20, 12..=18];
        assert_eq!(count_fresh_ingredients_ids(&ranges), 14);
    }
}
