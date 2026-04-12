use std::fs;

struct PaperGrid {
    rows: Vec<Vec<bool>>,
}

impl PaperGrid {
    fn count_total_available_rolls(&self) -> u32 {
        self.rows
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(x, elem)| **elem && self.count_rolls_around_coordinates(*x, y) < 4)
                    .count() as u32
            })
            .sum()
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
}

fn main() {
    let paper_grid = read_input("res/day_04_input.txt");
    let total_available_rolls = paper_grid.count_total_available_rolls();
    println!("Total available rolls: {total_available_rolls}");
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

    #[test]
    fn count_total_available_rolls_on_example() {
        const P: bool = true; // Present
        const A: bool = false; // Absent
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
        let example = PaperGrid { rows };
        assert_eq!(example.count_total_available_rolls(), 13);
    }
}
