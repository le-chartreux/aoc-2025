use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Debug)]
struct PaperGrid {
    rows: Vec<Vec<bool>>,
}

impl PaperGrid {
    fn remove_all_accessible_rolls(&mut self) {
        loop {
            let coordinates = self.available_rolls_coordinates();
            if coordinates.is_empty() {
                break;
            }
            for (x, y) in coordinates {
                self.remove_roll_at_coordinates(x, y);
            }
        }
    }

    fn available_rolls_coordinates(&self) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, elem) in row.iter().enumerate() {
                if *elem && self.count_rolls_around_coordinates(x, y) < 4 {
                    result.insert((x, y));
                }
            }
        }
        result
    }

    fn count_rolls_around_coordinates(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y = y as i32;
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .iter()
        .filter(|(x, y)| self.is_roll_present_at_coordinates(*x, *y))
        .count()
    }

    fn is_roll_present_at_coordinates(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let x = x as usize;
        let y = y as usize;
        if y >= self.rows.len() || x >= self.rows[y].len() {
            return false;
        }
        self.rows[y][x]
    }

    fn remove_roll_at_coordinates(&mut self, x: usize, y: usize) {
        if y >= self.rows.len() || x >= self.rows[y].len() {
            panic!("Can't remove a roll at ({x}, {y}): outside of the grid.");
        }
        self.rows[y][x] = false;
    }

    fn count_rolls(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.iter().filter(|elem| **elem).count())
            .sum()
    }
}

fn main() {
    let mut paper_grid = read_input("res/day_04_input.txt");
    let rolls_before = paper_grid.count_rolls();
    paper_grid.remove_all_accessible_rolls();
    let removed_rolls = rolls_before - paper_grid.count_rolls();
    println!("Removed rolls: {removed_rolls}");
}

fn read_input(path: &str) -> PaperGrid {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    let mut lines = Vec::new();
    for line in input_file_content.lines() {
        lines.push(line.chars().map(|char| char == '@').collect());
    }
    PaperGrid { rows: lines }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: bool = true; // Present
    const A: bool = false; // Absent

    #[test]
    fn remove_all_accessible_rolls_on_example() {
        let mut example = example();
        example.remove_all_accessible_rolls();

        let expected_rows = vec![
            vec![A, A, A, A, A, A, A, A, A, A],
            vec![A, A, A, A, A, A, A, A, A, A],
            vec![A, A, A, A, A, A, A, A, A, A],
            vec![A, A, A, A, P, P, A, A, A, A],
            vec![A, A, A, P, P, P, P, A, A, A],
            vec![A, A, A, P, P, P, P, P, A, A],
            vec![A, A, A, P, A, P, A, P, P, A],
            vec![A, A, A, P, P, A, P, P, P, A],
            vec![A, A, A, P, P, P, P, P, A, A],
            vec![A, A, A, A, P, P, P, A, A, A],
        ];
        let expected_result = PaperGrid {
            rows: expected_rows,
        };
        assert_eq!(example, expected_result);
    }

    #[test]
    fn available_rolls_coordinates_on_example() {
        let expected_result = HashSet::from([
            (0, 1),
            (0, 4),
            (0, 7),
            (0, 9),
            (2, 0),
            (2, 9),
            (3, 0),
            (5, 0),
            (6, 0),
            (6, 2),
            (8, 0),
            (8, 9),
            (9, 4),
        ]);
        assert_eq!(example().available_rolls_coordinates(), expected_result);
    }

    #[test]
    fn count_rolls_around_coordinates_on_example() {
        let example = example();
        assert_eq!(example.count_rolls_around_coordinates(0, 0), 2);
        assert_eq!(example.count_rolls_around_coordinates(9, 0), 3);
        assert_eq!(example.count_rolls_around_coordinates(0, 9), 1);
        assert_eq!(example.count_rolls_around_coordinates(9, 9), 2);
        assert_eq!(example.count_rolls_around_coordinates(3, 4), 7);
        assert_eq!(example.count_rolls_around_coordinates(4, 3), 7);
        assert_eq!(example.count_rolls_around_coordinates(8, 2), 4);
        assert_eq!(example.count_rolls_around_coordinates(2, 8), 5);
        assert_eq!(example.count_rolls_around_coordinates(9, 8), 4);
    }

    #[test]
    fn is_roll_present_at_coordinates_on_example() {
        let example = example();
        assert!(!example.is_roll_present_at_coordinates(0, 0));
        assert!(!example.is_roll_present_at_coordinates(9, 0));
        assert!(example.is_roll_present_at_coordinates(0, 9));
        assert!(!example.is_roll_present_at_coordinates(9, 9));
        assert!(!example.is_roll_present_at_coordinates(5, 7));
        assert!(example.is_roll_present_at_coordinates(3, 2));
    }

    #[test]
    fn remove_roll_at_coordinates_on_example() {
        let mut example = example();

        assert!(example.is_roll_present_at_coordinates(0, 1));
        example.remove_roll_at_coordinates(0, 1);
        assert!(!example.is_roll_present_at_coordinates(0, 1));

        assert!(example.is_roll_present_at_coordinates(8, 8));
        example.remove_roll_at_coordinates(8, 8);
        assert!(!example.is_roll_present_at_coordinates(0, 1));
        assert!(!example.is_roll_present_at_coordinates(8, 8));
    }

    #[test]
    fn count_rolls_on_example() {
        assert_eq!(example().count_rolls(), 71);
    }

    fn example() -> PaperGrid {
        let rows = vec![
            vec![A, A, P, P, A, P, P, P, P, A],
            vec![P, P, P, A, P, A, P, A, P, P],
            vec![P, P, P, P, P, A, P, A, P, P],
            vec![P, A, P, P, P, P, A, A, P, A],
            vec![P, P, A, P, P, P, P, A, P, P],
            vec![A, P, P, P, P, P, P, P, A, P],
            vec![A, P, A, P, A, P, A, P, P, P],
            vec![P, A, P, P, P, A, P, P, P, P],
            vec![A, P, P, P, P, P, P, P, P, A],
            vec![P, A, P, A, P, P, P, A, P, A],
        ];
        PaperGrid { rows }
    }
}
