use std::{collections::HashSet, fs};

type Id = u64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}

#[derive(Debug)]
struct IdRanges {
    ranges: HashSet<IdRange>,
}

impl IdRanges {
    fn new() -> Self {
        IdRanges {
            ranges: HashSet::new(),
        }
    }

    /// Calculate the number of IDs inside all the ranges.
    fn total_number_of_ids(&self) -> usize {
        self.ranges.iter().map(IdRange::size).sum()
    }

    /// Add a new range to the ranges, modifing and/or deleting existing ranges if necessary.
    fn add(&mut self, mut range: IdRange) {
        // Expand to the start of the range where the range's start is in, if any.
        if let Some(IdRange {
            start: range_start,
            end: _,
        }) = self
            .ranges
            .iter()
            .find(|existing_range| existing_range.contains_id(range.start))
        {
            range.start = *range_start;
        }

        // Expand to the end of the range where the range's end is in, if any.
        if let Some(IdRange {
            start: _,
            end: range_end,
        }) = self
            .ranges
            .iter()
            .find(|existing_range| existing_range.contains_id(range.end))
        {
            range.end = *range_end;
        }

        // Remove existing ranges in between the start and the end of the range.
        self.ranges.retain(|existing_range| {
            !(range.contains_id(existing_range.start) || range.contains_id(existing_range.end))
        });

        self.ranges.insert(range);
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
        assert_eq!(ranges.ranges, HashSet::from_iter(vec![IdRange::new(3, 5)]));

        ranges.add(IdRange::new(10, 14));
        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![IdRange::new(3, 5), IdRange::new(10, 14)])
        );

        ranges.add(IdRange::new(16, 20));
        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![
                IdRange::new(3, 5),
                IdRange::new(10, 14),
                IdRange::new(16, 20),
            ])
        );

        ranges.add(IdRange::new(12, 18));
        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![IdRange::new(3, 5), IdRange::new(10, 20)])
        );
    }

    #[test]
    fn add_id_range_contained_in_other() {
        let mut ranges = IdRanges::new();
        ranges.add(IdRange::new(3, 5));
        ranges.add(IdRange::new(10, 14));
        ranges.add(IdRange::new(16, 20));
        ranges.add(IdRange::new(11, 13));

        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![
                IdRange::new(3, 5),
                IdRange::new(10, 14),
                IdRange::new(16, 20),
            ])
        );
    }

    #[test]
    fn add_id_range_extending_one_no_overlap() {
        let mut ranges = IdRanges::new();
        ranges.add(IdRange::new(3, 5));
        ranges.add(IdRange::new(10, 14));
        ranges.add(IdRange::new(16, 20));
        ranges.add(IdRange::new(11, 15));

        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![
                IdRange::new(3, 5),
                IdRange::new(10, 15),
                IdRange::new(16, 20),
            ])
        );
    }

    #[test]
    fn add_id_range_extending_one_with_overlap_over_maximum() {
        let mut ranges = IdRanges::new();
        ranges.add(IdRange::new(3, 5));
        ranges.add(IdRange::new(10, 14));
        ranges.add(IdRange::new(16, 20));
        ranges.add(IdRange::new(11, 40));

        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![IdRange::new(3, 5), IdRange::new(10, 40)])
        );
    }

    #[test]
    fn add_id_range_extending_one_with_one_overlap() {
        let mut ranges = IdRanges::new();
        ranges.add(IdRange::new(3, 5));
        ranges.add(IdRange::new(10, 14));
        ranges.add(IdRange::new(16, 20));
        ranges.add(IdRange::new(2, 15));

        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![IdRange::new(2, 15), IdRange::new(16, 20)])
        );
    }

    #[test]
    fn add_id_range_before_others() {
        let mut ranges = IdRanges::new();
        ranges.add(IdRange::new(3, 5));
        ranges.add(IdRange::new(10, 14));
        ranges.add(IdRange::new(16, 20));
        ranges.add(IdRange::new(1, 2));

        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![
                IdRange::new(1, 2),
                IdRange::new(3, 5),
                IdRange::new(10, 14),
                IdRange::new(16, 20)
            ])
        );
    }

    #[test]
    fn add_id_range_after_other() {
        let mut ranges = IdRanges::new();
        ranges.add(IdRange::new(3, 5));
        ranges.add(IdRange::new(10, 14));

        assert_eq!(
            ranges.ranges,
            HashSet::from_iter(vec![IdRange::new(3, 5), IdRange::new(10, 14),])
        );
    }
}
