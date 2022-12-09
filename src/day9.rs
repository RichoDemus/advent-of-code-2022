use std::collections::HashSet;
use crate::day9::Direction::{Down, Left, Right, Up};

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut tail_visits: HashSet<(i32, i32)> = HashSet::new();
    let mut headX = 1000;
    let mut headY = 1000;
    let mut tailX = 1000;
    let mut tailY = 1000;
    for line in input.lines() {
        println!("{}", line);
        let (direction, steps) = directions(line);
        for i in 0..steps {
            match direction {
                Up => headY +=1,
                Down => headY -= 1,
                Left => headX -= 1,
                Right => headX += 1,
            };
            (tailX, tailY) = calc_new_tail_position(headX, headY, tailX, tailY);
            println!("\tStep, now at {},{}, tail {},{}", headX, headY, tailX, tailY);
            for y in ((headY-4)..(headY+4)).rev() {
                for x in (headX-4)..(headX+4) {
                    if headX == x && headY == y {
                        print!("H");
                    } else if tailX == x && tailY == y {
                        print!("T")
                    } else {
                        print!(".")
                    }
                }
                println!()
            }
            println!("\n------------------------------------");
            tail_visits.insert((tailX, tailY));
            assert!(headX > -1);
            assert!(headY > -1);
            assert!(tailX > -1);
            assert!(tailY > -1);
            assert!(distance(headX, headY, tailX, tailY) < 2);
        }
    }

    tail_visits.len()
}

fn calc_new_tail_position(headX:i32, headY:i32, tailX:i32, tailY:i32) -> (i32, i32) {
    if distance(headX, headY, tailX, tailY) < 2 {
        (tailX, tailY)
    } else if headX == tailX {
        if headY > tailY {
            (tailX, tailY + 1)
        } else {
            (tailX, tailY - 1)
        }
    } else if headY == tailY {
        if headX > tailX {
            (tailX + 1, tailY)
        } else {
            (tailX - 1, tailY)
        }
    } else {
        // diagonal
        if headX > tailX && headY > tailY {
            // UpRight
            (tailX + 1, tailY + 1)
        } else if headX > tailX && headY < tailY {
            // UpLeft
            (tailX + 1, tailY - 1)
        } else if headX < tailX && headY > tailY {
            // DownRight
            (tailX -1 , tailY + 1)
        } else {
            // DownLeft
            (tailX -1 , tailY - 1)
        }
    }
}

fn distance(headX:i32, headY:i32, tailX:i32, tailY:i32) -> i32 {
    (headX - tailX).abs().max((headY - tailY).abs())
}

enum Direction {
    Up, Down, Left, Right
}
fn directions(line:&str) -> (Direction, usize) {
    let mut split = line.split_ascii_whitespace();
    let direction = split.next().unwrap();
    let steps = split.next().unwrap();
    let direction = match direction {
        "U" => Up,
        "D" => Down,
        "L" => Left,
        "R" => Right,
        _ => panic!()
    };
    let steps = steps.parse::<usize>().unwrap();
    (direction, steps)
}

// #[aoc(day9, part2)]
// fn part2(input: &str) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day9.txt");
        assert_eq!(part1(input), 6175);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2022/day9.txt");
    //     assert_eq!(part2(input), 0);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
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