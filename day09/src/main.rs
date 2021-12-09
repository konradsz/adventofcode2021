use std::collections::BinaryHeap;

struct Map(Vec<Vec<u32>>);

impl Map {
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<u32> {
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
                (Some(x), Some(y)) => self.get_value(*x, *y),
                _ => None,
            })
            .collect()
    }

    fn get_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points = Vec::new();
        for (y, row) in self.0.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if self
                    .get_neighbours(x, y)
                    .iter()
                    .all(|neighbour_value| neighbour_value > value)
                {
                    low_points.push((x, y));
                }
            }
        }

        low_points
    }

    fn get_value(&self, x: usize, y: usize) -> Option<u32> {
        self.0.get(y)?.get(x).copied()
    }

    fn extend_basin_for_point(&self, x: usize, y: usize, basin: &mut Vec<(usize, usize)>) {
        let candidate_indices = [
            (Some(x), y.checked_sub(1)),
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), Some(y + 1)),
        ];

        candidate_indices
            .iter()
            .filter_map(|&(x, y)| match (x, y) {
                (Some(x), Some(y)) => {
                    self.get_value(x, y)
                        .map(|value| if value < 9 { Some((x, y)) } else { None })?
                }
                _ => None,
            })
            .for_each(|(x, y)| {
                if !basin.contains(&(x, y)) {
                    basin.push((x, y));

                    self.extend_basin_for_point(x, y, basin);
                }
            });
    }
}

fn part_1(map: &Map) -> usize {
    map.get_low_points()
        .iter()
        .map(|(x, y)| 1 + map.get_value(*x, *y).unwrap() as usize)
        .sum()
}

fn part_2(map: &Map) -> usize {
    let mut basins = map
        .get_low_points()
        .iter()
        .map(|&(x, y)| {
            let mut basin = Vec::new();
            map.extend_basin_for_point(x, y, &mut basin);
            basin.len()
        })
        .collect::<BinaryHeap<usize>>();

    basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let map = Map(input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect());

    assert_eq!(part_1(&map), 504);
    assert_eq!(part_2(&map), 1558722);
}
