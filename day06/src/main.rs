use std::collections::HashMap;

const CYCLE_LENGTH: usize = 7;
const INCUBATION: u32 = 8;

struct Fish {
    birthday: u32,
    counter: u32,
}

impl Fish {
    fn initial(counter: u32) -> Self {
        Self {
            birthday: 0,
            counter: counter + 1,
        }
    }

    fn born(birthday: u32) -> Self {
        Self {
            birthday,
            counter: INCUBATION + 1,
        }
    }

    fn get_children(&self, end: u32) -> impl Iterator<Item = Fish> {
        ((self.birthday + self.counter)..)
            .step_by(CYCLE_LENGTH)
            .take_while(move |day| *day <= end)
            .map(|birthday| Fish::born(birthday))
    }
}

fn populate(fishes: impl Iterator<Item = Fish>, end: u32, sum: &mut usize) {
    for fish in fishes {
        *sum += 1;
        populate(fish.get_children(end), end, sum);
    }
}

fn populate_groups(fish_groups: &HashMap<u32, usize>, end: u32) -> usize {
    fish_groups
        .iter()
        .map(|(fish, count)| {
            let mut sum = 0;
            populate(std::iter::once(Fish::initial(*fish)), end, &mut sum);
            count * sum
        })
        .sum()
}

fn part_1(fish_groups: &HashMap<u32, usize>) -> usize {
    populate_groups(fish_groups, 80)
}

fn part_2(fish_groups: &HashMap<u32, usize>) -> usize {
    populate_groups(fish_groups, 256)
}

fn main() {
    let initial = [
        5, 1, 2, 1, 5, 3, 1, 1, 1, 1, 1, 2, 5, 4, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, 5, 1,
        1, 1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 1, 4, 3, 1, 1, 4, 1, 1, 1, 1, 2, 1, 1, 1, 5, 1, 1, 5, 1,
        1, 1, 4, 4, 2, 5, 1, 1, 5, 1, 1, 2, 2, 1, 2, 1, 1, 5, 3, 1, 2, 1, 1, 3, 1, 4, 3, 3, 1, 1,
        3, 1, 5, 1, 1, 3, 1, 1, 4, 4, 1, 1, 1, 5, 1, 1, 1, 4, 4, 1, 3, 1, 4, 1, 1, 4, 5, 1, 1, 1,
        4, 3, 1, 4, 1, 1, 4, 4, 3, 5, 1, 2, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 4, 1, 1, 3, 1, 1, 2,
        1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 5, 5, 1, 1, 1, 5, 1, 1, 1, 1, 5, 1, 3, 2, 1, 1,
        5, 2, 3, 1, 2, 2, 2, 5, 1, 1, 3, 1, 1, 1, 5, 1, 4, 1, 1, 1, 3, 2, 1, 3, 3, 1, 3, 1, 1, 1,
        1, 1, 1, 1, 2, 3, 1, 5, 1, 4, 1, 3, 5, 1, 1, 1, 2, 2, 1, 1, 1, 1, 5, 4, 1, 1, 3, 1, 2, 4,
        2, 1, 1, 3, 5, 1, 1, 1, 3, 1, 1, 1, 5, 1, 1, 1, 1, 1, 3, 1, 1, 1, 4, 1, 1, 1, 1, 2, 2, 1,
        1, 1, 1, 5, 3, 1, 2, 3, 4, 1, 1, 5, 1, 2, 4, 2, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 4, 1, 5,
    ];

    let mut groups: HashMap<u32, usize> = HashMap::new();
    for fish in initial {
        *groups.entry(fish).or_default() += 1;
    }

    assert_eq!(part_1(&groups), 383160);
    assert_eq!(part_2(&groups), 1721148811504);
}
