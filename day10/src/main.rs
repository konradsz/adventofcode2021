#[derive(Debug, PartialEq)]
enum BracketType {
    Round,  // ()
    Square, // []
    Curly,  // {}
    Angle,  // <>
}

impl TryFrom<char> for BracketType {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' | ')' => Ok(BracketType::Round),
            '[' | ']' => Ok(BracketType::Square),
            '{' | '}' => Ok(BracketType::Curly),
            '<' | '>' => Ok(BracketType::Angle),
            _ => Err(String::from("unsupported character")),
        }
    }
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut stack: Vec<BracketType> = Vec::new();

            let mut result = 0;
            for bracket in line.chars() {
                match bracket {
                    '(' | '[' | '{' | '<' => {
                        stack.push(bracket.try_into().unwrap());
                    }
                    ')' => {
                        let top = stack.pop();
                        if top.is_none() || top.unwrap() != BracketType::Round {
                            result = 3;
                            break;
                        }
                    }
                    ']' => {
                        let top = stack.pop();
                        if top.is_none() || top.unwrap() != BracketType::Square {
                            result = 57;
                            break;
                        }
                    }
                    '}' => {
                        let top = stack.pop();
                        if top.is_none() || top.unwrap() != BracketType::Curly {
                            result = 1197;
                            break;
                        }
                    }
                    '>' => {
                        let top = stack.pop();
                        if top.is_none() || top.unwrap() != BracketType::Angle {
                            result = 25137;
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }

            result
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    let mut results = input
        .lines()
        .filter_map(|line| {
            let mut stack: Vec<BracketType> = Vec::new();
            for bracket in line.chars() {
                match bracket {
                    '(' | '[' | '{' | '<' => {
                        stack.push(bracket.try_into().unwrap());
                    }
                    ')' => {
                        if stack.pop()? != BracketType::Round {
                            return None;
                        }
                    }
                    ']' => {
                        if stack.pop()? != BracketType::Square {
                            return None;
                        }
                    }
                    '}' => {
                        if stack.pop()? != BracketType::Curly {
                            return None;
                        }
                    }
                    '>' => {
                        if stack.pop()? != BracketType::Angle {
                            return None;
                        }
                    }
                    _ => panic!(),
                }
            }

            Some(calculate_score(&stack))
        })
        .collect::<Vec<_>>();

    results.sort_unstable();
    results[results.len() / 2]
}

fn calculate_score(stack: &[BracketType]) -> usize {
    stack.iter().rev().fold(0, |result, bracket| match bracket {
        BracketType::Round => result * 5 + 1,
        BracketType::Square => result * 5 + 2,
        BracketType::Curly => result * 5 + 3,
        BracketType::Angle => result * 5 + 4,
    })
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    assert_eq!(part_1(&input), 243939);
    assert_eq!(part_2(&input), 2421222841);
}
