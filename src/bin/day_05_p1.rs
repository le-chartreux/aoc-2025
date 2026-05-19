use std::{fs, ops::RangeInclusive};

type Id = u64;
type IdRange = RangeInclusive<Id>;

fn main() {
    let (ranges, ids) = read_input("res/day_05_input.txt");
    let total_fresh_ingredients = count_fresh_ingredients_matching_ids(&ranges, &ids);
    println!("Total fresh ingredients: {total_fresh_ingredients}");
}

fn count_fresh_ingredients_matching_ids(ranges: &[IdRange], ids: &[Id]) -> usize {
    ids.iter().filter(|id| is_fresh(**id, ranges)).count()
}

fn is_fresh(id: Id, ranges: &[IdRange]) -> bool {
    ranges.iter().any(|range| range.contains(&id))
}

fn read_input(path: &str) -> (Vec<IdRange>, Vec<Id>) {
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

    let ids = input_file_lines
        .map(|line| line.parse().expect("failed to parse ID"))
        .collect();

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_fresh_ingredients_matching_ids_on_example() {
        let ranges = [3..=5, 10..=14, 16..=20, 12..=18];
        let ids = [1, 5, 8, 11, 17, 32];
        assert_eq!(count_fresh_ingredients_matching_ids(&ranges, &ids), 3);
    }
}
