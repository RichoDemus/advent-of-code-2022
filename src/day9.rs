use std::collections::HashSet;

use crate::day9::Direction::{Down, Left, Right, Up};

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut tail_visits: HashSet<(i32, i32)> = HashSet::new();
    let mut head_x = 1000;
    let mut head_y = 1000;
    let mut tail_x = 1000;
    let mut tail_y = 1000;
    for line in input.lines() {
        // println!("{}", line);
        let (direction, steps) = directions(line);
        for _ in 0..steps {
            match direction {
                Up => head_y += 1,
                Down => head_y -= 1,
                Left => head_x -= 1,
                Right => head_x += 1,
            };
            (tail_x, tail_y) = calc_new_tail_position(head_x, head_y, tail_x, tail_y);
            // println!(
            //     "\tStep, now at {},{}, tail {},{}",
            //     head_x, head_y, tail_x, tail_y
            // );
            // for y in ((head_y - 4)..(head_y + 4)).rev() {
            //     for x in (head_x - 4)..(head_x + 4) {
            //         if head_x == x && head_y == y {
            //             print!("H");
            //         } else if tail_x == x && tail_y == y {
            //             print!("T")
            //         } else {
            //             print!(".")
            //         }
            //     }
            //     println!()
            // }
            // println!("\n------------------------------------");
            tail_visits.insert((tail_x, tail_y));
            assert!(head_x > -1);
            assert!(head_y > -1);
            assert!(tail_x > -1);
            assert!(tail_y > -1);
            assert!(distance(head_x, head_y, tail_x, tail_y) < 2);
        }
    }

    tail_visits.len()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let mut tail_visits: HashSet<(i32, i32)> = HashSet::new();
    let mut head_x = 0;
    let mut tail = vec![];
    for _ in 0..9 {
        tail.push((0, 0));
    }
    let mut head_y = 0;
    for line in input.lines() {
        // println!("{}", line);
        let (direction, steps) = directions(line);
        for _ in 0..steps {
            match direction {
                Up => head_y += 1,
                Down => head_y -= 1,
                Left => head_x -= 1,
                Right => head_x += 1,
            };
            for i in 0..9 {
                let (x_to_follow, y_to_follow) = if i == 0 {
                    (head_x, head_y)
                } else {
                    *tail.get(i - 1).unwrap()
                };
                let (x, y) = tail.get(i).unwrap();
                let (new_x, new_y) = calc_new_tail_position(x_to_follow, y_to_follow, *x, *y);
                *tail.get_mut(i).unwrap() = (new_x, new_y);
            }
            // println!("\tStep, now at {},{}", head_x, head_y);

            // let tail_set = tail.iter().copied().collect::<HashSet<(_,_)>>();
            // const area: i32 = 10;
            // for y in ((head_y-area)..(head_y+area)).rev() {
            //     for x in (head_x-area)..(head_x+area) {
            //         if head_x == x && head_y == y {
            //             print!("H");
            //         } else if tail_set.contains(&(x,y)) {
            //             print!("T")
            //         } else {
            //             print!(".")
            //         }
            //     }
            //     println!()
            // }
            // println!("\n------------------------------------");
            tail_visits.insert(*tail.get(8).unwrap());
            // for (x,y) in &tail{
            //     // assert!([
            //     //             (0,0),
            //     //             (1,0),
            //     //             (2,0),
            //     //             (3,0),
            //     //             (4,0),
            //     //         ].contains(&(*x,*y)), "shouldn't add {},{}", x,y);
            //     tail_visits.insert((*x,*y));
            // }
            // assert!(head_x > -1);
            // assert!(head_y > -1);
            // assert!(tail_x > -1);
            // assert!(tail_y > -1);
            // assert!(distance(head_x, head_y, tail_x, tail_y) < 2);
        }
    }

    // println!("result: {:?}", tail_visits);
    // let x_min = *tail_visits.iter().map(|(x,y)|x).min().unwrap();
    // let y_min = *tail_visits.iter().map(|(x,y)|y).min().unwrap();
    // let x_max = *tail_visits.iter().map(|(x,y)|x).max().unwrap();
    // let y_max = *tail_visits.iter().map(|(x,y)|y).max().unwrap();
    // for y in (y_min..=y_max).rev() {
    //     for x in x_min..=x_max {
    //         if tail_visits.contains(&(x, y)) {
    //             print!("#")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!()
    // }
    tail_visits.len()
}

fn calc_new_tail_position(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    if distance(head_x, head_y, tail_x, tail_y) < 2 {
        (tail_x, tail_y)
    } else if head_x == tail_x {
        if head_y > tail_y {
            (tail_x, tail_y + 1)
        } else {
            (tail_x, tail_y - 1)
        }
    } else if head_y == tail_y {
        if head_x > tail_x {
            (tail_x + 1, tail_y)
        } else {
            (tail_x - 1, tail_y)
        }
    } else {
        // diagonal
        if head_x > tail_x && head_y > tail_y {
            // UpRight
            (tail_x + 1, tail_y + 1)
        } else if head_x > tail_x && head_y < tail_y {
            // UpLeft
            (tail_x + 1, tail_y - 1)
        } else if head_x < tail_x && head_y > tail_y {
            // DownRight
            (tail_x - 1, tail_y + 1)
        } else {
            // DownLeft
            (tail_x - 1, tail_y - 1)
        }
    }
}

fn distance(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> i32 {
    (head_x - tail_x).abs().max((head_y - tail_y).abs())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn directions(line: &str) -> (Direction, usize) {
    let mut split = line.split_ascii_whitespace();
    let direction = split.next().unwrap();
    let steps = split.next().unwrap();
    let direction = match direction {
        "U" => Up,
        "D" => Down,
        "L" => Left,
        "R" => Right,
        _ => panic!(),
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

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day9.txt");
        assert_eq!(part2(input), 2578);
    }

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

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
        );

        assert_eq!(result, 1)
    }

    #[test]
    fn part2_provided_example2() {
        let result = part2(
            r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
        );

        assert_eq!(result, 36)
    }
}
