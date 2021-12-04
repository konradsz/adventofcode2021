use anyhow::{anyhow, Result};

const GRID_SIZE: usize = 5;

#[derive(PartialEq)]
enum Field {
    Unmarked(u32),
    Marked,
}

struct Board {
    fields: Vec<Field>,
    is_completed: bool,
}

impl FromIterator<Field> for Board {
    fn from_iter<I: IntoIterator<Item = Field>>(iter: I) -> Self {
        Self {
            fields: Vec::from_iter(iter),
            is_completed: false,
        }
    }
}

impl Board {
    fn mark_number(&mut self, number: u32) {
        self.fields.iter_mut().for_each(|field| {
            if field == &Field::Unmarked(number) {
                *field = Field::Marked;
            }
        })
    }

    fn check_if_won_this_turn(&mut self) -> bool {
        !self.is_completed && (self.is_any_row_marked() || self.is_any_column_marked())
    }

    fn mark_as_completed(&mut self) {
        self.is_completed = true;
    }

    fn is_any_row_marked(&self) -> bool {
        self.fields
            .chunks(GRID_SIZE)
            .any(|row| row.iter().all(|field| matches!(field, Field::Marked)))
    }

    fn is_any_column_marked(&self) -> bool {
        (0..GRID_SIZE).any(|column| {
            self.fields
                .iter()
                .skip(column)
                .step_by(GRID_SIZE)
                .all(|field| matches!(field, Field::Marked))
        })
    }

    fn calculate_score(&self) -> u32 {
        self.fields
            .iter()
            .filter_map(|field| match field {
                Field::Unmarked(number) => Some(number),
                Field::Marked => None,
            })
            .sum::<u32>()
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input")?;

    let drawn_numbers = input
        .lines()
        .next()
        .ok_or(anyhow!("incorrect input"))?
        .split(',')
        .map(|number| -> Result<u32> { Ok(number.parse::<u32>()?) })
        .collect::<Result<Vec<_>>>()?;

    let elements = input
        .lines()
        .skip(2)
        .map(|line| {
            line.split_whitespace()
                .map(|el| -> Result<u32> { Ok(el.parse::<u32>()?) })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Result<Vec<_>>>()?;

    let mut boards = elements
        .chunks(GRID_SIZE * GRID_SIZE)
        .map(|chunk| chunk.iter().map(|f| Field::Unmarked(*f)).collect())
        .collect::<Vec<Board>>();

    let mut first_winning_score = None;
    let mut last_winning_score = None;
    for number in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_number(number);

            if board.check_if_won_this_turn() {
                board.mark_as_completed();
                let score = board.calculate_score() * number;
                if first_winning_score.is_none() {
                    first_winning_score = Some(score);
                }
                last_winning_score = Some(score);
            }
        }
    }

    assert_eq!(first_winning_score.expect("there is no winner"), 69579); // part 1
    assert_eq!(last_winning_score.expect("there is no winner"), 14877); // part 2

    Ok(())
}
