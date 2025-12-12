use std::collections::{HashMap, HashSet, hash_map::Entry};

fn parse(input: String) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn find_beam_entrypoint(manifold: &[Vec<u8>]) -> anyhow::Result<usize> {
    let column = manifold[0]
        .iter()
        .position(|&x| x == b'S')
        .ok_or(anyhow::format_err!(
            "unable to find entrypoint of tachyon beam"
        ))?;

    Ok(column)
}

fn beam_splits(manifold: &[Vec<u8>]) -> anyhow::Result<usize> {
    let entrypoint = find_beam_entrypoint(manifold)?;

    let mut beams = HashSet::from([entrypoint]);

    let mut splits = 0;

    for row in manifold.iter().skip(1) {
        let mut next = beams.clone();

        for &beam in beams.iter() {
            if row[beam] == b'^' {
                splits += 1;
                if beam != 0 {
                    next.insert(beam - 1);
                }
                if beam != row.len() - 1 {
                    next.insert(beam + 1);
                }

                next.remove(&beam);
            }
        }

        beams = next;
    }

    Ok(splits)
}

fn quantum_tachyon_timelines(manifold: &[Vec<u8>]) -> anyhow::Result<usize> {
    // let entrypoint = find_beam_entrypoint(manifold)?;

    // let mut stack = vec![(0usize, entrypoint)];
    // let mut timelines = 0;

    // let rows = manifold.len() - 1;

    // let mut split = 0;

    // while let Some((row, column)) = stack.pop() {
    //     if manifold[row][column] == b'^' {
    //         split += 1;
    //         stack.push((row + 1, column - 1));
    //         stack.push((row + 1, column + 1));
    //     } else if row >= rows {
    //         timelines += 1;
    //     } else {
    //         stack.push((row + 1, column));
    //     }
    // }

    // println!("{}", split);

    // Ok(timelines)
    let entrypoint = find_beam_entrypoint(manifold)?;

    let mut beams = HashMap::from([(entrypoint, 1usize)]);

    for row in manifold.iter().skip(1) {
        let mut next = beams.clone();

        for (&beam, timelines) in beams.iter() {
            if row[beam] == b'^' {
                *next.entry(beam + 1).or_insert(0) += timelines;
                *next.entry(beam - 1).or_insert(0) += timelines;

                if let Entry::Occupied(mut o) = next.entry(beam) {
                    if *o.get() == *timelines {
                        o.remove();
                    } else {
                        *o.get_mut() -= timelines;
                    }
                };
            }
        }

        beams = next;
    }

    Ok(beams.values().sum())
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let manifold = parse(input);

    println!("part1: {}", beam_splits(&manifold)?);

    println!("part2: {}", quantum_tachyon_timelines(&manifold)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_splits() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .to_string();

        let manifold = parse(input);

        assert_eq!(beam_splits(&manifold).unwrap(), 21);
    }

    #[test]
    fn test_quantum_tachyon_timelines() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .to_string();

        let manifold = parse(input);

        assert_eq!(quantum_tachyon_timelines(&manifold).unwrap(), 40);
    }
}
