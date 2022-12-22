use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use itertools::Itertools;

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    let packets = input.split("\n\n").map(|packets| {
        let mut lines = packets.trim().lines();
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();
        (parse(first), parse(second))
    }).collect::<Vec<_>>();

    println!("p: {:?}", packets);

    todo!()
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

fn parse(str: &str) -> Item {
    assert_eq!(str.chars().collect::<Vec<_>>().get(0).unwrap(), &'[');
    let mut stack = VecDeque::new();
    stack.push_back(Item::List(vec![]));
    let mut digit_buffer = String::new();
    println!("Parsing {}", str);
    for char in str.chars().skip(1) {
        println!("\tparsing {} onto {:?}", char, stack);
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
                    println!("Parsed: {}", closed_item);
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
    let item = stack.pop_back().unwrap();
    println!("Parsed: {}", item);
    assert_eq!(str, format!("{}", item));

    item
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
        assert_eq!(part1(input), 0);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2022/day13.txt");
    //     assert_eq!(part2(input), 0);
    // }

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
