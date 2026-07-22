use core::panic;
use std::fs;

type Position = i32;
type Distance = f64; // f64 to use ::from<Position>

#[derive(Debug, PartialEq)]
struct Position3d {
    x: Position,
    y: Position,
    z: Position,
}

impl Position3d {
    fn new(x: Position, y: Position, z: Position) -> Self {
        Position3d { x, y, z }
    }

    fn euclidean_distance_to(&self, other: &Self) -> Distance {
        Distance::from(
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2),
        )
        .sqrt()
    }
}

#[derive(Debug, PartialEq)]
struct Circuit {
    junction_boxes: Vec<Position3d>,
}

impl Circuit {
    fn new(junction_boxes: impl IntoIterator<Item = Position3d>) -> Self {
        Self {
            junction_boxes: junction_boxes.into_iter().collect(),
        }
    }

    fn euclidean_distance_to(&self, other: &Self) -> Distance {
        self.junction_boxes
            .iter()
            .map(|self_junction_box| {
                other
                    .junction_boxes
                    .iter()
                    .map(|other_junction_box| {
                        self_junction_box.euclidean_distance_to(other_junction_box)
                    })
                    .reduce(Distance::min)
                    .expect("failed to compare distance between two circuits as other is empty")
            })
            .reduce(Distance::min)
            .expect("failed to compare distance between two circuits as self is empty")
    }

    fn combine_with(mut self, mut other: Self) -> Self {
        self.junction_boxes.append(&mut other.junction_boxes);
        Self::new(self.junction_boxes)
    }
}

#[derive(Debug, PartialEq)]
struct Circuits {
    circuits: Vec<Circuit>,
}

impl Circuits {
    fn new(circuits: impl IntoIterator<Item = Circuit>) -> Self {
        Self {
            circuits: circuits.into_iter().collect(),
        }
    }

    fn connect_two_closest_circuits(&mut self) {
        let indexes_of_two_closest_circuits = self.indexes_of_two_closest_circuits();
        let closest_circuits = (
            self.circuits.remove(indexes_of_two_closest_circuits.0),
            // -1 as one was already removed
            self.circuits.remove(indexes_of_two_closest_circuits.1 - 1),
        );
        self.circuits
            .push((closest_circuits.0).combine_with(closest_circuits.1));
    }

    fn indexes_of_two_closest_circuits(&self) -> (usize, usize) {
        if self.circuits.len() < 2 {
            panic!("can't find indexes of two closest circuits with less than 2 circuits");
        }
        let mut indexes_of_currently_two_closest_circuits = (0, 1);
        let mut current_shortest_distance = self.circuits
            [indexes_of_currently_two_closest_circuits.0]
            .euclidean_distance_to(&self.circuits[indexes_of_currently_two_closest_circuits.1]);

        for (first_index, first_circuit) in self.circuits.iter().enumerate() {
            for (second_index, second_circuit) in self.circuits.iter().enumerate() {
                let distance = first_circuit.euclidean_distance_to(second_circuit);
                if first_index != second_index && distance < current_shortest_distance {
                    indexes_of_currently_two_closest_circuits = (first_index, second_index);
                    current_shortest_distance = distance;
                }
            }
        }
        indexes_of_currently_two_closest_circuits
    }

    fn product_of_sizes_of_three_largest_circuits(&self) -> usize {
        let mut circuit_lens = self
            .circuits
            .iter()
            .map(|circuit| circuit.junction_boxes.len())
            .collect::<Vec<usize>>();
        circuit_lens.sort();
        circuit_lens.reverse();
        if circuit_lens.len() < 3 {
            panic!("can't do the product of the three largest circuits with less than 3 circuits");
        }
        circuit_lens[0] * circuit_lens[1] * circuit_lens[2]
    }
}

fn main() {
    let mut circuits = read_input("res/day_08_input.txt");
    for _ in 0..1000 {
        circuits.connect_two_closest_circuits();
    }
    let product = circuits.product_of_sizes_of_three_largest_circuits();
    println!("The product of the sizes of the three largest circuits is {product}");
}

fn read_input(path: &str) -> Circuits {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    read_input_from_content(&input_file_content)
}

fn read_input_from_content(content: &str) -> Circuits {
    Circuits::new(
        content
            .lines()
            .map(|line| {
                line.split(',').map(|position| {
                    position
                        .parse()
                        .expect("failed to parse a position (as str) to a number")
                })
            })
            .map(|mut positions| {
                Circuit::new(vec![Position3d::new(
                    positions
                        .next()
                        .expect("position with missing x in text input"),
                    positions
                        .next()
                        .expect("position with missing y in text input"),
                    positions
                        .next()
                        .expect("position with missing z in text input"),
                )])
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_3d_euclidean_distance_between_0_and_0() {
        assert_eq!(
            Position3d::new(0, 0, 0).euclidean_distance_to(&Position3d::new(0, 0, 0)),
            0_f64
        )
    }
    #[test]
    fn position_3d_euclidean_distance_between_two_positive_points() {
        assert_eq!(
            Position3d::new(42, 1, 842).euclidean_distance_to(&Position3d::new(403, 61, 95)),
            831.823298543627
        )
    }

    #[test]
    fn position_3d_euclidean_distance_between_mixed_signs_points() {
        assert_eq!(
            Position3d::new(-43, 8413, -3013).euclidean_distance_to(&Position3d::new(-4991, -9, 0)),
            10222.081832973165
        )
    }

    #[test]
    fn read_input_from_content_on_example() {
        let example_read = read_input_from_content(
            &[
                "162,817,812",
                "57,618,57",
                "906,360,560",
                "592,479,940",
                "352,342,300",
                "466,668,158",
                "542,29,236",
                "431,825,988",
                "739,650,466",
                "52,470,668",
                "216,146,977",
                "819,987,18",
                "117,168,530",
                "805,96,715",
                "346,949,466",
                "970,615,88",
                "941,993,340",
                "862,61,35",
                "984,92,344",
                "425,690,689",
            ]
            .join("\n"),
        );
        assert_eq!(example_read, get_example())
    }

    #[test]
    fn indexes_of_two_closest_circuits_on_example() {
        assert_eq!(get_example().indexes_of_two_closest_circuits(), (0, 19));
    }

    #[test]
    fn connect_two_closest_circuits_on_example() {
        let mut example = get_example();
        example.connect_two_closest_circuits();
        assert_eq!(
            example,
            Circuits::new(vec![
                Circuit::new(vec![Position3d::new(57, 618, 57)]),
                Circuit::new(vec![Position3d::new(906, 360, 560)]),
                Circuit::new(vec![Position3d::new(592, 479, 940)]),
                Circuit::new(vec![Position3d::new(352, 342, 300)]),
                Circuit::new(vec![Position3d::new(466, 668, 158)]),
                Circuit::new(vec![Position3d::new(542, 29, 236)]),
                Circuit::new(vec![Position3d::new(431, 825, 988)]),
                Circuit::new(vec![Position3d::new(739, 650, 466)]),
                Circuit::new(vec![Position3d::new(52, 470, 668)]),
                Circuit::new(vec![Position3d::new(216, 146, 977)]),
                Circuit::new(vec![Position3d::new(819, 987, 18)]),
                Circuit::new(vec![Position3d::new(117, 168, 530)]),
                Circuit::new(vec![Position3d::new(805, 96, 715)]),
                Circuit::new(vec![Position3d::new(346, 949, 466)]),
                Circuit::new(vec![Position3d::new(970, 615, 88)]),
                Circuit::new(vec![Position3d::new(941, 993, 340)]),
                Circuit::new(vec![Position3d::new(862, 61, 35)]),
                Circuit::new(vec![Position3d::new(984, 92, 344)]),
                Circuit::new(vec![
                    Position3d::new(162, 817, 812),
                    Position3d::new(425, 690, 689),
                ]),
            ])
        );

        example.connect_two_closest_circuits();
        assert_eq!(
            example,
            Circuits::new(vec![
                Circuit::new(vec![Position3d::new(57, 618, 57)]),
                Circuit::new(vec![Position3d::new(906, 360, 560)]),
                Circuit::new(vec![Position3d::new(592, 479, 940)]),
                Circuit::new(vec![Position3d::new(352, 342, 300)]),
                Circuit::new(vec![Position3d::new(466, 668, 158)]),
                Circuit::new(vec![Position3d::new(542, 29, 236)]),
                Circuit::new(vec![Position3d::new(739, 650, 466)]),
                Circuit::new(vec![Position3d::new(52, 470, 668)]),
                Circuit::new(vec![Position3d::new(216, 146, 977)]),
                Circuit::new(vec![Position3d::new(819, 987, 18)]),
                Circuit::new(vec![Position3d::new(117, 168, 530)]),
                Circuit::new(vec![Position3d::new(805, 96, 715)]),
                Circuit::new(vec![Position3d::new(346, 949, 466)]),
                Circuit::new(vec![Position3d::new(970, 615, 88)]),
                Circuit::new(vec![Position3d::new(941, 993, 340)]),
                Circuit::new(vec![Position3d::new(862, 61, 35)]),
                Circuit::new(vec![Position3d::new(984, 92, 344)]),
                Circuit::new(vec![
                    Position3d::new(431, 825, 988),
                    Position3d::new(162, 817, 812),
                    Position3d::new(425, 690, 689),
                ]),
            ])
        );
    }

    #[test]
    fn product_of_sizes_of_three_largest_circuits_after_2_connections_on_example() {
        let mut example = get_example();
        for _ in 0..2 {
            example.connect_two_closest_circuits();
        }
        assert_eq!(example.product_of_sizes_of_three_largest_circuits(), 3);
    }

    #[test]
    fn product_of_sizes_of_three_largest_circuits_after_3_connections_on_example() {
        let mut example = get_example();
        for _ in 0..3 {
            example.connect_two_closest_circuits();
        }
        assert_eq!(example.product_of_sizes_of_three_largest_circuits(), 6);
    }

    #[test]
    fn product_of_sizes_of_three_largest_circuits_after_10_connections_on_example() {
        let mut example = get_example();
        for _ in 0..10 {
            example.connect_two_closest_circuits();
        }
        dbg!(&example);
        assert_eq!(example.product_of_sizes_of_three_largest_circuits(), 40);
    }

    fn get_example() -> Circuits {
        Circuits::new(vec![
            Circuit::new(vec![Position3d::new(162, 817, 812)]),
            Circuit::new(vec![Position3d::new(57, 618, 57)]),
            Circuit::new(vec![Position3d::new(906, 360, 560)]),
            Circuit::new(vec![Position3d::new(592, 479, 940)]),
            Circuit::new(vec![Position3d::new(352, 342, 300)]),
            Circuit::new(vec![Position3d::new(466, 668, 158)]),
            Circuit::new(vec![Position3d::new(542, 29, 236)]),
            Circuit::new(vec![Position3d::new(431, 825, 988)]),
            Circuit::new(vec![Position3d::new(739, 650, 466)]),
            Circuit::new(vec![Position3d::new(52, 470, 668)]),
            Circuit::new(vec![Position3d::new(216, 146, 977)]),
            Circuit::new(vec![Position3d::new(819, 987, 18)]),
            Circuit::new(vec![Position3d::new(117, 168, 530)]),
            Circuit::new(vec![Position3d::new(805, 96, 715)]),
            Circuit::new(vec![Position3d::new(346, 949, 466)]),
            Circuit::new(vec![Position3d::new(970, 615, 88)]),
            Circuit::new(vec![Position3d::new(941, 993, 340)]),
            Circuit::new(vec![Position3d::new(862, 61, 35)]),
            Circuit::new(vec![Position3d::new(984, 92, 344)]),
            Circuit::new(vec![Position3d::new(425, 690, 689)]),
        ])
    }
}
