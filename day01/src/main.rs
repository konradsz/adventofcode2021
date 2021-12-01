use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn part_1(input: &[u64]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part_2(input: &[u64]) -> usize {
    input
        .windows(3)
        .zip(input.windows(3).skip(1))
        .filter(|(w1, w2)| w1.iter().sum::<u64>() < w2.iter().sum::<u64>())
        .count()
}

fn main() -> Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|line| -> Result<u64> { Ok(line?.parse()?) })
        .collect::<Result<Vec<u64>>>()?;

    assert_eq!(part_1(&input), 1448);
    assert_eq!(part_2(&input), 1471);

    Ok(())
}
