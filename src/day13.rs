use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use itertools::Itertools;

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    let input = parse(input);

    println!("p: {:?}", input);

    input.into_iter().enumerate()
        .map(|(index, (left, right))|{
            println!("Checking {} and {}", left, right);
            if left < right {index+1} else { 0}
        })
        .sum()
}


fn parse(input: &str) -> Vec<(Item, Item)> {
    input.split("\n\n").map(|packets| {
        let mut lines = packets.trim().lines();
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();
        (parse_item(first), parse_item(second))
    }).collect::<Vec<_>>()
}

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Number(u32),
}
impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::List(list) => {
                let items = list.iter().map(|i|format!("{}", i)).join(",");
                write!(f,"[{}]", items)
            }
            Item::Number(n) => write!(f, "{}", n),
        }
    }
}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for Item {

}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::Number(left), Item::Number(right)) => left.partial_cmp(&right),
            (Item::List(left), Item::List(right)) => vec_cmp(left, right),
            (Item::List(left), Item::Number(r)) => left.partial_cmp(&vec![Item::Number(*r)]),
            (Item::Number(left), Item::List(r)) => vec![Item::Number(*left)].partial_cmp(r),
            // (l, r) => todo!("Not implemented for {} and {}", l, r)
        }
    }
}

fn vec_cmp(left: &Vec<Item>, right: &Vec<Item>) -> Option<Ordering> {
    println!("compare {:?} {:?}", left, right);
    let mut i = 0;
    loop {
        let l = left.get(i);
        let r = right.get(i);
        if l.is_none() && r.is_none() {
            return Some(Ordering::Equal)
        }
        if l.is_none() {
            return Some(Ordering::Less)
        }
        let l = l.unwrap();

        if r.is_none() {
            return Some(Ordering::Greater)
        }
        let r = r.unwrap();

        println!("\tcompare {} {}", l, r);
        match l.cmp(r) {
            Ordering::Less => {return Some(Ordering::Less)}
            Ordering::Equal => {
                // they are equal, we should compare the next
                // panic!("\t {} = {}", l, r)
            }
            Ordering::Greater => {return Some(Ordering::Greater)}
        }
        i += 1;
    }
    Some(Ordering::Equal)
}

fn parse_item(str: &str) -> Item {
    assert_eq!(str.chars().collect::<Vec<_>>().get(0).unwrap(), &'[');
    let mut stack = VecDeque::new();
    stack.push_back(Item::List(vec![]));
    let mut digit_buffer = String::new();
    // println!("Parsing {}", str);
    for char in str.chars().skip(1) {
        // println!("\tparsing {} onto {:?}", char, stack);
        match char {
            '[' => {
                stack.push_back(Item::List(vec![]));
                // if let Item::List(l) = &mut root {
                    // stack.back_mut().unwrap().push(Item::List(vec![]));
                // }
            }
            ']' => {
                if !digit_buffer.is_empty() {
                    if let Item::List(l) = stack.back_mut().unwrap() {
                        l.push(Item::Number(digit_buffer.parse::<u32>().unwrap()));
                    }
                    digit_buffer.clear();
                }
                let closed_item = stack.pop_back().unwrap();
                if let Some(Item::List(l)) = stack.back_mut() {
                    l.push(closed_item);
                } else {
                    // we pushed the last one?
                    // println!("Parsed: {}", closed_item);
                    assert_eq!(str, format!("{}", closed_item));
                    return closed_item
                }

            }
            ',' => {
                if !digit_buffer.is_empty() {
                    if let Item::List(l) = stack.back_mut().unwrap() {
                        l.push(Item::Number(digit_buffer.parse::<u32>().unwrap_or_else(|_| panic!("couldn't parse {:?}", digit_buffer))));
                    }
                    digit_buffer.clear();
                }
            }
            c => {
                assert!(c.is_digit(10));
                digit_buffer.push(c);

            }
        }
    }
    panic!("Unreachable code");
    // let item = stack.pop_back().unwrap();
    // println!("Parsed: {}", item);
    // assert_eq!(str, format!("{}", item));
    //
    // item
}

// #[aoc(day13, part2)]
// fn part2(input: &str) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day13.txt");
        assert_ne!(part1(input), 6640);
        assert_eq!(part1(input), 6415);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2022/day13.txt");
    //     assert_eq!(part2(input), 0);
    // }

    #[test]
    fn test_isolated_examples() {
        assert_eq!(part1("[1,1,3,1,1]\n[1,1,5,1,1]"), 1);
        assert_eq!(part1("[]\n[]"), 0);
        assert_eq!(part1("[1]\n[[1,2]]"), 1);
        assert_eq!(part1("[0]\n[[1,2]]"), 1);
        assert_eq!(part1("[2]\n[[1,2]]"), 0);
        assert_eq!(part1("[[1,2]]\n[3]"), 1);
        assert_eq!(part1("[[1],[2,3,4]]\n[[1],4]"), 1);
        assert_eq!(part1("[[],1]\n[[],2]"), 1);
        assert_eq!(part1("[[],2]\n[[],1]"), 0);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#,
        );

        assert_eq!(result, 13)
    }

    // #[test]
    // fn part2_provided_example() {
    //     let result = part2(
    //         r#""#,
    //     );
    //
    //     assert_eq!(result, 0)
    // }
}
