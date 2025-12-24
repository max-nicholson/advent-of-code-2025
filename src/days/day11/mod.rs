use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse(input: &str) -> anyhow::Result<HashMap<&str, Vec<&str>>> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").ok_or(anyhow::format_err!(
                "expected ': ' separated input and outputs"
            ))?;

            Ok((parts.0, parts.1.split_whitespace().collect()))
        })
        .collect()
}

fn paths<'a>(
    cache: &mut HashMap<(&'a str, &'a str), usize>,
    devices: &HashMap<&'a str, Vec<&'a str>>,
    from: &'a str,
    to: &'a str,
) -> usize {
    if let Some(total) = cache.get(&(from, to)) {
        return *total;
    }

    if from == to {
        let total = 1;

        cache.insert((from, to), total);

        return total;
    }

    match devices.get(from) {
        Some(outputs) => {
            let mut total = 0;
            for output in outputs {
                total += paths(cache, devices, output, to);
            }

            cache.insert((from, to), total);

            total
        }
        None => {
            cache.insert((from, to), 0);
            0
        }
    }
}

fn paths_with_required_waypoints<'a>(
    cache: &mut HashMap<(&'a str, &'a str), usize>,
    devices: &'a HashMap<&str, Vec<&str>>,
    from: &'a str,
    to: &'a str,
    required_waypoints: HashSet<&'a str>,
) -> usize {
    let total_waypoints = required_waypoints.len();

    let waypoint_permutations = required_waypoints.iter().permutations(total_waypoints);

    let mut total = 0;

    for waypoints in waypoint_permutations {
        let mut src = from;
        let mut subtotal = 1;

        for dst in waypoints.into_iter().chain([&to]) {
            subtotal *= paths(cache, devices, src, dst);
            src = dst;

            if subtotal == 0 {
                break;
            }

            if *dst == to {
                total += subtotal
            }
        }
    }

    total
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let devices = parse(&input)?;

    let mut cache = HashMap::new();

    println!("part1: {}", paths(&mut cache, &devices, "you", "out"));
    println!(
        "part2: {}",
        paths_with_required_waypoints(
            &mut cache,
            &devices,
            "svr",
            "out",
            HashSet::from(["dac", "fft"])
        )
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths() {
        let devices = parse(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        )
        .unwrap();

        assert_eq!(paths(&mut HashMap::new(), &devices, "you", "out"), 5);
    }

    #[test]
    fn test_paths_with_required_waypoints() {
        let devices = parse(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        )
        .unwrap();

        assert_eq!(
            paths_with_required_waypoints(
                &mut HashMap::new(),
                &devices,
                "svr",
                "out",
                HashSet::from(["dac", "fft"])
            ),
            2
        );
    }
}
