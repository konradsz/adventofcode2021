#[derive(Clone, Copy)]
enum State {
    Loading(u32),
    Flashing,
}

type Octopuses = Vec<Vec<State>>;

fn increase_each_octopus_energy(octopuses: &mut Octopuses) {
    octopuses.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|octopus| {
            if let State::Loading(energy) = octopus {
                *energy += 1;
            }
        })
    });
}

fn increase_adjacent_octopuses_energy(octopuses: &mut Octopuses, coordinates: Vec<(usize, usize)>) {
    let get_adjacent = |octopuses: &Octopuses, x: usize, y: usize| {
        let coordinates = [
            (x.checked_sub(1), y.checked_sub(1)),
            (Some(x), y.checked_sub(1)),
            (Some(x + 1), y.checked_sub(1)),
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (x.checked_sub(1), Some(y + 1)),
            (Some(x), Some(y + 1)),
            (Some(x + 1), Some(y + 1)),
        ];

        coordinates
            .into_iter()
            .filter_map(|(x, y)| match (x, y) {
                (Some(x), Some(y)) => {
                    octopuses.get(y)?.get(x)?;
                    Some((x, y))
                }
                _ => None,
            })
            .collect::<Vec<(usize, usize)>>()
    };

    for (x, y) in coordinates {
        let adjacent_coordinates = get_adjacent(octopuses, x, y);
        for (adjacent_x, adjacent_y) in adjacent_coordinates {
            let row = &mut octopuses[adjacent_y];
            if let State::Loading(energy) = &mut row[adjacent_x] {
                *energy += 1;
            }
        }
    }
}

fn change_state_to_flashing_if_needed(octopuses: &mut Octopuses) -> Vec<(usize, usize)> {
    let mut changed_state = Vec::new();
    for (y, row) in octopuses.iter_mut().enumerate() {
        for (x, octopus) in row.iter_mut().enumerate() {
            if let State::Loading(energy) = octopus {
                if *energy > 9 {
                    *octopus = State::Flashing;

                    changed_state.push((x, y));
                }
            }
        }
    }

    changed_state
}

fn reset_flashing_octopuses(octopuses: &mut Octopuses) {
    octopuses.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|octopus| {
            if let State::Flashing = octopus {
                *octopus = State::Loading(0);
            }
        })
    });
}

fn check_if_all_flashing(octopuses: &Octopuses) -> bool {
    octopuses
        .iter()
        .all(|row| row.iter().all(|octopus| matches!(octopus, State::Flashing)))
}

fn part_1(mut octopuses: Octopuses) -> usize {
    let mut total_flashes = 0;
    for _ in 0..100 {
        increase_each_octopus_energy(&mut octopuses);
        loop {
            let changed_state = change_state_to_flashing_if_needed(&mut octopuses);
            if changed_state.is_empty() {
                break;
            } else {
                total_flashes += changed_state.len();
                increase_adjacent_octopuses_energy(&mut octopuses, changed_state);
            }
        }

        reset_flashing_octopuses(&mut octopuses);
    }

    total_flashes
}

fn part_2(mut octopuses: Octopuses) -> usize {
    for step in 0.. {
        increase_each_octopus_energy(&mut octopuses);
        loop {
            let changed_state = change_state_to_flashing_if_needed(&mut octopuses);
            if changed_state.is_empty() {
                break;
            } else {
                increase_adjacent_octopuses_energy(&mut octopuses, changed_state);
            }
        }

        if check_if_all_flashing(&octopuses) {
            return step + 1;
        }
        reset_flashing_octopuses(&mut octopuses);
    }

    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let octopuses: Octopuses = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| State::Loading(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect();

    assert_eq!(part_1(octopuses.clone()), 1694);
    assert_eq!(part_2(octopuses), 346);
}
