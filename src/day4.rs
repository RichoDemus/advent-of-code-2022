use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Range {
    lower: i32,
    upper: i32,
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let regex = Lazy::new(|| Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap());

    regex
        .captures_iter(input)
        .map(|caps| {
            (
                Range {
                    lower: caps.get(1).unwrap().as_str().parse().unwrap(),
                    upper: caps.get(2).unwrap().as_str().parse().unwrap(),
                },
                Range {
                    lower: caps.get(3).unwrap().as_str().parse().unwrap(),
                    upper: caps.get(4).unwrap().as_str().parse().unwrap(),
                },
            )
        })
        .map(|(first_pair, second_pair): (Range, Range)| {
            if first_pair.lower <= second_pair.lower && first_pair.upper >= second_pair.upper {
                1
            } else if second_pair.lower <= first_pair.lower && second_pair.upper >= first_pair.upper
            {
                1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let regex = Lazy::new(|| Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap());

    regex
        .captures_iter(input)
        .map(|caps| {
            (
                Range {
                    lower: caps.get(1).unwrap().as_str().parse().unwrap(),
                    upper: caps.get(2).unwrap().as_str().parse().unwrap(),
                },
                Range {
                    lower: caps.get(3).unwrap().as_str().parse().unwrap(),
                    upper: caps.get(4).unwrap().as_str().parse().unwrap(),
                },
            )
        })
        .map(|(first_pair, second_pair): (Range, Range)| {
            if first_pair.lower <= second_pair.lower && first_pair.upper >= second_pair.lower {
                1
            } else if second_pair.lower <= first_pair.lower && second_pair.upper >= first_pair.lower
            {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day4.txt");
        assert_eq!(part1(input), 526);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day4.txt");
        assert_ne!(part2(input), 265);
        assert_ne!(part2(input), 752);
        assert_eq!(part2(input), 886);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#,
        );

        assert_eq!(result, 2)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#,
        );

        assert_eq!(result, 4)
    }
}
