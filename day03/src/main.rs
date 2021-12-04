use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const DIGITS: u32 = 12;

fn part_1(numbers: &[u32]) -> u32 {
    let is_1_more_popular_on_position = |position: u32| {
        numbers
            .iter()
            .filter(|&&number| number & 1 << position > 0)
            .count()
            > numbers.len() / 2
    };

    let gamma_rate = (0..DIGITS).fold(0u32, |result, position| {
        if is_1_more_popular_on_position(position) {
            result | (1 << position)
        } else {
            result
        }
    });

    let epsilon_rate = !gamma_rate & u32::MAX >> (32 - DIGITS);

    gamma_rate * epsilon_rate
}

fn part_2(numbers: Vec<u32>) -> u32 {
    let mut oxygen_numbers = numbers.clone();
    let mut co2_numbers = numbers;

    let is_more_1_bits_on_position = |numbers: &[u32], position: u32| {
        let (zeros, ones) = numbers.iter().fold((0, 0), |(zeros, ones), number| {
            if (*number & 1 << position) > 0 {
                (zeros, ones + 1)
            } else {
                (zeros + 1, ones)
            }
        });

        ones >= zeros
    };

    for shift in 0..DIGITS {
        let more_1_bits = is_more_1_bits_on_position(&oxygen_numbers, DIGITS - 1 - shift);
        if more_1_bits {
            oxygen_numbers.retain(|number| (number & 1 << (DIGITS - 1 - shift)) >= 1);
        } else {
            oxygen_numbers.retain(|number| (number & 1 << (DIGITS - 1 - shift)) == 0);
        }

        if oxygen_numbers.len() == 1 {
            break;
        }
    }

    for shift in 0..DIGITS {
        let more_0_bits = !is_more_1_bits_on_position(&co2_numbers, DIGITS - 1 - shift);
        if more_0_bits {
            co2_numbers.retain(|number| (number & 1 << (DIGITS - 1 - shift)) >= 1);
        } else {
            co2_numbers.retain(|number| (number & 1 << (DIGITS - 1 - shift)) == 0);
        }

        if co2_numbers.len() == 1 {
            break;
        }
    }

    oxygen_numbers[0] * co2_numbers[0]
}

fn main() -> Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let numbers = reader
        .lines()
        .map(|line| -> Result<u32> { Ok(u32::from_str_radix(&line?, 2)?) })
        .collect::<Result<Vec<_>>>()?;

    assert_eq!(part_1(&numbers), 4174964);
    assert_eq!(part_2(numbers), 4474944);

    Ok(())
}
