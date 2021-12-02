use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse_input() -> Result<Vec<Command>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| -> Result<Command> {
            let line = line?;
            let mut parts = line.split_whitespace();
            let (direction, value) = (
                parts.next().ok_or(anyhow!("missing direction"))?,
                parts
                    .next()
                    .ok_or(anyhow!("missing value"))?
                    .parse::<usize>()?,
            );
            match direction {
                "forward" => Ok(Command::Forward(value)),
                "down" => Ok(Command::Down(value)),
                "up" => Ok(Command::Up(value)),
                _ => Err(anyhow!("unsupported command")),
            }
        })
        .collect()
}

fn part_1(commands: &[Command]) -> usize {
    let (horizontal_position, depth) = commands.iter().fold(
        (0, 0),
        |(horizontal_position, depth), command| match command {
            Command::Forward(value) => (horizontal_position + value, depth),
            Command::Down(value) => (horizontal_position, depth + value),
            Command::Up(value) => (horizontal_position, depth - value),
        },
    );

    horizontal_position * depth
}

fn part_2(commands: &[Command]) -> usize {
    let (horizontal_position, depth, _) = commands.iter().fold(
        (0, 0, 0),
        |(horizontal_position, depth, aim), command| match command {
            Command::Forward(value) => (horizontal_position + value, depth + (aim * value), aim),
            Command::Down(value) => (horizontal_position, depth, aim + value),
            Command::Up(value) => (horizontal_position, depth, aim - value),
        },
    );

    horizontal_position * depth
}

fn main() -> Result<()> {
    let input = parse_input()?;
    assert_eq!(part_1(&input), 1936494);
    assert_eq!(part_2(&input), 1997106066);
    Ok(())
}
