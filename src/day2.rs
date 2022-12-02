#[derive(Eq, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Move {
    fn from(mov: char) -> Self {
        match mov {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("asd"),
        }
    }
}

impl Move {
    fn score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn beats(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
    fn beaten_by(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl From<char> for Outcome {
    fn from(mov: char) -> Self {
        match mov {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("asd"),
        }
    }
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn calc_outcome(my_move: Move, opponent_move: Move) -> Outcome {
    if my_move == opponent_move {
        return Outcome::Draw;
    }
    match (my_move, opponent_move) {
        (Move::Rock, Move::Scissors) => Outcome::Win,
        (Move::Rock, Move::Paper) => Outcome::Lose,
        (Move::Paper, Move::Scissors) => Outcome::Lose,
        (Move::Paper, Move::Rock) => Outcome::Win,
        (Move::Scissors, Move::Rock) => Outcome::Lose,
        (Move::Scissors, Move::Paper) => Outcome::Win,
        _ => panic!(),
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
        .map(|(opponent, strat)| (Move::from(*opponent), Move::from(*strat)))
        .fold(0, |score, (opponent_move, my_move)| {
            let outcome = calc_outcome(my_move, opponent_move);
            score + my_move.score() + outcome.score()
        })
}

#[aoc(day2, part2)]
fn part2(input: &[(char, char)]) -> usize {
    input
        .iter()
        .map(|(opponent, outcome)| (Move::from(*opponent), Outcome::from(*outcome)))
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
