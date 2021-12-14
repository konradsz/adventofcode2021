use std::collections::HashMap;

fn process(
    steps: usize,
    insertion_rules: &HashMap<String, [String; 2]>,
    mut counts: HashMap<String, usize>,
    last_char: char,
) -> usize {
    for _ in 0..steps {
        let to_increase = counts
            .iter()
            .filter(|&(_, count)| *count > 0)
            .map(|(el, count)| (el.to_owned(), *count))
            .collect::<Vec<_>>();

        counts.values_mut().for_each(|v| *v = 0);

        for (pair, count) in to_increase {
            for e in insertion_rules.get(&pair).unwrap() {
                *counts.entry(e.clone()).or_default() += count;
            }
        }
    }

    let mut char_counts: HashMap<char, usize> = HashMap::new();
    *char_counts.entry(last_char).or_default() += 1;
    for (pair, count) in counts.iter() {
        *char_counts.entry(pair.chars().next().unwrap()).or_default() += count;
    }

    let min = char_counts.values().min().unwrap();
    let max = char_counts.values().max().unwrap();
    max - min
}

fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();

    let input = std::fs::read_to_string("input").unwrap();
    let mut parts = input.split("\n\n");

    let template = parts.next().unwrap();
    template.as_bytes().windows(2).for_each(|w| {
        *counts
            .entry(String::from_utf8(w.to_vec()).unwrap())
            .or_default() += 1;
    });
    let last_char = template.chars().last().unwrap();

    let insertion_rules: HashMap<String, [String; 2]> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let from = parts.next().unwrap();
            let to = parts.next().unwrap();
            let mut cs = from.chars();
            (
                from.to_string(),
                [
                    format!("{}{}", cs.next().unwrap(), to),
                    format!("{}{}", to, cs.next().unwrap()),
                ],
            )
        })
        .collect();

    let part_1 = process(10, &insertion_rules, counts.clone(), last_char);
    let part_2 = process(40, &insertion_rules, counts, last_char);
    assert_eq!(part_1, 2891);
    assert_eq!(part_2, 4607749009683);
}
