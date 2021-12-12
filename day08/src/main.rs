use std::ops::Sub;

type Entry<'a> = (Vec<Pattern<'a>>, Vec<Pattern<'a>>);

#[derive(Copy, Clone, Default)]
struct Pattern<'a>(&'a str);

impl<'a> Pattern<'a> {
    fn contains(&self, letters: &[char]) -> bool {
        letters
            .iter()
            .all(|c| self.0.chars().any(|pattern_char| pattern_char == *c))
    }
}

impl<'a> Sub<&Pattern<'a>> for &Pattern<'a> {
    type Output = Vec<char>;

    fn sub(self, rhs: &Pattern<'a>) -> Self::Output {
        self.0
            .chars()
            .filter(|lhs_c| rhs.0.chars().all(|rhs_c| rhs_c != *lhs_c))
            .collect()
    }
}

impl<'a> Sub<&Pattern<'a>> for Vec<char> {
    type Output = Vec<char>;

    fn sub(self, rhs: &Pattern<'a>) -> Self::Output {
        self.into_iter()
            .filter(|&item| !rhs.0.chars().any(|c| item == c))
            .collect()
    }
}

impl<'a> From<&Pattern<'a>> for Vec<char> {
    fn from(pattern: &Pattern<'a>) -> Self {
        pattern.0.chars().collect()
    }
}

impl<'a> PartialEq for Pattern<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.chars().count() == other.0.chars().count()
            && self.0.chars().all(|c| other.0.find(|o| o == c).is_some())
    }
}

fn part_1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|entry| {
            entry
                .1
                .iter()
                .filter(|el| {
                    el.0.len() == 2 || el.0.len() == 3 || el.0.len() == 4 || el.0.len() == 7
                })
                .count()
        })
        .sum()
}

fn part_2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|(patterns, digits)| {
            let mut v5 = Vec::new();
            let mut v6 = Vec::new();

            let mut decoded = vec![Pattern::default(); 10];

            for pattern in patterns.iter().cloned() {
                match pattern.0.len() {
                    2 => decoded[1] = pattern,
                    3 => decoded[7] = pattern,
                    4 => decoded[4] = pattern,
                    5 => v5.push(pattern),
                    6 => v6.push(pattern),
                    7 => decoded[8] = pattern,
                    _ => panic!("unexpected pattern"),
                }
            }

            let bottom_left_and_bottom = &decoded[8] - &decoded[7] - &decoded[4];

            for candidate in v5 {
                if candidate.contains(&bottom_left_and_bottom) {
                    decoded[2] = candidate;
                } else if candidate.contains(&Vec::<char>::from(&decoded[1])) {
                    decoded[3] = candidate;
                } else {
                    decoded[5] = candidate;
                }
            }

            for candidate in v6 {
                if !candidate.contains(&bottom_left_and_bottom) {
                    decoded[9] = candidate;
                } else if candidate.contains(&Vec::<char>::from(&decoded[1])) {
                    decoded[0] = candidate;
                } else {
                    decoded[6] = candidate;
                }
            }

            digits
                .iter()
                .enumerate()
                .map(|(shift, &digit)| {
                    decoded
                        .iter()
                        .position(|&f| f == digit)
                        .expect("pattern not decoded")
                        * 10usize.pow((3 - shift) as u32)
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let input: Vec<Entry> = input
        .lines()
        .map(|line| {
            let mut it = line.split(" | ");
            (it.next().unwrap(), it.next().unwrap())
        })
        .map(|(patterns, digits)| {
            (
                patterns.split_whitespace().map(|s| Pattern(s)).collect(),
                digits.split_whitespace().map(|s| Pattern(s)).collect(),
            )
        })
        .collect();

    assert_eq!(part_1(&input), 239);
    assert_eq!(part_2(&input), 946346);
}
