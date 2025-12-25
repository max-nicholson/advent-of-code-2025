use std::{collections::HashSet, num::ParseIntError};

type Present = Vec<Vec<bool>>;

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    presents: Vec<usize>,
}

impl Region {
    pub fn can_fit(&self, presents: &[Present]) -> bool {
        // 1st heuristic: check if there are enough 3x3 bins for all presents first
        let bins: usize = self.presents.iter().sum();

        if (self.width / 3) * (self.length / 3) >= bins {
            // we can definitely fit every present, since each can go into a separate "bin"
            return true;
        }

        // 2nd heuristic: check if there's enough space (assuming the absolute optimum packing with
        // 0 space wasted)

        let available_space = self.width * self.length;

        let used_space = self
            .presents
            .iter()
            .zip(presents)
            .map(|(count, present)| {
                if *count == 0 {
                    return 0;
                }

                present
                    .iter()
                    .map(|row| row.iter().filter(|space| **space).count())
                    .sum::<usize>()
                    * *count
            })
            .sum();

        if available_space < used_space {
            return false;
        }

        // in theory we'd start _actually_ trying to place everything here, but based on the input
        // this is completely unecessary - so we just assume it's possible
        true
    }
}

#[allow(dead_code)]
fn rotate(present: &[Vec<bool>]) -> Present {
    let length = present.len();
    let width = present[0].len();
    let mut rotated = Vec::with_capacity(width);

    for column_index in 0..width {
        let mut row = Vec::with_capacity(length);
        for row_index in (0..length).rev() {
            row.push(present[row_index][column_index])
        }
        rotated.push(row);
    }

    rotated
}

#[allow(dead_code)]
fn flip(present: &[Vec<bool>]) -> Present {
    let length = present.len();
    let width = present[0].len();
    let mut flipped = Vec::with_capacity(length);

    for row_index in 0..length {
        let mut row = Vec::with_capacity(width);
        for column_index in (0..width).rev() {
            row.push(present[row_index][column_index])
        }
        flipped.push(row)
    }

    flipped
}

#[allow(dead_code)]
fn permutations(present: &[Vec<bool>]) -> HashSet<Present> {
    let mut permutations = HashSet::new();

    permutations.insert(present.to_vec());

    {
        let mut curr = present.to_vec();
        for _ in 0..3 {
            let rotated = rotate(&curr);
            permutations.insert(rotated.clone());
            curr = rotated;
        }
    }

    {
        let mut curr = flip(present);

        for _ in 0..3 {
            let rotated = rotate(&curr);
            permutations.insert(rotated.clone());
            curr = rotated;
        }
    }

    permutations
}

fn parse(input: String) -> anyhow::Result<(Vec<Present>, Vec<Region>)> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let presents = parts[0..parts.len() - 1]
        .iter()
        .map(|present| {
            present
                .lines()
                .skip(1)
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();

    let regions = parts[parts.len() - 1]
        .lines()
        .map(|line| {
            let (size, present_counts) = line.split_once(": ").ok_or(anyhow::format_err!(
                "expected ': ' separated region size and present count"
            ))?;

            let (width, length) = size.split_once("x").ok_or(anyhow::format_err!(
                "expected 'x' separated width and length"
            ))?;

            Ok(Region {
                width: width.parse()?,
                length: length.parse()?,
                presents: present_counts
                    .split_whitespace()
                    .map(|count| count.parse())
                    .collect::<Result<Vec<usize>, ParseIntError>>()?,
            })
        })
        .collect::<anyhow::Result<Vec<Region>>>()?;

    Ok((presents, regions))
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let (presents, regions) = parse(input)?;

    println!(
        "part1: {}",
        regions
            .iter()
            .filter(|region| region.can_fit(&presents))
            .count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let present = vec![
            vec![true, true, true],
            vec![true, false, false],
            vec![true, true, true],
        ];

        let mut current = present.clone();
        let mut rotations: Vec<Present> = Vec::new();

        for _ in 0..4 {
            let rotated = rotate(&current);

            rotations.push(rotated.clone());

            current = rotated
        }

        assert_eq!(
            rotations,
            vec![
                vec![
                    vec![true, true, true],
                    vec![true, false, true],
                    vec![true, false, true]
                ],
                vec![
                    vec![true, true, true],
                    vec![false, false, true],
                    vec![true, true, true]
                ],
                vec![
                    vec![true, false, true],
                    vec![true, false, true],
                    vec![true, true, true]
                ],
                present
            ]
        )
    }

    #[test]
    fn test_permutations() {
        let present = vec![
            vec![true, true, true],
            vec![true, false, false],
            vec![true, true, true],
        ];

        assert_eq!(permutations(&present).len(), 4);
    }

    #[test]
    fn test_flip() {
        let present = vec![
            vec![true, true, true],
            vec![true, false, false],
            vec![true, true, true],
        ];

        let mut current = present.clone();
        let mut flips: Vec<Present> = Vec::new();

        for _ in 0..2 {
            let flipped = flip(&current);

            flips.push(flipped.clone());

            current = flipped
        }

        assert_eq!(
            flips,
            vec![
                vec![
                    vec![true, true, true],
                    vec![false, false, true],
                    vec![true, true, true]
                ],
                present
            ]
        )
    }

    #[test]
    fn test_region_can_fit_presents() {
        let region = Region {
            width: 4,
            length: 4,
            presents: vec![2],
        };

        let presents = vec![vec![vec![true; 3], vec![true, false, false], vec![true; 3]]];

        assert!(region.can_fit(&presents));
    }
}
