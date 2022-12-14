use std::collections::VecDeque;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::day11::Operation::{Mul, MulOld, Plus};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test_div: u128,
    true_throw: usize,
    false_throw: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Plus(u128),
    Mul(u128),
    MulOld,
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.trim().split_ascii_whitespace();
        let op = iter.next().unwrap();
        let num = iter.next().unwrap();

        if num == "old" {
            Ok(MulOld)
        } else if op == "+" {
            Ok(Plus(num.parse().unwrap()))
        } else {
            Ok(Mul(num.parse().unwrap()))
        }
    }
}

fn parse_items(items: &str) -> VecDeque<u128> {
    items
        .trim()
        .split(',')
        .map(|item| item.trim().parse().unwrap())
        .collect()
}

// will return a copy of the monkey but delete the items from the original monkey
fn monkey_copy(monkey: &mut Monkey) -> Monkey {
    let new_monkey = monkey.clone();
    monkey.items.clear();
    new_monkey
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let regex = Lazy::new(|| {
        Regex::new(r"Monkey (\d).*\n.*Starting items:(.*)\n.*Operation: new = old (.*)\n.*Test: divisible by (.*)\n.*If true: throw to monkey (\d)\n.*If false: throw to monkey (\d)").unwrap()
    });

    let mut inspects = vec![];
    let mut monkeys: Vec<Monkey> = regex
        .captures_iter(input)
        .map(|caps| {
            inspects.push(0);
            Monkey {
                items: parse_items(caps.get(2).unwrap().as_str()),
                operation: Operation::try_from(caps.get(3).unwrap().as_str()).unwrap(),
                test_div: caps.get(4).unwrap().as_str().parse().unwrap(),
                true_throw: caps.get(5).unwrap().as_str().parse().unwrap(),
                false_throw: caps.get(6).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    for _round in 1..=20 {
        // println!("Round {}", round);
        for i in 0..monkeys.len() {
            let mut monkey = monkey_copy(monkeys.get_mut(i).unwrap());
            // println!("\t{:?}", monkey);
            while let Some(item) = monkey.items.pop_front() {
                // println!("\t\titem: {}", item);
                *inspects.get_mut(i).unwrap() += 1;
                // Monkey ispects item
                let worry_level = match monkey.operation {
                    Plus(n) => item + n,
                    Mul(n) => item * n,
                    MulOld => item * item,
                };
                // println!("\t\tworry level: {}", worry_level);
                // item not damaged, decrease worry level
                let worry_level = worry_level / 3;
                // println!("\t\tworry level: {}", worry_level);
                if worry_level % monkey.test_div == 0 {
                    // println!("\t\tdivisible by {}, throwing to {}", monkey.test_div, monkey.true_throw);
                    monkeys
                        .get_mut(monkey.true_throw)
                        .unwrap()
                        .items
                        .push_back(worry_level);
                } else {
                    // println!("\t\tnot divisible by {}, throwing to {}", monkey.test_div, monkey.false_throw);
                    monkeys
                        .get_mut(monkey.false_throw)
                        .unwrap()
                        .items
                        .push_back(worry_level);
                }
            }
        }
    }
    inspects.sort_unstable();
    inspects.reverse();
    inspects.first().unwrap() * inspects.get(1).unwrap()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let regex = Lazy::new(|| {
        Regex::new(r"Monkey (\d).*\n.*Starting items:(.*)\n.*Operation: new = old (.*)\n.*Test: divisible by (.*)\n.*If true: throw to monkey (\d)\n.*If false: throw to monkey (\d)").unwrap()
    });

    let mut inspects = vec![];
    let mut monkeys: Vec<Monkey> = regex
        .captures_iter(input)
        .map(|caps| {
            inspects.push(0);
            Monkey {
                items: parse_items(caps.get(2).unwrap().as_str()),
                operation: Operation::try_from(caps.get(3).unwrap().as_str()).unwrap(),
                test_div: caps.get(4).unwrap().as_str().parse().unwrap(),
                true_throw: caps.get(5).unwrap().as_str().parse().unwrap(),
                false_throw: caps.get(6).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let gcd: u128 = monkeys.iter().map(|m| m.test_div).product();

    for _round in 1..=10000 {
        // println!("Round {}", round);
        for i in 0..monkeys.len() {
            let mut monkey = monkey_copy(monkeys.get_mut(i).unwrap());
            // println!("\t{:?}", monkey);
            while let Some(item) = monkey.items.pop_front() {
                // println!("\t\titem: {}", item);
                *inspects.get_mut(i).unwrap() += 1;
                // Monkey ispects item
                let worry_level = match monkey.operation {
                    Plus(n) => item + n,
                    Mul(n) => item * n,
                    MulOld => item * item,
                };
                // println!("\t\tworry level: {}", worry_level);
                // item not damaged, decrease worry level
                let worry_level = worry_level % gcd;
                // println!("\t\tworry level: {}", worry_level);
                if worry_level % monkey.test_div == 0 {
                    // println!("\t\tdivisible by {}, throwing to {}", monkey.test_div, monkey.true_throw);
                    monkeys
                        .get_mut(monkey.true_throw)
                        .unwrap()
                        .items
                        .push_back(worry_level);
                } else {
                    // println!("\t\tnot divisible by {}, throwing to {}", monkey.test_div, monkey.false_throw);
                    monkeys
                        .get_mut(monkey.false_throw)
                        .unwrap()
                        .items
                        .push_back(worry_level);
                }
            }
        }
    }
    inspects.sort_unstable();
    inspects.reverse();
    inspects.first().unwrap() * inspects.get(1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day11.txt");
        assert_eq!(part1(input), 78678);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day11.txt");
        assert_eq!(part2(input), 15333249714);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#,
        );

        assert_eq!(result, 10605)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#,
        );

        assert_eq!(result, 2713310158)
    }
}
