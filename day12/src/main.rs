use std::collections::HashMap;

fn is_small_cave(cave: &str) -> bool {
    cave == "start" || cave.chars().all(|c| c.is_lowercase())
}

fn remove_connections_to(cave: &str, connections: &mut HashMap<String, Vec<String>>) {
    connections.values_mut().for_each(|destination_caves| {
        destination_caves.retain(|destination_cave| destination_cave != cave)
    });
}

fn look_for_path(
    current_path: Vec<String>,
    mut connections: HashMap<String, Vec<String>>,
    mut visited_small_caves: Vec<String>,
    mut visited_twice: bool,
) -> usize {
    let current_cave = current_path.last().unwrap();
    if current_cave == "end" {
        return 1;
    }

    let destinations = if let Some(destinations) = connections.get(current_cave) {
        destinations.clone()
    } else {
        return 0;
    };

    if is_small_cave(current_cave) {
        if !visited_twice && visited_small_caves.contains(current_cave) {
            visited_twice = true;
            for vsc in visited_small_caves.iter() {
                remove_connections_to(vsc, &mut connections);
                connections.remove(vsc);
            }
        }

        if visited_twice || current_cave == "start" {
            remove_connections_to(current_cave, &mut connections);
            connections.remove(current_cave);
        } else {
            visited_small_caves.push(current_cave.clone());
        }
    }

    let mut possible_path_count = 0;

    for destination in destinations {
        let mut my_path = current_path.clone();
        my_path.push(destination.clone());
        possible_path_count += look_for_path(
            my_path.clone(),
            connections.clone(),
            visited_small_caves.clone(),
            visited_twice,
        );
    }

    possible_path_count
}

fn part_1(connections: HashMap<String, Vec<String>>) -> usize {
    look_for_path(vec!["start".into()], connections, Vec::new(), true)
}

fn part_2(connections: HashMap<String, Vec<String>>) -> usize {
    look_for_path(vec!["start".into()], connections, Vec::new(), false)
}

fn main() {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let input = std::fs::read_to_string("input").unwrap();
    for line in input.lines() {
        let mut parts = line.split('-');
        let (from, to) = (parts.next().unwrap(), parts.next().unwrap());

        connections.entry(from.into()).or_default().push(to.into());

        if from != "start" && to != "end" {
            connections.entry(to.into()).or_default().push(from.into());
        }
    }

    assert_eq!(part_1(connections.clone()), 4773);
    assert_eq!(part_2(connections.clone()), 116985);
}
