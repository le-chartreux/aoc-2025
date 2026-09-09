use std::{error::Error, fs, path::Path};

type Coordinate = u64;
type Area = u64;

#[derive(Debug, PartialEq)]
struct Position2d {
    x: Coordinate,
    y: Coordinate,
}

impl Position2d {
    fn new(x: Coordinate, y: Coordinate) -> Self {
        Position2d { x, y }
    }

    fn area_with_other_corner(&self, other: &Self) -> Area {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let red_tiles_positions = read_input("res/day_09_input.txt")?;
    let largest_rectangle_area = largest_area_of_any_rectangle(&red_tiles_positions);
    println!("The largest rectangle has area {largest_rectangle_area}.");
    Ok(())
}

fn largest_area_of_any_rectangle(positions: &[Position2d]) -> Area {
    let mut largest = 0;
    for (i, corner_1) in positions.iter().enumerate() {
        for corner_2 in &positions[i + 1..] {
            largest = largest.max(corner_1.area_with_other_corner(corner_2));
        }
    }
    largest
}

fn read_input(path: impl AsRef<Path>) -> Result<Vec<Position2d>, Box<dyn Error>> {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    read_input_from_content(&input_file_content)
}

fn read_input_from_content(content: &str) -> Result<Vec<Position2d>, Box<dyn Error>> {
    content
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(",")
                .ok_or_else(|| format!("invalid input line: {line:?}"))?;

            let x = x.parse::<Coordinate>()?;
            let y = y.parse::<Coordinate>()?;
            Ok(Position2d::new(x, y))
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_input_from_content_on_example() {
        let example_read = read_input_from_content(
            &["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"].join("\n"),
        )
        .expect("failed to parse example input");
        assert_eq!(example_read, get_example());
    }

    #[test]
    fn read_input_from_content_rejects_invalid_line() {
        let result = read_input_from_content("7,1\ninvalid");

        assert!(result.is_err());
    }

    #[test]
    fn read_input_from_content_rejects_missing_position() {
        let result = read_input_from_content("7");

        assert!(result.is_err());
    }

    #[test]
    fn area_with_other_corner_returns_expected() {
        let corner_1 = Position2d::new(3, 5);
        let corner_2 = Position2d::new(4, 2);
        assert_eq!(corner_1.area_with_other_corner(&corner_2), 8);
    }

    #[test]
    fn largest_area_of_any_rectangle_on_example() {
        assert_eq!(largest_area_of_any_rectangle(&get_example()), 50);
    }

    fn get_example() -> Vec<Position2d> {
        vec![
            Position2d::new(7, 1),
            Position2d::new(11, 1),
            Position2d::new(11, 7),
            Position2d::new(9, 7),
            Position2d::new(9, 5),
            Position2d::new(2, 5),
            Position2d::new(2, 3),
            Position2d::new(7, 3),
        ]
    }
}
