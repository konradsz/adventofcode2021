use std::collections::HashMap;

type Image = HashMap<(i32, i32), char>;

fn enhance(enhancement_algorithm: &[char], image: &Image, default: char) -> Image {
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
                        if *field == '#' {
                            result | (1 << (8 - index))
                        } else {
                            result
                        }
                    });

                ((x, y), *enhancement_algorithm.get(number).unwrap())
            })
        })
        .flatten()
        .collect()
}

fn part_1(enhancement_algorithm: &[char], mut image: Image) -> usize {
    let mut default = '.';
    for _ in 0..2 {
        image = enhance(enhancement_algorithm, &image, default);
        if default == '.' {
            default = '#';
        } else {
            default = '.';
        }
    }

    image.values().filter(|&&c| c == '#').count()
}

fn part_2(enhancement_algorithm: &[char], mut image: Image) -> usize {
    let mut default = '.';
    for _ in 0..50 {
        image = enhance(enhancement_algorithm, &image, default);
        if default == '.' {
            default = '#';
        } else {
            default = '.';
        }
    }

    image.values().filter(|&&c| c == '#').count()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut parts = input.split("\n\n");

    let enhancement_algorithm = parts.next().unwrap().chars().collect::<Vec<char>>();
    let image = parts
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    assert_eq!(part_1(&enhancement_algorithm, image.clone()), 5354);
    assert_eq!(part_2(&enhancement_algorithm, image), 18269);
}
