use std::fs;
use std::mem;

type Id = u64;

#[derive(Debug, Clone, PartialEq)]
struct IdRange {
    start: Id,
    end: Id,
}

impl IdRange {
    fn new(start: Id, end: Id) -> Self {
        IdRange { start, end }
    }

    fn size(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    fn contains_id(&self, id: Id) -> bool {
        id >= self.start && id <= self.end
    }

    fn contains_id_range(&self, range: &IdRange) -> bool {
        self.contains_id(range.start) && self.contains_id(range.end)
    }
}

#[derive(Debug)]
struct IdRanges {
    ranges: Vec<IdRange>,
}

impl IdRanges {
    fn new() -> Self {
        IdRanges { ranges: vec![] }
    }
    
    /// Calculate the number of IDs inside all the ranges.
    fn total_number_of_ids(&self) -> usize {
        self.ranges.iter().map(IdRange::size).sum()
    }
    
    /// Add a new range to the ranges, modifing existing ranges if necessary.
    fn add(&mut self, range: IdRange) {
        if self.ranges.is_empty() {
            self.ranges.push(range);
            return;
        }

        let mut inserted = false;
        let mut i = 0;
        while i < self.ranges.len() && !inserted {
            let existing_range = self.ranges.get_mut(i).expect("can't obtain ranges[i]");
            inserted = existing_range.contains_id_range(&range);
            if range.start < existing_range.start && range.end >= existing_range.start {
                existing_range.start = range.start;
                inserted = true;
            }
            if range.end > existing_range.end && range.start <= existing_range.end {
                existing_range.end = range.end;
                inserted = true;
            }
            i += 1;
        }
        if inserted {
            self.reload();
        } else {
            self.ranges.push(range);
        }
    }
    
    /// Re-add all the ranges of the vector so overlapping ranges are merged.
    fn reload(&mut self) {
        let old_ranges = mem::take(&mut self.ranges);
        for range in old_ranges {
            self.add(range);
        }
    }
}

fn main() {
    let ranges = read_input("res/day_05_input.txt");
    let total_fresh_ingredients_ids = ranges.total_number_of_ids();
    println!("Total fresh ingredients IDs: {total_fresh_ingredients_ids}");
}

fn read_input(path: &str) -> IdRanges {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    let mut input_file_lines = input_file_content.lines();

    let mut ranges = IdRanges::new();
    while let Some(line) = input_file_lines.next()
        && !line.is_empty()
    {
        let mut split = line.split("-");
        if let (Some(inf), Some(sup)) = (split.next(), split.next()) {
            ranges.add(IdRange::new(
                inf.parse().expect("failed to parse inf of range"),
                sup.parse().expect("failed to parse sup of range"),
            ));
        }
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_number_of_ids_on_example() {
        let mut ranges = IdRanges::new();
        for (start, end) in [(3, 5), (10, 14), (16, 20), (12, 18)] {
            ranges.add(IdRange::new(start, end));
        }
        assert_eq!(ranges.total_number_of_ids(), 14);
    }

    #[test]
    fn add_id_range_on_example() {
        let mut ranges = IdRanges::new();

        ranges.add(IdRange::new(3, 5));
        assert_eq!(ranges.ranges, vec![IdRange::new(3, 5)]);

        ranges.add(IdRange::new(10, 14));
        assert_eq!(
            ranges.ranges,
            vec![IdRange::new(3, 5), IdRange::new(10, 14)]
        );

        ranges.add(IdRange::new(16, 20));
        assert_eq!(
            ranges.ranges,
            vec![
                IdRange::new(3, 5),
                IdRange::new(10, 14),
                IdRange::new(16, 20)
            ]
        );

        ranges.add(IdRange::new(12, 18));
        assert_eq!(
            ranges.ranges,
            vec![IdRange::new(3, 5), IdRange::new(10, 20)]
        );
    }
}
