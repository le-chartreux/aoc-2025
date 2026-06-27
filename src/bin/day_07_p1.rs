use std::{fs, iter::zip};

#[derive(PartialEq, Debug)]
enum ManifoldElement {
    TachyonBeam,
    EmptySpace,
    Splitter,
}

impl From<char> for ManifoldElement {
    fn from(value: char) -> Self {
        match value {
            '.' => ManifoldElement::EmptySpace,
            'S' | '|' => ManifoldElement::TachyonBeam,
            '^' => ManifoldElement::Splitter,
            _ => panic!("failed to read input: unknown char"),
        }
    }
}

#[derive(PartialEq, Debug)]
struct ManifoldLine {
    line: Vec<ManifoldElement>,
}

impl std::iter::FromIterator<ManifoldElement> for ManifoldLine {
    fn from_iter<T: IntoIterator<Item = ManifoldElement>>(iter: T) -> Self {
        Self {
            line: iter.into_iter().collect(),
        }
    }
}

impl ManifoldLine {
    fn expand_beams_of_previous(&mut self, beams_positions_of_previous: &[usize]) {
        for &beam_position_of_previous in beams_positions_of_previous {
            match self.line[beam_position_of_previous] {
                ManifoldElement::TachyonBeam => {}
                ManifoldElement::EmptySpace => {
                    self.line[beam_position_of_previous] = ManifoldElement::TachyonBeam
                }
                ManifoldElement::Splitter => {
                    self.add_beams_around(beam_position_of_previous);
                }
            }
        }
    }

    fn add_beams_around(&mut self, position: usize) {
        if position >= 1 && self.line[position - 1] == ManifoldElement::EmptySpace {
            self.line[position - 1] = ManifoldElement::TachyonBeam;
        }
        if position + 1 < self.line.len() && self.line[position + 1] == ManifoldElement::EmptySpace
        {
            self.line[position + 1] = ManifoldElement::TachyonBeam;
        }
    }

    fn get_beams_positions(&self) -> Vec<usize> {
        self.line
            .iter()
            .enumerate()
            .filter_map(|(pos, elem)| (*elem == ManifoldElement::TachyonBeam).then_some(pos))
            .collect()
    }

    fn get_splitters_positions(&self) -> Vec<usize> {
        self.line
            .iter()
            .enumerate()
            .filter_map(|(pos, elem)| (*elem == ManifoldElement::Splitter).then_some(pos))
            .collect()
    }

    fn len(&self) -> usize {
        self.line.len()
    }
}

#[derive(PartialEq, Debug)]
struct Manifold {
    lines: Vec<ManifoldLine>,
}

impl std::iter::FromIterator<ManifoldLine> for Manifold {
    fn from_iter<T: IntoIterator<Item = ManifoldLine>>(iter: T) -> Self {
        let lines: Vec<ManifoldLine> = iter.into_iter().collect();
        let first_line_len = lines[0].len();
        assert!(
            lines.iter().all(|line| line.len() == first_line_len),
            "all lines of manifold should have the same len"
        );
        Self { lines }
    }
}

impl Manifold {
    fn count_number_of_beam_split(&self) -> usize {
        zip(
            self.lines[..self.lines.len() - 1].iter(),
            self.lines[1..].iter(),
        )
        .map(|(previous_line, current_line)| {
            let previous_beams_positions = previous_line.get_beams_positions();
            current_line
                .get_splitters_positions()
                .iter()
                .filter(|&pos| previous_beams_positions.contains(pos))
                .count()
        })
        .sum()
    }

    fn expand_beam_of_first_line(&mut self) {
        for i in 1..self.lines.len() {
            let beams_positions_of_previous = self.lines[i - 1].get_beams_positions();
            self.lines[i].expand_beams_of_previous(&beams_positions_of_previous);
        }
    }
}

fn main() {
    let mut manifold = read_input("res/day_07_input.txt");
    manifold.expand_beam_of_first_line();
    let total_of_splits = manifold.count_number_of_beam_split();
    println!("The tachyon beam was split a total of {total_of_splits} times.");
}

fn read_input(path: &str) -> Manifold {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");

    read_input_from_content(&input_file_content)
}

fn read_input_from_content(content: &str) -> Manifold {
    content
        .lines()
        .map(|line| line.chars().map(ManifoldElement::from).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifold_element_from_char() {
        assert_eq!(ManifoldElement::from('.'), ManifoldElement::EmptySpace);
        assert_eq!(ManifoldElement::from('S'), ManifoldElement::TachyonBeam);
        assert_eq!(ManifoldElement::from('|'), ManifoldElement::TachyonBeam);
        assert_eq!(ManifoldElement::from('^'), ManifoldElement::Splitter);
    }

    #[test]
    fn expand_beams_of_previous() {
        let mut line = ManifoldLine {
            line: vec![
                ManifoldElement::EmptySpace,
                ManifoldElement::EmptySpace,
                ManifoldElement::EmptySpace,
                ManifoldElement::Splitter,
                ManifoldElement::Splitter,
                ManifoldElement::EmptySpace,
                ManifoldElement::Splitter,
                ManifoldElement::EmptySpace,
            ],
        };
        line.expand_beams_of_previous(&[0, 2, 4, 5, 6]);
        assert_eq!(
            line,
            ManifoldLine {
                line: vec![
                    ManifoldElement::TachyonBeam,
                    ManifoldElement::EmptySpace,
                    ManifoldElement::TachyonBeam,
                    ManifoldElement::Splitter,
                    ManifoldElement::Splitter,
                    ManifoldElement::TachyonBeam,
                    ManifoldElement::Splitter,
                    ManifoldElement::TachyonBeam,
                ]
            }
        )
    }

    #[test]
    fn read_input_from_content_example_first_5_lines() {
        let content = [
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
        ]
        .join("\n");
        assert_eq!(
            read_input_from_content(&content),
            Manifold {
                lines: vec![
                    ManifoldLine {
                        line: vec![
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::TachyonBeam,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                        ]
                    },
                    ManifoldLine {
                        line: vec![
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                        ]
                    },
                    ManifoldLine {
                        line: vec![
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::Splitter,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                        ]
                    },
                    ManifoldLine {
                        line: vec![
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                        ]
                    },
                    ManifoldLine {
                        line: vec![
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::Splitter,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::Splitter,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                            ManifoldElement::EmptySpace,
                        ]
                    },
                ]
            }
        )
    }

    #[test]
    fn expand_beam_of_first_line_on_example() {
        let mut example = read_input_from_content(
            &[
                ".......S.......",
                "...............",
                ".......^.......",
                "...............",
                "......^.^......",
                "...............",
                ".....^.^.^.....",
                "...............",
                "....^.^...^....",
                "...............",
                "...^.^...^.^...",
                "...............",
                "..^...^.....^..",
                "...............",
                ".^.^.^.^.^...^.",
                "...............",
            ]
            .join("\n"),
        );
        example.expand_beam_of_first_line();
        assert_eq!(
            example,
            read_input_from_content(
                &[
                    ".......S.......",
                    ".......|.......",
                    "......|^|......",
                    "......|.|......",
                    ".....|^|^|.....",
                    ".....|.|.|.....",
                    "....|^|^|^|....",
                    "....|.|.|.|....",
                    "...|^|^|||^|...",
                    "...|.|.|||.|...",
                    "..|^|^|||^|^|..",
                    "..|.|.|||.|.|..",
                    ".|^|||^||.||^|.",
                    ".|.|||.||.||.|.",
                    "|^|^|^|^|^|||^|",
                    "|.|.|.|.|.|||.|",
                ]
                .join("\n")
            )
        )
    }

    #[test]
    fn count_number_of_beam_split_on_example_resolved() {
        let example = read_input_from_content(
            &[
                ".......S.......",
                ".......|.......",
                "......|^|......",
                "......|.|......",
                ".....|^|^|.....",
                ".....|.|.|.....",
                "....|^|^|^|....",
                "....|.|.|.|....",
                "...|^|^|||^|...",
                "...|.|.|||.|...",
                "..|^|^|||^|^|..",
                "..|.|.|||.|.|..",
                ".|^|||^||.||^|.",
                ".|.|||.||.||.|.",
                "|^|^|^|^|^|||^|",
                "|.|.|.|.|.|||.|",
            ]
            .join("\n"),
        );
        assert_eq!(example.count_number_of_beam_split(), 21);
    }
}
