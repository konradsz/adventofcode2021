fn get_neighbours(map: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<u32> {
    let indices = [
        (x.checked_sub(1), y.checked_sub(1)),
        (Some(x), y.checked_sub(1)),
        (Some(x + 1), y.checked_sub(1)),
        (x.checked_sub(1), Some(y)),
        (Some(x + 1), Some(y)),
        (x.checked_sub(1), Some(y + 1)),
        (Some(x), Some(y + 1)),
        (Some(x + 1), Some(y + 1)),
    ];

    indices
        .iter()
        .filter_map(|(x, y)| match (x, y) {
            (Some(x), Some(y)) => get_value(map, *x, *y),
            _ => None,
        })
        .collect()
}

fn get_low_points(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let neighbours = get_neighbours(&map, x, y);
            if neighbours
                .iter()
                .all(|neighbour_value| neighbour_value > value)
            {
                low_points.push((x, y));
            }
        }
    }

    low_points
}

fn get_value(map: &Vec<Vec<u32>>, x: usize, y: usize) -> Option<u32> {
    map.get(y)?.get(x).copied()
}

fn part_1(map: &Vec<Vec<u32>>) -> u32 {
    get_low_points(map)
        .iter()
        .map(|(x, y)| 1 + get_value(map, *x, *y).unwrap())
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("{}", part_1(&map));
}
