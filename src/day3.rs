use std::collections::VecDeque;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let input = input
        .split_ascii_whitespace()
        .map(|backpack| {
            let (first, second) = backpack.split_at(backpack.len() / 2);
            (first.chars().collect(), second.chars().collect())
        })
        .collect::<Vec<(Vec<char>, Vec<char>)>>();

    input
        .iter()
        .cloned()
        .map(|(left_compartment, right_compartment)| {
            for c in (65..=90).chain(97..=122) {
                let c = std::char::from_u32(c).unwrap();
                if left_compartment.contains(&c) && right_compartment.contains(&c) {
                    return priority(c);
                }
            }
            panic!("wat")
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let mut groups = vec![];

    let mut input = VecDeque::from(input.split_ascii_whitespace().collect::<Vec<_>>());
    while !input.is_empty() {
        groups.push((
            input.pop_front().unwrap(),
            input.pop_front().unwrap(),
            input.pop_front().unwrap(),
        ));
    }

    groups
        .iter()
        .map(|(b1, b2, b3)| {
            for c in (65..=90).chain(97..=122) {
                let c = std::char::from_u32(c).unwrap();
                if b1.contains(c) && b2.contains(c) && b3.contains(c) {
                    return priority(c);
                }
            }
            panic!("wat")
        })
        .sum()
}

const fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day3.txt");
        assert_eq!(part1(input), 7821);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day3.txt");
        assert_eq!(part2(input), 2752);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#,
        );

        assert_eq!(result, 157)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#,
        );

        assert_eq!(result, 70)
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('L'), 38);
        assert_eq!(priority('P'), 42);
        assert_eq!(priority('v'), 22);
        assert_eq!(priority('t'), 20);
        assert_eq!(priority('s'), 19);
    }
}
