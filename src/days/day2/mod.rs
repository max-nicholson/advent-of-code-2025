use std::ops::RangeInclusive;

type IDRange = RangeInclusive<ID>;

type ID = usize;

fn parse_id_ranges(ranges: String) -> anyhow::Result<Vec<IDRange>> {
    ranges
        .split(",")
        .map(|value| match value.trim().split_once("-") {
            Some((start, end)) => {
                let start: usize = start.parse()?;
                let end: usize = end.parse()?;

                Ok(RangeInclusive::new(start, end))
            }
            None => anyhow::bail!("expected ID range to be '-' separated"),
        })
        .collect()
}

fn invalid_ids_with_exactly_one_repeated_sequence(range: IDRange) -> impl Iterator<Item = ID> {
    range.filter(|id| {
        let s: Vec<char> = id.to_string().chars().collect();

        let digits = s.len();

        if digits % 2 == 1 {
            return false;
        }

        let mut left = digits / 2;
        let mut right = 0;

        for _ in 0..(digits / 2) {
            if s[left] != s[right] {
                return false;
            }

            right += 1;
            left += 1;
        }

        true
    })
}

fn invalid_ids_with_at_least_two_repeated_sequences(range: IDRange) -> impl Iterator<Item = ID> {
    range.filter(|id| {
        let s: Vec<char> = id.to_string().chars().collect();

        let digits = s.len();

        if digits == 1 {
            return false;
        }

        for i in 1..=(digits / 2) {
            if digits.is_multiple_of(i)
                && s.chunks(i)
                    .collect::<Vec<_>>()
                    .windows(2)
                    .all(|w| w[0] == w[1])
            {
                return true;
            }
        }

        false
    })
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let ranges = parse_id_ranges(input)?;

    println!(
        "part 1: {}",
        ranges.iter().cloned().fold(0, |acc, range| acc
            + invalid_ids_with_exactly_one_repeated_sequence(range).sum::<usize>())
    );

    println!(
        "part 2: {}",
        ranges.iter().cloned().fold(0, |acc, range| acc
            + invalid_ids_with_at_least_two_repeated_sequences(range).sum::<usize>())
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();

        let ranges = parse_id_ranges(input).unwrap();

        assert_eq!(
            ranges
                .iter()
                .cloned()
                .flat_map(invalid_ids_with_exactly_one_repeated_sequence)
                .collect::<Vec<usize>>(),
            vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859,]
        );
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();

        let ranges = parse_id_ranges(input).unwrap();

        assert_eq!(
            ranges
                .iter()
                .cloned()
                .flat_map(invalid_ids_with_at_least_two_repeated_sequences)
                .collect::<Vec<usize>>(),
            vec![
                11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656,
                824824824, 2121212121
            ]
        );
    }
}
