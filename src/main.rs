#![feature(result_option_map_or_default)]
use std::fs;

mod days;

use anyhow::Context;

use crate::days::*;

struct Args {
    day: u8,
}

impl Args {
    pub fn parse() -> anyhow::Result<Self> {
        let args = std::env::args().collect::<Vec<String>>();

        match args.len() {
            2 => {
                let day: u8 = args[1].parse()?;

                Ok(Self { day })
            }
            1 => anyhow::bail!("not enough arguments; want day"),
            _ => anyhow::bail!("too many arguments; want day"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;
    let path = format!("src/days/day{}/input.txt", args.day);
    let input = fs::read_to_string(&path).context(path)?;

    match args.day {
        1 => day1::solve(input)?,
        2 => day2::solve(input)?,
        3 => day3::solve(input)?,
        4 => day4::solve(input)?,
        5 => day5::solve(input)?,
        6 => day6::solve(input)?,
        7 => day7::solve(input)?,
        8 => day8::solve(input)?,
        9 => day9::solve(input)?,
        10 => day10::solve(input)?,
        11 => day11::solve(input)?,
        12 => day12::solve(input)?,
        _ => anyhow::bail!("expected day between 1 and 12, got {}", args.day),
    }

    Ok(())
}
