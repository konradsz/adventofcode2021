use std::collections::HashMap;

type Image = HashMap<(i32, i32), u32>;

fn enhance(enhancement_algorithm: &[u32], image: &Image, default: u32) -> Image {
    let min = image.keys().min_by(|lhs, rhs| lhs.0.cmp(&rhs.0)).unwrap().0 - 3;
    let max = image.keys().max_by(|lhs, rhs| lhs.0.cmp(&rhs.0)).unwrap().0 + 3;

    (min..=max)
        .map(|x| {
            (min..=max).map(move |y| {
                let coords = [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ];
                let number = coords
                    .iter()
                    .enumerate()
                    .fold(0, |result, (index, &(x, y))| {
                        let field = image.get(&(x, y)).unwrap_or(&default);
                        result | ((*field as usize) << 8 - index)
                    });

                ((x, y), *enhancement_algorithm.get(number).unwrap())
            })
        })
        .flatten()
        .collect()
}

fn part_1(enhancement_algorithm: &[u32], mut image: Image) -> u32 {
    let mut default = 0;
    for _ in 0..2 {
        image = enhance(enhancement_algorithm, &image, default);
        default = 1 - default;
    }

    image.values().sum()
}

fn part_2(enhancement_algorithm: &[u32], mut image: Image) -> u32 {
    let mut default = 0;
    for _ in 0..50 {
        image = enhance(enhancement_algorithm, &image, default);
        default = 1 - default;
    }

    image.values().sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut parts = input.split("\n\n");

    let enhancement_algorithm = parts
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("unexpected character"),
        })
        .collect::<Vec<_>>();
    let image = parts
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    (x as i32, y as i32),
                    match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!("unexpected character"),
                    },
                )
            })
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    assert_eq!(part_1(&enhancement_algorithm, image.clone()), 5354);
    assert_eq!(part_2(&enhancement_algorithm, image), 18269);
}
