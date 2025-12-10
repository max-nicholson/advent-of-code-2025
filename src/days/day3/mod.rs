use std::str::FromStr;

#[derive(Debug)]
struct Battery {
    joltage: u8,
}

#[derive(Debug)]
struct Bank {
    batteries: Vec<Battery>,
}

impl FromStr for Bank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            batteries: s
                .bytes()
                .map(|b| Ok(Battery { joltage: b - 48 }))
                .collect::<anyhow::Result<Vec<_>>>()?,
        })
    }
}

impl Bank {
    pub fn max_joltage(&self, batteries: usize) -> usize {
        if batteries > self.batteries.len() {
            return 0;
        }

        let mut total = 0;
        let mut left = 0;

        for digit in 0..batteries {
            let mut max_index = 0;
            let mut max = 0;

            let must_leave = batteries - digit - 1;

            self.batteries
                .iter()
                .enumerate()
                .skip(left)
                .take(self.batteries.len() - must_leave - left)
                .for_each(|(i, battery)| {
                    if battery.joltage > max {
                        max_index = i;
                        max = battery.joltage;
                    }
                });

            total += 10usize.pow((batteries - digit) as u32 - 1) * max as usize;

            left = max_index + 1;
        }

        total
    }
}

fn parse(input: String) -> anyhow::Result<Vec<Bank>> {
    input.lines().map(str::parse::<Bank>).collect()
}

fn total_joltage(banks: &[Bank], batteries: usize) -> usize {
    banks
        .iter()
        .fold(0, |acc, bank| acc + bank.max_joltage(batteries))
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let banks = parse(input)?;

    println!("part1: {}", total_joltage(&banks, 2));
    println!("part2: {}", total_joltage(&banks, 12));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_bank() {
        let input = "9876".to_string();

        assert_eq!(total_joltage(&parse(input).unwrap(), 2), 98)
    }

    #[test]
    fn test_part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_string();

        assert_eq!(total_joltage(&parse(input).unwrap(), 2), 357);
    }

    #[test]
    fn test_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_string();

        assert_eq!(total_joltage(&parse(input).unwrap(), 12), 3121910778619);
    }
}
