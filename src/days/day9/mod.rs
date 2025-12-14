use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

fn parse_red_tile_positions(input: String) -> anyhow::Result<Vec<[usize; 2]>> {
    input
        .lines()
        .map(|line| match line.split_once(",") {
            Some((left, right)) => {
                let column: usize = left.parse()?;
                let row: usize = right.parse()?;

                Ok([column, row])
            }
            None => anyhow::bail!("expected comma separated column,row"),
        })
        .collect()
}

fn largest_rectangle_with_red_tile_corners(red_tile_positions: &[[usize; 2]]) -> usize {
    let mut max = usize::MIN;

    for (i, a) in red_tile_positions.iter().enumerate() {
        for b in red_tile_positions.iter().skip(i + 1) {
            let area = (a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1);

            if area > max {
                max = area;
            }
        }
    }

    max
}

fn areas_by_descending_size(red_tile_positions: &[[usize; 2]]) -> Vec<(usize, usize, usize)> {
    let mut areas: Vec<(usize, usize, usize)> = Vec::new();

    for (i, a) in red_tile_positions.iter().enumerate() {
        for (j, b) in red_tile_positions.iter().enumerate().skip(i + 1) {
            let area = (a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1);

            areas.push((i, j, area))
        }
    }

    areas.sort_by_key(|(_, _, area)| Reverse(*area));

    areas
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<bool>]) {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    true => 'X',
                    false => '.',
                })
                .collect::<Vec<char>>()
        })
        .for_each(|row| println!("{:?}", row));
    println!();
}

fn flood_fill(grid: &mut [Vec<bool>]) {
    let mut frontier: VecDeque<[usize; 2]> = VecDeque::new();
    let mut outside: HashSet<[usize; 2]> = HashSet::new();

    grid[0][0] = false;
    outside.insert([0, 0]);
    frontier.push_back([0, 0]);

    while let Some([column, row]) = frontier.pop_front() {
        let mut neighbours: Vec<[usize; 2]> = Vec::with_capacity(4);

        if row != 0 {
            neighbours.push([column, row - 1]);
        }
        if row < grid.len() - 1 {
            neighbours.push([column, row + 1])
        }

        if column != 0 {
            neighbours.push([column - 1, row])
        }
        if column < grid[0].len() - 1 {
            neighbours.push([column + 1, row])
        }

        for neighbour in neighbours {
            let [nc, nr] = neighbour;

            if !grid[nr][nc] && !outside.contains(&neighbour) {
                frontier.push_back(neighbour);
                outside.insert(neighbour);
            }
        }
    }

    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            let position = [column, row];

            if !outside.contains(&position) {
                grid[row][column] = true
            }
        }
    }
}

fn largest_rectangle_with_red_tile_corners_and_green_tiles(
    red_tile_positions: &[[usize; 2]],
) -> usize {
    let mut rows = HashSet::new();
    let mut columns = HashSet::new();

    red_tile_positions.iter().for_each(|&[column, row]| {
        rows.insert(row);
        columns.insert(column);
    });

    let mut rows: Vec<usize> = rows.iter().cloned().collect();
    let max_row = *rows.iter().max().unwrap();
    let min_row = *rows.iter().min().unwrap();
    rows.push(max_row + 1);
    rows.push(min_row - 1);
    rows.sort();

    let mut columns: Vec<usize> = columns.iter().cloned().collect();
    let max_column = *columns.iter().max().unwrap();
    let min_column = *columns.iter().min().unwrap();
    columns.push(min_column - 1);
    columns.push(max_column + 1);
    columns.sort();

    let compressed_positions: Vec<[usize; 2]> = red_tile_positions
        .iter()
        .map(|[column, row]| {
            [
                columns.iter().position(|c| c == column).unwrap(),
                rows.iter().position(|r| r == row).unwrap(),
            ]
        })
        .collect();

    let mut grid = vec![vec![false; columns.len()]; rows.len()];

    for (a, b) in compressed_positions.iter().zip(
        compressed_positions
            .iter()
            .skip(1)
            .chain(compressed_positions.iter().take(1)),
    ) {
        for row in a[1].min(b[1])..=a[1].max(b[1]) {
            for column in a[0].min(b[0])..=a[0].max(b[0]) {
                grid[row][column] = true
            }
        }
    }

    flood_fill(&mut grid);

    let areas = areas_by_descending_size(red_tile_positions);

    for (a, b, area) in areas.iter() {
        // check perimeter of rectangle made by a and b
        // if any point on the perimeter is outside the border, then the area is invalid
        // if the perimeter is inside, then so are the contents
        let mut valid = true;

        let ca = compressed_positions[*a];
        let cb = compressed_positions[*b];

        let mut cr = [ca[1], cb[1]];
        cr.sort();

        let mut cc = [ca[0], cb[0]];
        cc.sort();

        for column in cc[0]..=cc[1] {
            for row in [cr[0], cr[1]] {
                if !grid[row][column] {
                    valid = false;
                    break;
                }
            }
        }

        if !valid {
            continue;
        }

        for row in cr[0]..=cr[1] {
            for column in [cc[0], cc[1]] {
                if !grid[row][column] {
                    valid = false;
                    break;
                }
            }
        }

        if !valid {
            continue;
        }

        return *area;
    }

    // should be unreachable, but default to 0 if no rectangle is valid
    0
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let red_tile_positions = parse_red_tile_positions(input)?;

    println!(
        "part1: {}",
        largest_rectangle_with_red_tile_corners(&red_tile_positions)
    );

    println!(
        "part2: {}",
        largest_rectangle_with_red_tile_corners_and_green_tiles(&red_tile_positions,)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_with_red_tile_corners() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        .to_string();

        assert_eq!(
            largest_rectangle_with_red_tile_corners(&parse_red_tile_positions(input).unwrap()),
            50
        );
    }

    #[test]
    fn test_largest_rectangle_with_red_tile_corners_and_green_tiles() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        .to_string();

        let red_tile_positions = &parse_red_tile_positions(input).unwrap();

        assert_eq!(
            largest_rectangle_with_red_tile_corners_and_green_tiles(red_tile_positions,),
            24
        );
    }
}
