#[derive(Eq, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            e => Err(format!("unexpected value: {}", e)),
        }
    }
}

impl Move {
    const fn score(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    const fn beats(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
    const fn beaten_by(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            e => Err(format!("unexpected value: {}", e)),
        }
    }
}

impl Outcome {
    const fn score(self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

fn calc_outcome(my_move: Move, opponent_move: Move) -> Outcome {
    if my_move == opponent_move {
        Outcome::Draw
    } else if my_move.beats() == opponent_move {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

fn calc_move(opponent_move: Move, desired_outcome: Outcome) -> Move {
    if desired_outcome == Outcome::Draw {
        opponent_move
    } else if desired_outcome == Outcome::Win {
        opponent_move.beaten_by()
    } else {
        opponent_move.beats()
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let split = line.split_ascii_whitespace().collect::<Vec<_>>();
            (
                split[0].chars().next().unwrap(),
                split[1].chars().next().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(char, char)]) -> usize {
    input
        .iter()
        .map(|(opponent, strat)| {
            (
                Move::try_from(*opponent).unwrap(),
                Move::try_from(*strat).unwrap(),
            )
        })
        .fold(0, |score, (opponent_move, my_move)| {
            let outcome = calc_outcome(my_move, opponent_move);
            score + my_move.score() + outcome.score()
        })
}

#[aoc(day2, part2)]
fn part2(input: &[(char, char)]) -> usize {
    input
        .iter()
        .map(|(opponent, outcome)| {
            (
                Move::try_from(*opponent).unwrap(),
                Outcome::try_from(*outcome).unwrap(),
            )
        })
        .fold(0, |score, (opponent_move, outcome)| {
            let my_move = calc_move(opponent_move, outcome);
            score + my_move.score() + outcome.score()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day2.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 12586);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day2.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 13193);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"A Y
B X
C Z"#,
        ));

        assert_eq!(result, 15)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"A Y
B X
C Z"#,
        ));

        assert_eq!(result, 12)
    }
}
