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

        let (mut r1, mut r2, mut r3, mut r4, mut r5, mut r6, mut r7, mut r8);
        let range: &mut dyn Iterator<Item = (u32, u32)> = if x1 == x2 {
            if y1 < y2 {
                r1 = (y1..=y2).map(|y| (x1, y));
                &mut r1
            } else {
                r2 = (y2..=y1).map(|y| (x1, y));
                &mut r2
            }
        } else if y1 == y2 {
            if x1 < x2 {
                r3 = (x1..=x2).map(|x| (x, y1));
                &mut r3
            } else {
                r4 = (x2..=x1).map(|x| (x, y1));
                &mut r4
            }
        } else if x1 < x2 {
            if y1 < y2 {
                r5 = (x1..=x2).zip(y1..=y2);
                &mut r5
            } else {
                r6 = (x1..=x2).zip((y2..=y1).rev());
                &mut r6
            }
        } else if x1 > x2 {
            if y1 < y2 {
                r7 = (x2..=x1).zip((y1..=y2).rev());
                &mut r7
            } else {
                r8 = (x2..=x1).zip(y2..=y1);
                &mut r8
            }
        } else {
            unreachable!()
        };

        range.for_each(|(x, y)| *diagram.entry((x, y)).or_default() += 1);
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
                .ok_or(anyhow!("missing start"))?
                .split(',')
                .chain(parts.next().ok_or(anyhow!("missing end"))?.split(','))
                .map(|n| -> Result<u32> { Ok(n.parse::<u32>()?) });

            Ok((
                (
                    coords.next().ok_or(anyhow!("no x1"))??,
                    coords.next().ok_or(anyhow!("no y1"))??,
                ),
                (
                    coords.next().ok_or(anyhow!("no x2"))??,
                    coords.next().ok_or(anyhow!("no y2"))??,
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
