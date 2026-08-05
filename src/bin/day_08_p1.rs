use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

type Position = i64;
type Distance = f64;

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
        Position3d { x, y, z }
    }

    fn euclidean_distance_to(&self, other: &Self) -> Distance {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        ((dx.pow(2) + dy.pow(2) + dz.pow(2)) as Distance).sqrt()
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
    pairs.sort_by(|a, b| {
        a.0.euclidean_distance_to(a.1)
            .partial_cmp(&b.0.euclidean_distance_to(b.1))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    pairs
}

fn circuits_after_connecting_n_pairs<'a>(
    junction_boxes: &'a JunctionBoxes,
    n: usize,
) -> Circuits<'a> {
    let junction_box_pairs_by_distance = all_junction_box_pairs_sorted_by_distance(junction_boxes);
    let mut circuits: Circuits = junction_boxes
        .iter()
        .map(|junction_box| HashSet::from([junction_box]))
        .collect();

    for pair in junction_box_pairs_by_distance.iter().take(n) {
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
    }
    circuits
}

fn product_of_sizes_of_three_largest_circuits(circuits: &Circuits) -> usize {
    if circuits.len() < 3 {
        panic!("can't do the product of the three largest circuits with less than 3 circuits");
    }
    let mut circuits_lenghs: Vec<usize> = circuits.iter().map(Circuit::len).collect();
    circuits_lenghs.sort_unstable();
    circuits_lenghs.reverse();
    circuits_lenghs[0] * circuits_lenghs[1] * circuits_lenghs[2]
}

fn main() {
    let junction_boxes = read_input("res/day_08_input.txt");
    let circuits = circuits_after_connecting_n_pairs(&junction_boxes, 1000);
    let product = product_of_sizes_of_three_largest_circuits(&circuits);
    println!("The product of the sizes of the three largest circuits is {product}");
}

fn read_input(path: &str) -> JunctionBoxes {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    read_input_from_content(&input_file_content)
}

fn read_input_from_content(content: &str) -> JunctionBoxes {
    content
        .lines()
        .map(|line| {
            let mut positions = line.split(",");
            Position3d::new(
                positions
                    .next()
                    .expect("position with missing x in text input")
                    .parse()
                    .expect("bad value for x"),
                positions
                    .next()
                    .expect("position with missing y in text input")
                    .parse()
                    .expect("bad value for y"),
                positions
                    .next()
                    .expect("position with missing z in text input")
                    .parse()
                    .expect("bad value for z"),
            )
        })
        .collect()
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
    fn circuits_after_connecting_0_pairs_on_example() {
        let example = get_example();
        assert_eq!(
            circuits_after_connecting_n_pairs(&example, 0),
            vec![
                HashSet::from([&example[0]]),
                HashSet::from([&example[1]]),
                HashSet::from([&example[2]]),
                HashSet::from([&example[3]]),
                HashSet::from([&example[4]]),
                HashSet::from([&example[5]]),
                HashSet::from([&example[6]]),
                HashSet::from([&example[7]]),
                HashSet::from([&example[8]]),
                HashSet::from([&example[9]]),
                HashSet::from([&example[10]]),
                HashSet::from([&example[11]]),
                HashSet::from([&example[12]]),
                HashSet::from([&example[13]]),
                HashSet::from([&example[14]]),
                HashSet::from([&example[15]]),
                HashSet::from([&example[16]]),
                HashSet::from([&example[17]]),
                HashSet::from([&example[18]]),
                HashSet::from([&example[19]]),
            ]
        );
    }

    #[test]
    fn circuits_after_connecting_1_pair_on_example() {
        let example = get_example();
        assert_eq!(
            circuits_after_connecting_n_pairs(&example, 1),
            vec![
                HashSet::from([&example[0], &example[19]]),
                HashSet::from([&example[1]]),
                HashSet::from([&example[2]]),
                HashSet::from([&example[3]]),
                HashSet::from([&example[4]]),
                HashSet::from([&example[5]]),
                HashSet::from([&example[6]]),
                HashSet::from([&example[7]]),
                HashSet::from([&example[8]]),
                HashSet::from([&example[9]]),
                HashSet::from([&example[10]]),
                HashSet::from([&example[11]]),
                HashSet::from([&example[12]]),
                HashSet::from([&example[13]]),
                HashSet::from([&example[14]]),
                HashSet::from([&example[15]]),
                HashSet::from([&example[16]]),
                HashSet::from([&example[17]]),
                HashSet::from([&example[18]]),
            ]
        );
    }

    #[test]
    fn circuits_after_connecting_2_pairs_on_example() {
        let example = get_example();
        assert_eq!(
            circuits_after_connecting_n_pairs(&example, 2),
            vec![
                HashSet::from([&example[0], &example[19], &example[7]]),
                HashSet::from([&example[1]]),
                HashSet::from([&example[2]]),
                HashSet::from([&example[3]]),
                HashSet::from([&example[4]]),
                HashSet::from([&example[5]]),
                HashSet::from([&example[6]]),
                HashSet::from([&example[8]]),
                HashSet::from([&example[9]]),
                HashSet::from([&example[10]]),
                HashSet::from([&example[11]]),
                HashSet::from([&example[12]]),
                HashSet::from([&example[13]]),
                HashSet::from([&example[14]]),
                HashSet::from([&example[15]]),
                HashSet::from([&example[16]]),
                HashSet::from([&example[17]]),
                HashSet::from([&example[18]]),
            ]
        );
    }

    #[test]
    fn circuits_after_connecting_3_pairs_on_example() {
        let example = get_example();
        assert_eq!(
            circuits_after_connecting_n_pairs(&example, 3),
            vec![
                HashSet::from([&example[0], &example[19], &example[7]]),
                HashSet::from([&example[1]]),
                HashSet::from([&example[2], &example[13]]),
                HashSet::from([&example[3]]),
                HashSet::from([&example[4]]),
                HashSet::from([&example[5]]),
                HashSet::from([&example[6]]),
                HashSet::from([&example[8]]),
                HashSet::from([&example[9]]),
                HashSet::from([&example[10]]),
                HashSet::from([&example[11]]),
                HashSet::from([&example[12]]),
                HashSet::from([&example[14]]),
                HashSet::from([&example[15]]),
                HashSet::from([&example[16]]),
                HashSet::from([&example[17]]),
                HashSet::from([&example[18]]),
            ]
        );
    }

    #[test]
    fn circuits_after_connecting_4_pairs_on_example() {
        let example = get_example();
        assert_eq!(
            circuits_after_connecting_n_pairs(&example, 4),
            vec![
                HashSet::from([&example[0], &example[19], &example[7]]),
                HashSet::from([&example[1]]),
                HashSet::from([&example[2], &example[13]]),
                HashSet::from([&example[3]]),
                HashSet::from([&example[4]]),
                HashSet::from([&example[5]]),
                HashSet::from([&example[6]]),
                HashSet::from([&example[8]]),
                HashSet::from([&example[9]]),
                HashSet::from([&example[10]]),
                HashSet::from([&example[11]]),
                HashSet::from([&example[12]]),
                HashSet::from([&example[14]]),
                HashSet::from([&example[15]]),
                HashSet::from([&example[16]]),
                HashSet::from([&example[17]]),
                HashSet::from([&example[18]]),
            ]
        );
    }

    #[test]
    fn product_of_sizes_of_three_largest_circuits_on_example_after_10_connections() {
        let example = get_example();
        let circuits = circuits_after_connecting_n_pairs(&example, 10);
        assert_eq!(product_of_sizes_of_three_largest_circuits(&circuits), 40);
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
