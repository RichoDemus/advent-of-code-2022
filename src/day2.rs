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
    input.into_iter().fold(0, |score, (oppo, strategy)| {
        let score_from_move = match *strategy {
            'X' => 1, // rock
            'Y' => 2, // paper
            'Z' => 3, // scissors
            _ => panic!("wat"),
        };

        // A Rock
        // B paper
        // c scisssors
        let score_from_outcome = match (*oppo, *strategy) {
            ('A', 'Y') => 6,
            ('A', 'Z') => 0,
            ('A', 'X') => 3,
            ('B', 'X') => 0,
            ('B', 'Y') => 3,
            ('B', 'Z') => 6,
            ('C', 'Z') => 3,
            ('C', 'Y') => 0,
            ('C', 'X') => 6,
            (o, s) => panic!("unhanled: {} {}", o, s),
        };

        score + score_from_move + score_from_outcome
    })
}

#[aoc(day2, part2)]
fn part2(input: &[(char, char)]) -> usize {
    input.into_iter().fold(0, |score, (oppo, outcome)| {
        // A Rock
        // B paper
        // c scisssors
        // x lose
        // y draw
        // z win

        let score_from_outcome = match *outcome {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("wat"),
        };

        let score_from_move = if *outcome == 'X' {
            //lose
            if *oppo == 'A' {
                // we do scissors
                3
            } else if *oppo == 'B' {
                // we do rock
                1
            } else {
                // we do paper
                2
            }
        } else if *outcome == 'Y' {
            // draw
            if *oppo == 'A' {
                // we do rock
                1
            } else if *oppo == 'B' {
                // we do paper
                2
            } else {
                // we do scissors
                3
            }
        } else {
            //win
            if *oppo == 'A' {
                // we do paper
                2
            } else if *oppo == 'B' {
                // we do scissors
                3
            } else {
                // we do rock
                1
            }
        };

        score + score_from_move + score_from_outcome
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
