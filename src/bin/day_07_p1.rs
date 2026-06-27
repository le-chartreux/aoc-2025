use std::collections::HashSet;
use std::fs;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ManifoldElement {
    TachyonBeam,
    EmptySpace,
    Splitter,
}

impl From<char> for ManifoldElement {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::EmptySpace,
            'S' | '|' => Self::TachyonBeam,
            '^' => Self::Splitter,
            _ => panic!("unknown manifold element: {value:?}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ManifoldLine {
    line: Vec<ManifoldElement>,
}

impl Deref for ManifoldLine {
    type Target = [ManifoldElement];

    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

impl DerefMut for ManifoldLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.line
    }
}

impl std::iter::FromIterator<ManifoldElement> for ManifoldLine {
    fn from_iter<T: IntoIterator<Item = ManifoldElement>>(iter: T) -> Self {
        Self {
            line: iter.into_iter().collect(),
        }
    }
}

impl ManifoldLine {
    fn expand_beams_of_previous(
        &mut self,
        previous_beams_positions: impl IntoIterator<Item = usize>,
    ) {
        for previous_beams_position in previous_beams_positions {
            self.expand_beam_of_previous(previous_beams_position);
        }
    }

    fn expand_beam_of_previous(&mut self, previous_beam_position: usize) {
        match self[previous_beam_position] {
            ManifoldElement::EmptySpace => {
                self[previous_beam_position] = ManifoldElement::TachyonBeam
            }
            ManifoldElement::Splitter => {
                self.add_beams_around(previous_beam_position);
            }
            ManifoldElement::TachyonBeam => {}
        }
    }

    fn add_beams_around(&mut self, position: usize) {
        for &neighbor in [
            position.checked_sub(1),
            (position + 1 < self.len()).then_some(position + 1),
        ]
        .iter()
        .flatten()
        {
            if matches!(self[neighbor], ManifoldElement::EmptySpace) {
                self[neighbor] = ManifoldElement::TachyonBeam;
            }
        }
    }

    fn beams_positions(&self) -> impl Iterator<Item = usize> {
        self.iter()
            .enumerate()
            .filter_map(|(pos, elem)| (matches!(elem, ManifoldElement::TachyonBeam)).then_some(pos))
    }

    fn splitters_positions(&self) -> impl Iterator<Item = usize> {
        self.iter()
            .enumerate()
            .filter_map(|(pos, elem)| (matches!(elem, ManifoldElement::Splitter)).then_some(pos))
    }
}

#[derive(Debug, PartialEq, Eq)]
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
        self.lines
            .windows(2)
            .map(|window| {
                let previous_line = &window[0];
                let current_line = &window[1];
                let previous_beams: HashSet<usize> = previous_line.beams_positions().collect();
                current_line
                    .splitters_positions()
                    .filter(|splitter_position| previous_beams.contains(splitter_position))
                    .count()
            })
            .sum()
    }

    fn expand_beams_of_first_line(&mut self) {
        for i in 1..self.lines.len() {
            let previous_beams: HashSet<usize> = self.lines[i - 1].beams_positions().collect();
            self.lines[i].expand_beams_of_previous(previous_beams);
        }
    }
}

fn main() {
    let mut manifold = read_input("res/day_07_input.txt");
    manifold.expand_beams_of_first_line();
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
        line.expand_beams_of_previous([0, 2, 4, 5, 6]);
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
        example.expand_beams_of_first_line();
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
