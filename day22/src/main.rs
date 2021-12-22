use std::{collections::HashSet, ops::RangeInclusive};

fn turn_on_cubes(
    cubes: &mut HashSet<(i64, i64, i64)>,
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
) {
    for x in x_range {
        for y in y_range.clone() {
            for z in z_range.clone() {
                cubes.insert((x, y, z));
            }
        }
    }
}

fn turn_off_cubes(
    cubes: &mut HashSet<(i64, i64, i64)>,
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
) {
    for x in x_range {
        for y in y_range.clone() {
            for z in z_range.clone() {
                cubes.remove(&(x, y, z));
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut cubes = HashSet::new();
    let instructions = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let instruction = parts.next().unwrap();

            let extract_numbers = |range: &str| {
                let mut numbers = range.split_at(2).1.split("..");

                numbers.next().unwrap().parse::<i64>().unwrap()
                    ..=numbers.next().unwrap().parse::<i64>().unwrap()
            };
            let mut coords = parts.next().unwrap().split(',');

            (
                instruction,
                extract_numbers(coords.next().unwrap()),
                extract_numbers(coords.next().unwrap()),
                extract_numbers(coords.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    for (instruction, x_range, y_range, z_range) in instructions {
        if *x_range.start() < -50
            || *x_range.end() > 50
            || *y_range.start() < -50
            || *y_range.end() > 50
            || *z_range.start() < -50
            || *z_range.end() > 50
        {
            continue;
        }
        match instruction {
            "on" => turn_on_cubes(&mut cubes, x_range, y_range, z_range),
            "off" => turn_off_cubes(&mut cubes, x_range, y_range, z_range),
            _ => panic!("unknown instruction"),
        };
    }

    // println!("{}", cubes.len());
    assert_eq!(cubes.len(), 553201);
}
