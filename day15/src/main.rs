// I am lazy and I solved it with a vulgar brute force

use std::collections::HashMap;

type RiskMap = HashMap<(usize, usize), u32>;

const SIZE: usize = 100;

fn calculate_total_risk(
    risk_map: &RiskMap,
    (current_x, current_y): (usize, usize),
    current_risk: u32,
    risk_to_reach_position: &mut RiskMap,
    best_path_so_far: &mut u32,
) {
    if current_x == 5 * SIZE - 1 && current_y == 5 * SIZE - 1 {
        if current_risk < *best_path_so_far {
            *best_path_so_far = current_risk;
        }
    }
    risk_to_reach_position.insert((current_x, current_y), current_risk);

    let next_positions = [
        (Some(current_x), current_y.checked_sub(1)),
        (current_x.checked_sub(1), Some(current_y)),
        (Some(current_x + 1), Some(current_y)),
        (Some(current_x), Some(current_y + 1)),
    ];

    next_positions
        .iter()
        .filter_map(|&(x, y)| match (x, y) {
            (Some(x), Some(y)) => {
                if let Some(risk) = risk_map.get(&(x, y)) {
                    Some(((x, y), *risk))
                } else {
                    None
                }
            }
            _ => None,
        })
        .for_each(
            |((new_x, new_y), risk)| match risk_to_reach_position.get(&(new_x, new_y)) {
                Some(&calculated_risk) => {
                    if calculated_risk > current_risk + risk
                        && current_risk + risk < *best_path_so_far
                    {
                        calculate_total_risk(
                            risk_map,
                            (new_x, new_y),
                            current_risk + risk,
                            risk_to_reach_position,
                            best_path_so_far,
                        );
                    }
                }
                None => {
                    if current_risk + risk < *best_path_so_far {
                        calculate_total_risk(
                            risk_map,
                            (new_x, new_y),
                            current_risk + risk,
                            risk_to_reach_position,
                            best_path_so_far,
                        );
                    }
                }
            },
        );
}

fn extend_map(risk_map: &mut RiskMap) {
    (0..SIZE).for_each(|y| {
        (SIZE..5 * SIZE).for_each(|x| {
            let previous = risk_map.get(&(x - SIZE, y)).unwrap();
            let new = (previous + 1) % 10;
            let new = if new == 0 { 1 } else { new };
            risk_map.insert((x, y), new);
        })
    });

    (0..SIZE * 5).for_each(|x| {
        (SIZE..SIZE * 5).for_each(|y| {
            let previous = risk_map.get(&(x, y - SIZE)).unwrap();
            let new = (previous + 1) % 10;
            let new = if new == 0 { 1 } else { new };
            risk_map.insert((x, y), new);
        });
    });
}

fn calculate_simple_path_risk(risk_map: &RiskMap) -> u32 {
    (0..5 * SIZE)
        .map(|x| risk_map.get(&(x, 0)).unwrap())
        .sum::<u32>()
        + (0..5 * SIZE)
            .map(|y| risk_map.get(&(y, 5 * SIZE - 1)).unwrap())
            .sum::<u32>()
}

fn part_1(risk_map: &RiskMap) -> u32 {
    let mut simple_path = u32::MAX;
    let mut risk_to_reach_position = RiskMap::default();
    calculate_total_risk(
        &risk_map,
        (0, 0),
        0,
        &mut risk_to_reach_position,
        &mut simple_path,
    );

    *risk_to_reach_position.get(&(SIZE - 1, SIZE - 1)).unwrap()
}

fn part_2(mut risk_map: RiskMap) -> u32 {
    extend_map(&mut risk_map);

    let mut simple_path = calculate_simple_path_risk(&risk_map);
    let mut risk_to_reach_position = RiskMap::default();
    calculate_total_risk(
        &risk_map,
        (0, 0),
        0,
        &mut risk_to_reach_position,
        &mut simple_path,
    );

    *risk_to_reach_position
        .get(&(5 * SIZE - 1, 5 * SIZE - 1))
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let risk_map: RiskMap = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, field)| ((x, y.clone()), field.to_digit(10).unwrap()))
        })
        .flatten()
        .collect();

    assert_eq!(part_1(&risk_map), 811);
    assert_eq!(part_2(risk_map), 3012);
}
