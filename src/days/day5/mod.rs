use std::{collections::HashSet, num::ParseIntError, ops::RangeInclusive};

fn parse_fresh_ingredient_ranges(input: &str) -> anyhow::Result<Vec<RangeInclusive<usize>>> {
    input
        .lines()
        .map(|line| {
            let range = line
                .split_once('-')
                .ok_or(anyhow::format_err!("expected - separated ingredient IDs"))?;

            let start: usize = range.0.parse()?;
            let end: usize = range.1.parse()?;

            Ok(start..=end)
        })
        .collect()
}

fn parse_available_ingredients(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.lines().map(str::parse::<usize>).collect()
}

fn parse(input: String) -> anyhow::Result<(Vec<RangeInclusive<usize>>, Vec<usize>)> {
    let parts = input.split_once("\n\n").ok_or(anyhow::format_err!(
        "expected fresh ingredient ID ranges separated by blank line, then available ingredient IDs"
    ))?;

    let fresh_ingredients = parse_fresh_ingredient_ranges(parts.0)?;

    let available_ingredients = parse_available_ingredients(parts.1)?;

    Ok((fresh_ingredients, available_ingredients))
}

fn fresh_available_ingredients(
    fresh_ingredient_ranges: &[RangeInclusive<usize>],
    available_ingredients: &[usize],
) -> usize {
    available_ingredients
        .iter()
        .filter(|ingredient_id| {
            fresh_ingredient_ranges
                .iter()
                .any(|fresh_ingredient_range| fresh_ingredient_range.contains(ingredient_id))
        })
        .count()
}

fn size(range: &RangeInclusive<usize>) -> usize {
    range.end() - range.start() + 1
}

fn is_contained_by(subset: &RangeInclusive<usize>, superset: &RangeInclusive<usize>) -> bool {
    superset.start() <= subset.start() && superset.end() >= subset.end()
}

fn total_fresh_ingredients(fresh_ingredient_ranges: &[RangeInclusive<usize>]) -> usize {
    let mut fresh_ingredient_ranges: Vec<RangeInclusive<usize>> = fresh_ingredient_ranges.to_vec();

    let mut total = 0;
    let mut skip: HashSet<usize> = HashSet::new();

    for i in 0..fresh_ingredient_ranges.len() {
        let range = fresh_ingredient_ranges[i].clone();

        if skip.contains(&i) {
            continue;
        }

        let mut subtotal = size(&range);

        for j in i + 1..fresh_ingredient_ranges.len() {
            if skip.contains(&j) {
                continue;
            }

            let other = &fresh_ingredient_ranges[j];

            // self:   2--6
            // other: 1----10
            // skip self to avoid double counting later
            if is_contained_by(&range, other) {
                skip.insert(i);
                subtotal = 0;
                break;
            }

            // self: 1------10
            // other:  2--6
            // skip other to avoid double counting later
            if is_contained_by(other, &range) {
                skip.insert(j);
                continue;
            }

            if other.start() > range.end() || other.end() < range.start() {
                continue;
            }
            // self:    5----10
            // other:       8---11
            else if range.start() < other.start() && range.end() < other.end() {
                fresh_ingredient_ranges[j] = RangeInclusive::new(range.end() + 1, *other.end());

                if fresh_ingredient_ranges[j].is_empty() {
                    skip.insert(j);
                }
            // self:     5----10
            // other:  3---6
            } else if other.start() < range.start() && other.end() < range.end() {
                fresh_ingredient_ranges[j] = RangeInclusive::new(*other.start(), range.start() - 1);

                if fresh_ingredient_ranges[j].is_empty() {
                    skip.insert(j);
                }
            }
        }

        total += subtotal;
    }

    total
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let (fresh_ingredient_ranges, available_ingredients) = parse(input)?;

    println!(
        "part1: {}",
        fresh_available_ingredients(&fresh_ingredient_ranges, &available_ingredients)
    );

    println!(
        "part2: {}",
        total_fresh_ingredients(&fresh_ingredient_ranges)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::days::day5::{
        fresh_available_ingredients, parse, parse_fresh_ingredient_ranges, total_fresh_ingredients,
    };

    #[test]
    fn test_part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .to_string();

        let (fresh_ranges, available) = parse(input).unwrap();

        assert_eq!(fresh_available_ingredients(&fresh_ranges, &available), 3);
    }

    #[test]
    fn test_part2() {
        let input = "3-5
10-14
16-20
12-18"
            .to_string();

        assert_eq!(
            total_fresh_ingredients(&parse_fresh_ingredient_ranges(input.as_str()).unwrap()),
            14
        );
    }
}
