use std::{fmt::Display, str::FromStr};

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => anyhow::bail!("expected direction of L or R"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

struct Rotation {
    direction: Direction,
    distance: u16,
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let direction: Direction = value[..1].parse()?;

        let distance: u16 = value[1..].parse()?;

        Ok(Self {
            distance,
            direction,
        })
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}

struct Dial {
    clicks: u8,
}

impl Dial {
    pub fn rotate(&mut self, rotation: &Rotation) {
        let distance: u8 = (rotation.distance % 100) as u8;

        match rotation.direction {
            Direction::Left => {
                self.clicks = (self.clicks + (100 - distance)) % 100;
            }
            Direction::Right => {
                self.clicks = (self.clicks + distance) % 100;
            }
        }

        debug_assert!((0..=99).contains(&self.clicks));
    }

    pub fn count_zeroes_between_rotations<'a>(
        &mut self,
        rotations: impl Iterator<Item = &'a Rotation>,
    ) -> usize {
        let mut counter = 0;

        for rotation in rotations {
            self.rotate(rotation);

            if self.clicks == 0 {
                counter += 1
            }
        }

        counter
    }

    pub fn count_zeroes<'a>(&mut self, rotations: impl Iterator<Item = &'a Rotation>) -> usize {
        let mut counter: usize = 0;

        for rotation in rotations {
            match rotation.direction {
                Direction::Left => {
                    let quotient = rotation.distance / 100;
                    let remainder = rotation.distance % 100;

                    let clicks = (self.clicks as i16) - (remainder as i16);

                    if clicks == 0 {
                        self.clicks = 0;
                        counter += (quotient + 1) as usize;
                    } else if clicks < 0 {
                        if self.clicks == 0 {
                            // avoid treating 0 -> X as "going through 0"
                            counter += (quotient) as usize;
                        } else {
                            counter += (quotient + 1) as usize;
                        }
                        self.clicks = (clicks + 100) as u8;
                    } else {
                        self.clicks = clicks as u8;
                        counter += quotient as usize;
                    }
                }
                Direction::Right => {
                    let quotient = rotation.distance / 100;
                    let remainder = rotation.distance % 100;

                    let clicks = self.clicks + (remainder as u8);

                    if clicks >= 100 {
                        counter += (quotient + 1) as usize;
                        self.clicks = clicks % 100;
                    } else {
                        counter += quotient as usize;
                        self.clicks = clicks;
                    }
                }
            }

            debug_assert!((0..=99).contains(&self.clicks));
        }

        counter
    }

    pub fn new() -> Self {
        Self { clicks: 50 }
    }
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let rotations = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::parse::<Rotation>)
        .collect::<anyhow::Result<Vec<_>>>()?;

    println!(
        "part 1: {}",
        Dial::new().count_zeroes_between_rotations(rotations.iter())
    );

    println!("part 2: {}", Dial::new().count_zeroes(rotations.iter()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_zeroes_between_rotations() {
        let mut dial = Dial::new();

        let rotations = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string()
        .lines()
        .map(str::parse::<Rotation>)
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();

        assert_eq!(dial.count_zeroes_between_rotations(rotations.iter()), 3);
    }
    #[test]
    fn test_count_zeroes() {
        let mut dial = Dial::new();

        let rotations = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string()
        .lines()
        .map(str::parse::<Rotation>)
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();

        assert_eq!(dial.count_zeroes(rotations.iter()), 6);
    }

    #[test]
    fn test_count_zeroes_overflowing_rotation() {
        assert_eq!(
            Dial::new().count_zeroes(
                [
                    "R1000".parse::<Rotation>().unwrap(),
                    "L1000".parse::<Rotation>().unwrap()
                ]
                .iter()
            ),
            20
        )
    }

    #[test]
    fn test_count_zeroes_right_rotation_under_99() {
        let mut dial = Dial::new();

        assert_eq!(
            dial.count_zeroes(["R49".parse::<Rotation>().unwrap(),].iter()),
            0
        );
        assert_eq!(dial.clicks, 99);
    }

    #[test]
    fn test_count_zeroes_right_rotation_to_0() {
        let mut dial = Dial::new();
        assert_eq!(
            dial.count_zeroes(["R50".parse::<Rotation>().unwrap(),].iter()),
            1
        );

        assert_eq!(dial.clicks, 0)
    }

    #[test]
    fn test_count_zeroes_right_rotation_past_0() {
        let mut dial = Dial::new();
        assert_eq!(
            dial.count_zeroes(["R51".parse::<Rotation>().unwrap(),].iter()),
            1
        );

        assert_eq!(dial.clicks, 1)
    }

    #[test]
    fn test_count_zeroes_left_rotation_above_0() {
        let mut dial = Dial::new();

        assert_eq!(
            dial.count_zeroes(["L49".parse::<Rotation>().unwrap(),].iter()),
            0
        );
        assert_eq!(dial.clicks, 1);
    }

    #[test]
    fn test_count_zeroes_left_rotation_to_0() {
        let mut dial = Dial::new();

        assert_eq!(
            dial.count_zeroes(["L50".parse::<Rotation>().unwrap(),].iter()),
            1
        );

        assert_eq!(dial.clicks, 0)
    }

    #[test]
    fn test_count_zeroes_left_rotation_past_0() {
        let mut dial = Dial::new();

        assert_eq!(
            dial.count_zeroes(["L51".parse::<Rotation>().unwrap(),].iter()),
            1
        );

        assert_eq!(dial.clicks, 99)
    }

    #[test]
    fn test_count_zeroes_left_rotation_from_0() {
        let mut dial = Dial { clicks: 0 };

        assert_eq!(
            dial.count_zeroes(["L1".parse::<Rotation>().unwrap(),].iter()),
            0
        );

        assert_eq!(dial.clicks, 99)
    }

    #[test]
    fn test_sequence_of_rotations() {
        let mut dial = Dial::new();

        let mut clicks: Vec<u8> = Vec::new();

        let rotations = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string()
        .lines()
        .map(str::parse::<Rotation>)
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();

        for rotation in rotations {
            dial.rotate(&rotation);
            clicks.push(dial.clicks)
        }

        assert_eq!(clicks, vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32]);
    }

    #[test]
    fn test_overflowing_rotations() {
        let mut dial = Dial::new();

        dial.rotate(&Rotation {
            direction: Direction::Left,
            distance: 101,
        });

        assert_eq!(dial.clicks, 49);

        dial.rotate(&Rotation {
            direction: Direction::Right,
            distance: 101,
        });

        assert_eq!(dial.clicks, 50);

        dial.rotate(&Rotation {
            direction: Direction::Right,
            distance: 407,
        });

        assert_eq!(dial.clicks, 57);
    }
}
