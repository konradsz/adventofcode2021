use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

type Coordinates = ((u32, u32), (u32, u32));

fn part_1(input: &Vec<Coordinates>) -> usize {
    let is_vertical_or_horizontal = |((x1, y1), (x2, y2)): &&Coordinates| x1 == x2 || y1 == y2;
    count_overlapping_points(input.iter().filter(is_vertical_or_horizontal))
}

fn part_2(input: &Vec<Coordinates>) -> usize {
    count_overlapping_points(input.iter())
}

fn count_overlapping_points<'a>(input_iterator: impl Iterator<Item = &'a Coordinates>) -> usize {
    let mut diagram: HashMap<(u32, u32), u32> = HashMap::new();
    for coords in input_iterator {
        let ((x1, y1), (x2, y2)) = *coords;

        if x1 == x2 {
            if y1 < y2 {
                (y1..=y2).for_each(|y| *diagram.entry((x1, y)).or_default() += 1);
            } else {
                (y2..=y1).for_each(|y| *diagram.entry((x1, y)).or_default() += 1);
            }
        } else if y1 == y2 {
            if x1 < x2 {
                (x1..=x2).for_each(|x| *diagram.entry((x, y1)).or_default() += 1);
            } else {
                (x2..=x1).for_each(|x| *diagram.entry((x, y1)).or_default() += 1);
            }
        } else if x1 < x2 {
            if y1 < y2 {
                (x1..=x2)
                    .zip(y1..=y2)
                    .for_each(|(x, y)| *diagram.entry((x, y)).or_default() += 1);
            } else {
                (x1..=x2)
                    .zip((y2..=y1).rev())
                    .for_each(|(x, y)| *diagram.entry((x, y)).or_default() += 1);
            }
        } else if x1 > x2 {
            if y1 < y2 {
                (x2..=x1)
                    .zip((y1..=y2).rev())
                    .for_each(|(x, y)| *diagram.entry((x, y)).or_default() += 1);
            } else {
                (x2..=x1)
                    .zip(y2..=y1)
                    .for_each(|(x, y)| *diagram.entry((x, y)).or_default() += 1);
            }
        }
    }

    diagram.values().filter(|&&value| value > 1).count()
}

fn parse_input() -> Result<Vec<Coordinates>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| -> Result<Coordinates> {
            let line = line?;

            let mut parts = line.split(" -> ");
            let mut coords = parts
                .next()
                .ok_or(anyhow!("invalid input"))?
                .split(',')
                .chain(parts.next().ok_or(anyhow!("invalid input"))?.split(','))
                .map(|n| -> Result<u32> { Ok(n.parse::<u32>()?) });

            Ok((
                (
                    coords.next().ok_or(anyhow!("invalid input"))??,
                    coords.next().ok_or(anyhow!("invalid input"))??,
                ),
                (
                    coords.next().ok_or(anyhow!("invalid input"))??,
                    coords.next().ok_or(anyhow!("invalid input"))??,
                ),
            ))
        })
        .collect::<Result<Vec<_>>>()?)
}

fn main() -> Result<()> {
    let input = parse_input()?;

    assert_eq!(part_1(&input), 6564);
    assert_eq!(part_2(&input), 19172);

    Ok(())
}
