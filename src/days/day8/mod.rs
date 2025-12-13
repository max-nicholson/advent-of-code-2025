use std::{collections::HashSet, num::ParseIntError};

fn parse(input: String) -> anyhow::Result<Vec<Vec<usize>>> {
    input
        .lines()
        .map(|line| {
            let coordinates = line
                .split(",")
                .map(str::parse::<usize>)
                .collect::<Result<Vec<usize>, ParseIntError>>()?;

            debug_assert_eq!(coordinates.len(), 3);

            Ok(coordinates)
        })
        .collect()
}

fn compute_distances(junction_boxes: &[Vec<usize>]) -> Vec<(usize, usize, f64)> {
    let number_of_junctions = junction_boxes.len();

    let mut distances =
        vec![(0, 0, f64::MAX); (number_of_junctions) / 2 * (number_of_junctions - 1)];
    let mut index = 0;

    for i in 0..junction_boxes.len() {
        let a = &junction_boxes[i];

        for j in i + 1..junction_boxes.len() {
            let b = &junction_boxes[j];

            let distance = (((a[0].abs_diff(b[0])).pow(2)
                + (a[1].abs_diff(b[1])).pow(2)
                + (a[2].abs_diff(b[2])).pow(2)) as f64)
                .sqrt();

            distances[index] = (i, j, distance);
            index += 1;
        }
    }

    distances.sort_by(|(_, _, a), (_, _, b)| PartialOrd::partial_cmp(a, b).unwrap());

    distances
}

fn three_largest_circuits_after_n_connections(
    distances: &[(usize, usize, f64)],
    connections: usize,
) -> usize {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for (i, j, _) in distances.iter().take(connections) {
        let i_circuit = circuits.iter().position(|s| s.contains(i));
        let j_circuit = circuits.iter().position(|s| s.contains(j));

        match (i_circuit, j_circuit) {
            (Some(a), Some(b)) => {
                if a == b {
                    continue;
                } else {
                    let other_circuit = &circuits[b].clone();
                    circuits[a].extend(other_circuit);
                    circuits.swap_remove(b);
                }
            }
            (Some(a), None) => {
                circuits[a].insert(*j);
            }
            (None, Some(b)) => {
                circuits[b].insert(*i);
            }
            (None, None) => {
                circuits.push(HashSet::from([*i, *j]));
            }
        }
    }

    circuits.sort_by_cached_key(|circuit| circuit.len());
    circuits
        .iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.len())
        .product()
}

fn first_connection_to_form_single_circuit(
    junction_boxes: &[Vec<usize>],
    distances: &[(usize, usize, f64)],
) -> [(usize, usize, usize); 2] {
    let number_of_junctions = junction_boxes.len();

    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for (i, j, _) in distances.iter() {
        let i_circuit = circuits.iter().position(|s| s.contains(i));
        let j_circuit = circuits.iter().position(|s| s.contains(j));

        match (i_circuit, j_circuit) {
            (Some(a), Some(b)) => {
                if a == b {
                    continue;
                } else {
                    let other_circuit = &circuits[b].clone();
                    circuits[a].extend(other_circuit);
                    circuits.swap_remove(b);
                }
            }
            (Some(a), None) => {
                circuits[a].insert(*j);
            }
            (None, Some(b)) => {
                circuits[b].insert(*i);
            }
            (None, None) => {
                circuits.push(HashSet::from([*i, *j]));
            }
        }

        if circuits.len() == 1 && circuits[0].len() == number_of_junctions {
            let from = &junction_boxes[*i];
            let to = &junction_boxes[*j];
            return [(from[0], from[1], from[2]), (to[0], to[1], to[2])];
        }
    }

    unreachable!()
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let junction_boxes = parse(input)?;

    let distances = compute_distances(&junction_boxes);

    println!(
        "part1: {}",
        three_largest_circuits_after_n_connections(&distances, 1000)
    );

    let [a, b] = first_connection_to_form_single_circuit(&junction_boxes, &distances);

    println!("part2: {}", a.0 * b.0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_largest_circuits() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .to_string();

        assert_eq!(
            three_largest_circuits_after_n_connections(
                &compute_distances(&parse(input).unwrap()),
                10
            ),
            40
        );
    }

    #[test]
    fn test_first_connection_to_form_single_circuit() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .to_string();

        let junction_boxes = &parse(input).unwrap();

        let distances = &compute_distances(junction_boxes);

        assert_eq!(
            first_connection_to_form_single_circuit(junction_boxes, distances),
            [(216, 146, 977), (117, 168, 530)],
        );
    }
}
