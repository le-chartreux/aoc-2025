use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

type Position = i64;
type Distance = i64;

type JunctionBox = Position3d;
type JunctionBoxes = Vec<JunctionBox>;
type JunctionBoxPair<'a> = (&'a JunctionBox, &'a JunctionBox);
/// All possible connections between junction boxes, ordered by distance (closest first).
type JunctionBoxPairsByDistance<'a> = Vec<JunctionBoxPair<'a>>;

type Circuit<'a> = HashSet<&'a JunctionBox>;
type Circuits<'a> = Vec<Circuit<'a>>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position3d {
    x: Position,
    y: Position,
    z: Position,
}

impl Position3d {
    fn new(x: Position, y: Position, z: Position) -> Self {
        Self { x, y, z }
    }

    /// Squared to not bother with floats.
    fn squared_euclidean_distance_to(&self, other: &Self) -> Distance {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}

/// Get all the pairs of junction boxes, ordered by distance (closest first).
/// Each pair only appears one time, so A and B will produce either (A, B) or (B, A), not both.
fn all_junction_box_pairs_sorted_by_distance<'a>(
    junction_boxes: &'a JunctionBoxes,
) -> JunctionBoxPairsByDistance<'a> {
    let mut pairs = Vec::with_capacity(junction_boxes.len() * (junction_boxes.len() - 1) / 2);
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            pairs.push((&junction_boxes[i], &junction_boxes[j]));
        }
    }
    pairs.sort_by_key(|(a, b)| a.squared_euclidean_distance_to(b));
    pairs
}

fn junction_box_pair_whose_connection_reduces_circuits_to_one(
    junction_boxes: &JunctionBoxes,
) -> JunctionBoxPair<'_> {
    let junction_box_pairs_by_distance = all_junction_box_pairs_sorted_by_distance(junction_boxes);
    let mut circuits: Circuits = junction_boxes
        .iter()
        .map(|junction_box| HashSet::from([junction_box]))
        .collect();

    let mut i = 0;
    while circuits.len() != 1 && i < junction_box_pairs_by_distance.len() {
        let pair = junction_box_pairs_by_distance[i];
        let index_of_circuit_of_pair_0 = circuits
            .iter()
            .position(|circuit| circuit.contains(pair.0))
            .expect("pair.0 not found in circuits");
        let index_of_circuit_of_pair_1 = circuits
            .iter()
            .position(|circuit| circuit.contains(pair.1))
            .expect("pair.1 not found in circuits");

        if index_of_circuit_of_pair_0 != index_of_circuit_of_pair_1 {
            let index_to_keep = min(index_of_circuit_of_pair_0, index_of_circuit_of_pair_1);
            let index_to_remove = max(index_of_circuit_of_pair_0, index_of_circuit_of_pair_1);
            let content_to_merge = circuits.remove(index_to_remove);
            circuits[index_to_keep].extend(content_to_merge);
        }
        i += 1;
    }
    if i >= junction_box_pairs_by_distance.len() {
        panic!("connection of all junction box pairs did not result in one circuit");
    }
    junction_box_pairs_by_distance[i - 1] // -1 as i+=1 all the time in the loop.
}

fn main() {
    let junction_boxes = read_input("res/day_08_input.txt");
    let pair = junction_box_pair_whose_connection_reduces_circuits_to_one(&junction_boxes);
    println!("The last pair to connect for circuits to be all connected is {pair:?}.");
    println!(
        "The product of their X coordinates is {}.",
        pair.0.x * pair.1.x
    );
}

fn read_input(path: &str) -> JunctionBoxes {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    read_input_from_content(&input_file_content)
}

fn read_input_from_content(content: &str) -> JunctionBoxes {
    content
        .lines()
        .map(|line| {
            let positions: Vec<Position> = line
                .split(",")
                .map(|position| position.parse().expect("invalid position in input"))
                .collect();
            assert_eq!(
                positions.len(),
                3,
                "invalid number of positions in an input line"
            );
            Position3d::new(positions[0], positions[1], positions[2])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_3d_euclidean_distance_between_0_and_0() {
        assert_eq!(
            Position3d::new(0, 0, 0).squared_euclidean_distance_to(&Position3d::new(0, 0, 0)),
            0
        )
    }
    #[test]
    fn position_3d_euclidean_distance_between_two_positive_points() {
        assert_eq!(
            Position3d::new(42, 1, 842)
                .squared_euclidean_distance_to(&Position3d::new(403, 61, 95)),
            691930
        )
    }

    #[test]
    fn position_3d_euclidean_distance_between_mixed_signs_points() {
        assert_eq!(
            Position3d::new(-43, 8413, -3013)
                .squared_euclidean_distance_to(&Position3d::new(-4991, -9, 0)),
            104490957
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
    fn all_junction_box_pairs_sorted_by_distance_on_4_first_lines_of_example() {
        let junction_boxes = vec![
            Position3d::new(162, 817, 812),
            Position3d::new(57, 618, 57),
            Position3d::new(906, 360, 560),
            Position3d::new(592, 479, 940),
        ];
        assert_eq!(
            all_junction_box_pairs_sorted_by_distance(&junction_boxes),
            vec![
                (&junction_boxes[2], &junction_boxes[3]), // 507.106498
                (&junction_boxes[0], &junction_boxes[3]), // 561.718791
                (&junction_boxes[0], &junction_boxes[1]), // 787.814064
                (&junction_boxes[0], &junction_boxes[2]), // 908.784353
                (&junction_boxes[1], &junction_boxes[2]), // 1005.322834
                (&junction_boxes[1], &junction_boxes[3]), // 1041.74613
            ]
        )
    }

    #[test]
    fn junction_box_pair_whose_connection_reduces_circuits_to_one_on_example() {
        let example = get_example();
        assert_eq!(
            junction_box_pair_whose_connection_reduces_circuits_to_one(&example),
            (
                &Position3d::new(216, 146, 977),
                &Position3d::new(117, 168, 530)
            )
        );
    }

    fn get_example() -> JunctionBoxes {
        vec![
            Position3d::new(162, 817, 812),
            Position3d::new(57, 618, 57),
            Position3d::new(906, 360, 560),
            Position3d::new(592, 479, 940),
            Position3d::new(352, 342, 300),
            Position3d::new(466, 668, 158),
            Position3d::new(542, 29, 236),
            Position3d::new(431, 825, 988),
            Position3d::new(739, 650, 466),
            Position3d::new(52, 470, 668),
            Position3d::new(216, 146, 977),
            Position3d::new(819, 987, 18),
            Position3d::new(117, 168, 530),
            Position3d::new(805, 96, 715),
            Position3d::new(346, 949, 466),
            Position3d::new(970, 615, 88),
            Position3d::new(941, 993, 340),
            Position3d::new(862, 61, 35),
            Position3d::new(984, 92, 344),
            Position3d::new(425, 690, 689),
        ]
    }
}
