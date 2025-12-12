use std::num::ParseIntError;

fn calculate_worksheet(numbers: &[Vec<usize>], operators: &[&str]) -> usize {
    operators
        .iter()
        .zip(numbers)
        .map(|(&operator, problem)| match operator {
            "+" => problem.iter().sum::<usize>(),
            "*" => problem.iter().product::<usize>(),
            _ => unimplemented!(),
        })
        .sum()
}
fn parse_operators(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

fn parse_as_cephalopod_math(input: &str) -> anyhow::Result<(Vec<Vec<usize>>, Vec<&str>)> {
    let lines: Vec<&str> = input.lines().collect();

    let inputs_per_problem = lines.len() - 1;

    let problems = lines[0].split_whitespace().count();

    let mut numbers: Vec<Vec<usize>> = vec![vec![0; 0]; problems];

    let mut numbers_as_bytes: Vec<Vec<u8>> = lines
        .iter()
        .take(inputs_per_problem)
        .map(|&line| line.as_bytes().to_vec())
        .collect();

    let total_characters = numbers_as_bytes
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or_default();

    // re-pad any stripped line
    numbers_as_bytes.iter_mut().for_each(|line| {
        let missing = total_characters - line.len();

        for _ in 0..missing {
            line.push(b' ');
        }
    });

    let mut i = total_characters - 1;
    let mut problem = problems - 1;

    loop {
        let input: Vec<u8> = numbers_as_bytes
            .iter()
            .filter_map(|line| {
                if line[i] >= 49 && line[i] <= 57 {
                    Some(line[i] - b'0')
                } else {
                    None
                }
            })
            .collect();

        if input.is_empty() {
            problem -= 1;
        } else {
            numbers[problem].push(
                input
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0usize, |acc, (digit, &value)| {
                        acc + 10usize.pow(digit as u32) * value as usize
                    }),
            );
        }

        if i != 0 {
            i -= 1;
        } else {
            break;
        }
    }

    let operators = parse_operators(lines[lines.len() - 1]);

    Ok((numbers, operators))
}

fn parse(input: &str) -> anyhow::Result<(Vec<Vec<usize>>, Vec<&str>)> {
    let lines: Vec<&str> = input.lines().collect();

    let operators = parse_operators(lines[lines.len() - 1]);

    let number_of_problems = operators.len();

    let numbers = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, ParseIntError>>()
        })
        .collect::<Result<Vec<Vec<usize>>, ParseIntError>>()?;

    let numbers: Vec<Vec<usize>> = (0..number_of_problems)
        .map(|i| numbers.iter().map(|line| line[i]).collect())
        .collect();

    Ok((numbers, operators))
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let (numbers, operators) = parse(&input)?;

    println!("part1: {}", calculate_worksheet(&numbers, &operators));

    let (numbers, operators) = parse_as_cephalopod_math(&input)?;

    println!("part2: {}", calculate_worksheet(&numbers, &operators));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_worksheet() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
            .to_string();

        let (numbers, operators) = parse(&input).unwrap();

        assert_eq!(calculate_worksheet(&numbers, &operators), 4277556);
    }

    #[test]
    fn test_calculate_worksheet_with_cephalopod_math() {
        // need to break up the input to prevent editor from trimming (in this case meaningful) whitespace
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
            .to_string();

        let (numbers, operators) = parse_as_cephalopod_math(&input).unwrap();

        assert_eq!(calculate_worksheet(&numbers, &operators), 3263827);
    }
}
