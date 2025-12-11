use std::collections::HashSet;

fn parse(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

pub fn accessible_rolls_of_paper(grid: &[Vec<char>]) -> impl Iterator<Item = (usize, usize)> {
    grid.iter().enumerate().flat_map(move |(row_index, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(column_index, position)| {
                if *position != '@' {
                    return None;
                }

                let mut adjacent_paper_rolls = 0;

                for row_offset in -1isize..=1 {
                    if row_index == 0 && row_offset == -1
                        || row_index == grid.len() - 1 && row_offset == 1
                    {
                        continue;
                    }

                    for column_offset in -1isize..=1 {
                        if column_index == 0 && column_offset == -1
                            || column_index == grid[0].len() - 1 && column_offset == 1
                        {
                            continue;
                        }

                        if column_index == 0 && row_index == 0 {
                            continue;
                        }

                        if grid[row_index.strict_add_signed(row_offset)]
                            [column_index.strict_add_signed(column_offset)]
                            == '@'
                        {
                            adjacent_paper_rolls += 1
                        }
                    }
                }

                if adjacent_paper_rolls <= 4 {
                    Some((row_index, column_index))
                } else {
                    None
                }
            })
    })
}

fn accessible_rolls_of_paper_with_removals(original_grid: &[Vec<char>]) -> usize {
    let mut counter = 0;
    let mut grid: Vec<Vec<char>> = original_grid.to_vec();

    loop {
        let removals: HashSet<(usize, usize)> = accessible_rolls_of_paper(&grid).collect();

        if removals.is_empty() {
            break;
        }

        counter += removals.len();

        for row_index in 0..grid.len() {
            for column_index in 0..grid[0].len() {
                if removals.contains(&(row_index, column_index)) {
                    grid[row_index][column_index] = '.'
                }
            }
        }
    }

    counter
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let grid = parse(input);

    println!("part1: {}", accessible_rolls_of_paper(&grid).count());
    println!("part2: {}", accessible_rolls_of_paper_with_removals(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();

        assert_eq!(accessible_rolls_of_paper(&parse(input)).count(), 13)
    }

    #[test]
    fn test_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();

        assert_eq!(accessible_rolls_of_paper_with_removals(&parse(input)), 43)
    }
}
