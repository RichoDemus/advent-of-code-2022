use std::collections::VecDeque;

#[aoc(day10, part1)]
fn part1(input: &str) -> i64 {
    let mut result = 0;
    let mut cycle = 0;
    let mut register = 1;
    for line in input.lines() {
        println!("cycle: {}, reg: {}, line: {}>", cycle, register, line);
        if line.contains("noop") {
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                println!("1cycle {} reg is {} fpr {}", cycle, register, register*cycle);
                result += cycle * register;
            }
        } else if line.contains("addx") {
            let mut split = line.split_ascii_whitespace();
            split.next();
            let num = split.next().unwrap();
            let num: i64 = num.parse().unwrap();
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                println!("2cycle {} reg is {} fpr {}", cycle, register, register*cycle);
                result += cycle * register;
            }
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                println!("3cycle {} reg is {} fpr {}", cycle, register, register*cycle);
                result += cycle * register;
            }
            register += num;
        } else {
            panic!()
        }
    }
    result
}

#[derive(Debug)]
enum Op{
    Noop, Addx, Addx2(i64),
}

#[aoc(day10, part1,cycle)]
fn part1_alt(input: &str) -> i64 {
    let mut lines = input.lines().collect::<VecDeque<_>>();
    let mut op_buffer = VecDeque::new();
    let mut cycle = 1;
    let mut register = 1;
    let mut result = 0;
    loop {
        // println!("Cycle {}, reg: {}, buff: {:?}", cycle, register, op_buffer);
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            // println!("cycle {} reg is {} fpr {}", cycle, register, register*cycle);
            result += cycle * register;
        }
        if op_buffer.is_empty() {
            let option1 = lines.pop_front();
            // println!("\t\tnext op: {:?}", option1);
            match option1 {
                None => {
                    break;
                }
                Some(op) => {
                    if op.contains("noop") {
                        // println!("\t\tAdding noop");
                        op_buffer.push_back(Op::Noop);
                    } else if op.contains("addx ") {
                        let mut split = op.split_ascii_whitespace();
                        split.next();
                        let num = split.next().unwrap();
                        let num: i64 = num.parse().unwrap();
                        // println!("\t\tAdding Addx({})", num);
                        op_buffer.push_back(Op::Addx);
                        op_buffer.push_back(Op::Addx2(num));
                    }
                }
            }
        }
        let option = op_buffer.pop_front();
        // println!("\tOp: {:?}", option);
        match option {
            Some(Op::Noop) | Some(Op::Addx) => {
            // noop
            },
            Some(Op::Addx2(add)) => {
                register += add;
            },
            None => {
            }
        };

        cycle += 1;
    }

    result
}
// #[aoc(day10, part2)]
// fn part2(input: &str) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day10.txt");
        // assert_eq!(part1(input), 13920);
        assert_eq!(part1(input), 13920);
        assert_eq!(part1_alt(input), 13920);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2022/day10.txt");
    //     assert_eq!(part2(input), 0);
    // }

    #[test]
    fn part1_small_example() {
        let input = r#"noop
addx 3
addx -5"#;
    part1_alt(input);
    }

    #[test]
    fn part1_provided_example() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        assert_eq!(part1(input, ), 13140);
        assert_eq!(part1_alt(input, ), 13140);
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
