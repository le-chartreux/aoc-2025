use std::collections::HashMap;
use std::fs;
use std::ops::{Deref, DerefMut};

type TimelineCount = u64;
type Position = usize;
type Beams = HashMap<Position, TimelineCount>;
type ManifoldLines = Vec<ManifoldLine>;

fn add_timeline_count_to_beams(
    position: Position,
    timeline_count: TimelineCount,
    beams: &mut Beams,
) {
    if let Some(existing_timeline_count) = beams.get_mut(&position) {
        *existing_timeline_count += timeline_count;
    } else {
        beams.insert(position, timeline_count);
    }
}

fn add_many_timeline_count_to_beams(
    positions: impl IntoIterator<Item = Position>,
    timeline_count: TimelineCount,
    beams: &mut Beams,
) {
    for position in positions {
        add_timeline_count_to_beams(position, timeline_count, beams);
    }
}

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
    /// Expand beams through this line, and return the new positions of beams.
    fn trace_beams(&self, beams_before_line: Beams) -> Beams {
        let mut beams_after_line = Beams::new();
        for (position, timeline_count) in beams_before_line {
            match self[position] {
                ManifoldElement::EmptySpace => {
                    add_timeline_count_to_beams(position, timeline_count, &mut beams_after_line);
                }
                ManifoldElement::Splitter => {
                    add_many_timeline_count_to_beams(
                        self.get_empty_existing_neighbors_positions(position),
                        timeline_count,
                        &mut beams_after_line,
                    );
                }
                ManifoldElement::TachyonBeamStart => {}
            }
        }
        beams_after_line
    }

    /// Get neighbors of the position that are empty and aren't out of bound.
    fn get_empty_existing_neighbors_positions(&self, position: Position) -> Vec<Position> {
        [position.checked_sub(1), Some(position + 1)]
            .iter()
            .flatten()
            .filter_map(|&neighbor_position| {
                (neighbor_position < self.len()
                    && matches!(self[neighbor_position], ManifoldElement::EmptySpace))
                .then_some(neighbor_position)
            })
            .collect()
    }

    fn start_beams_positions(&self) -> Vec<Position> {
        self.iter()
            .enumerate()
            .filter_map(|(position, element)| {
                (matches!(element, ManifoldElement::TachyonBeamStart)).then_some(position)
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Manifold {
    lines: ManifoldLines,
}

impl std::iter::FromIterator<ManifoldLine> for Manifold {
    fn from_iter<T: IntoIterator<Item = ManifoldLine>>(iter: T) -> Self {
        let lines: ManifoldLines = iter.into_iter().collect();
        let first_line_len = lines[0].len();
        assert!(
            lines.iter().all(|line| line.len() == first_line_len),
            "all lines of manifold must have the same len"
        );
        Self { lines }
    }
}

impl Manifold {
    fn count_number_of_timelines(&self) -> TimelineCount {
        let mut beams = Beams::new();
        for line in &self.lines {
            beams = line.trace_beams(beams);
            add_many_timeline_count_to_beams(line.start_beams_positions(), 1, &mut beams);
        }
        beams.values().sum()
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
            line.trace_beams(Beams::from([(0, 1), (2, 1), (4, 1), (5, 1), (6, 1),])),
            Beams::from([(0, 1), (2, 1), (5, 3), (7, 1)]),
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
