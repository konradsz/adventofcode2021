use std::collections::HashSet;

type Coordinates = (u32, u32);

fn fold(
    paper: &mut HashSet<Coordinates>,
    at: u32,
    fold_functor: &dyn Fn(&HashSet<Coordinates>, u32) -> (Vec<Coordinates>, Vec<Coordinates>),
) {
    let (to_remove, to_add) = fold_functor(paper, at);

    for el in to_remove {
        paper.remove(&el);
    }
    for el in to_add {
        paper.insert(el);
    }
}

fn fold_along_y(paper: &HashSet<Coordinates>, at: u32) -> (Vec<Coordinates>, Vec<Coordinates>) {
    paper
        .iter()
        .filter(|&&(_, y)| y >= at)
        .map(|&(x, y)| ((x, y), (x, 2 * at - y)))
        .unzip()
}

fn fold_along_x(paper: &HashSet<Coordinates>, at: u32) -> (Vec<Coordinates>, Vec<Coordinates>) {
    paper
        .iter()
        .filter(|&&(x, _)| x >= at)
        .map(|&(x, y)| ((x, y), (2 * at - x, y)))
        .unzip()
}

fn print_paper(paper: &HashSet<Coordinates>) {
    let max_x = paper.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = paper.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut parts = input.split("\n\n");
    let mut paper: HashSet<Coordinates> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    let mut part_1 = None;
    parts.next().unwrap().lines().for_each(|line| {
        let (instruction, at) = line.split_at(13);
        let at = at.parse::<u32>().unwrap();
        if instruction.contains('x') {
            fold(&mut paper, at, &fold_along_x);
        } else if instruction.contains('y') {
            fold(&mut paper, at, &fold_along_y);
        }
        if part_1.is_none() {
            part_1 = Some(paper.len());
        }
    });

    assert_eq!(part_1.unwrap(), 724);

    print_paper(&paper);
}
