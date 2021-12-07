use anyhow::{anyhow, Result};

fn part_1(positions: &[i32], min: i32, max: i32) -> i32 {
    (min..=max)
        .map(|meeting_point| {
            positions
                .iter()
                .map(|&p| (meeting_point - p).abs())
                .sum::<i32>()
        })
        .min()
        .unwrap()
}

fn part_2(positions: &[i32], min: i32, max: i32) -> i32 {
    (min..=max)
        .map(|meeting_point| {
            positions
                .iter()
                .map(|&p| {
                    let n = (meeting_point - p).abs();
                    (n * (n + 1)) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
}

fn main() -> Result<()> {
    let positions = std::fs::read_to_string("input")?
        .split(',')
        .map(|n| -> Result<i32> { Ok(n.parse::<i32>()?) })
        .collect::<Result<Vec<_>>>()?;

    let min = *positions.iter().min().ok_or(anyhow!("empty input"))?;
    let max = *positions.iter().max().ok_or(anyhow!("empty input"))?;

    assert_eq!(part_1(&positions, min, max), 337488);
    assert_eq!(part_2(&positions, min, max), 89647695);

    Ok(())
}
