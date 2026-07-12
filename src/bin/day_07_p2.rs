use std::fs;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ManifoldElement {
    TachyonBeamStart,
    EmptySpace,
    Splitter,
}

impl From<char> for ManifoldElement {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::EmptySpace,
            'S' => Self::TachyonBeamStart,
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
    /// Expand beams of previous line to this one, and return the new positions of beams.
    /// With quantum tachyon, multiple beams can have the same position, as they're in
    /// a different timeline.
    fn expand_beams_of_previous(
        &self,
        previous_beams_positions: impl IntoIterator<Item = (usize, u64)>,
    ) -> (Vec<(usize, u64)>, u64) {
        let mut result = Vec::<(usize, u64)>::new();
        let mut number_of_split = 0;
        for (previous_beam_position, number_of_beams) in previous_beams_positions {
            match self[previous_beam_position] {
                ManifoldElement::EmptySpace => {
                    ManifoldLine::add_to_beams_positions(
                        previous_beam_position,
                        number_of_beams,
                        &mut result,
                    );
                }
                ManifoldElement::Splitter => {
                    number_of_split += number_of_beams;
                    for neighbor in self.get_empty_existing_neighbors(previous_beam_position) {
                        ManifoldLine::add_to_beams_positions(
                            neighbor,
                            number_of_beams,
                            &mut result,
                        );
                    }
                }
                ManifoldElement::TachyonBeamStart => {}
            }
        }
        (result, number_of_split)
    }

    fn add_to_beams_positions(
        position: usize,
        number_of_beams: u64,
        beams_positions: &mut Vec<(usize, u64)>,
    ) {
        let mut existing_position = beams_positions
            .iter_mut()
            .filter(|(pos, _)| *pos == position);
        if let Some(beam_position) = existing_position.next() {
            beam_position.1 += number_of_beams;
        } else {
            beams_positions.push((position, number_of_beams));
        }
    }

    /// Get neighbors of the position that are empty and aren't out of bound.
    fn get_empty_existing_neighbors(&self, position: usize) -> Vec<usize> {
        [
            position.checked_sub(1),
            (position + 1 < self.len()).then_some(position + 1),
        ]
        .iter()
        .flatten()
        .filter_map(|&neighbor| {
            matches!(self[neighbor], ManifoldElement::EmptySpace).then_some(neighbor)
        })
        .collect::<Vec<usize>>()
    }

    fn start_beams_positions(&self) -> Vec<(usize, u64)> {
        self.iter()
            .enumerate()
            .filter_map(|(pos, elem)| {
                (matches!(elem, ManifoldElement::TachyonBeamStart)).then_some((pos, 1))
            })
            .collect()
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
            "all lines of manifold must have the same len"
        );
        Self { lines }
    }
}

impl Manifold {
    fn count_number_of_beam_split(&self) -> u64 {
        let mut previous_beams_positions = Vec::<(usize, u64)>::new();
        let mut total_beam_split = 0;
        for line in &self.lines {
            let (mut current_beams_positions, beam_splits_current_line) =
                line.expand_beams_of_previous(previous_beams_positions);
            total_beam_split += beam_splits_current_line;
            current_beams_positions.extend(line.start_beams_positions());
            previous_beams_positions = current_beams_positions;
        }

        total_beam_split
    }

    fn count_number_of_timelines(&self) -> u64 {
        // If there is 0 split, there is 1 timeline, if there is 1 split,
        // there is 2 timelines, etc.
        self.count_number_of_beam_split() + 1
    }
}

fn main() {
    let manifold = read_input("res/day_07_input.txt");
    let number_of_different_timelines = manifold.count_number_of_timelines();
    println!("There is {number_of_different_timelines} timelines.");
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
        assert_eq!(
            ManifoldElement::from('S'),
            ManifoldElement::TachyonBeamStart
        );
        assert_eq!(ManifoldElement::from('^'), ManifoldElement::Splitter);
    }

    #[test]
    fn expand_beams_of_previous() {
        let line = ManifoldLine {
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
        assert_eq!(
            line.expand_beams_of_previous([(0, 1), (2, 1), (4, 1), (5, 1), (6, 1)]),
            (vec![(0, 1), (2, 1), (5, 3), (7, 1)], 2)
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
                            ManifoldElement::TachyonBeamStart,
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
    fn count_number_of_timelines_on_example() {
        let example = read_input_from_content(
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
        assert_eq!(example.count_number_of_timelines(), 40);
    }
}
