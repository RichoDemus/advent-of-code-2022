use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct Order{
    amount:usize,
    source:usize,
    destination:usize,
}

impl From<&str> for Order {
    fn from(_: &str) -> Self {
        todo!()
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let chunks = input.split("\n\n").collect::<Vec<_>>();
    let stacks = chunks[0];
    let orders = chunks[1];
    // println!("stacks:\n{}, orders:\n{}", stacks, orders);

    let mut stacks = {
        let columns = stacks.trim().chars().last().unwrap().to_digit(10).unwrap() as usize;
        let stacks = stacks.split("\n").collect::<Vec<_>>();
        let mut result = vec![];
        result.push(vec![]); //lets push in a 0 for now, fix later
        // println!("columns: {}", columns);
        for column in 0..columns {
            // print!("column {}", column+1);
            result.push(vec![]);
            for row in (0..stacks.len()-1).rev() {
                let option = stacks[row].chars().collect::<Vec<_>>().get(column * 4 + 1).cloned();
                // print!(" ({},{}): '{:?}'", column, row, option);
                if let Some(c) = option {
                    if !c.is_whitespace() {
                        result.get_mut(column+1).unwrap().push(c);
                    }
                }

            }
            // println!()
        }
        // for (i,col) in result.iter().enumerate() {
        //     println!("col:{} {:?}", i, col);
        // }



        // stacks.split("\n")
        //     .map(|line|{
        //         // replace 3 spaces with [ ]
        //         let line = line.replace("   ","[ ]");
        //         println!("{} l:{}", line, line.len());
        //         for i in 0..=(line.len()/4) {
        //             let lower = i*4;
        //             let upper = i*4+4;
        //             print!("'{}'", &line[lower..upper]);
        //         }
        //         println!();
        //     }).collect::<Vec<_>>()
        result
    };

    let orders = {
        let regex = Lazy::new(|| Regex::new(r"move (\d*) from (\d) to (\d)").unwrap());

        let orders = regex
            .captures_iter(input)
            .map(|caps| {
                Order {
                    amount: caps.get(1).unwrap().as_str().parse().unwrap(),
                    source: caps.get(2).unwrap().as_str().parse().unwrap(),
                    destination: caps.get(3).unwrap().as_str().parse().unwrap(),
                }
            }).collect::<Vec<_>>();
        // println!("{:?}", orders);
        orders
    };

    for order in orders {
        println!("{:?}", order);
        for _op in 0..order.amount {
            let source_stack = stacks.get_mut(order.source).unwrap();
            if source_stack.is_empty() {
                continue
            }
            let crate_in_hand = source_stack.remove(source_stack.len()-1);
            let destination_stack = stacks.get_mut(order.destination).unwrap();
            destination_stack.push(crate_in_hand);
        }
        for (i,col) in stacks.iter().enumerate() {
            println!("col:{} {:?}", i, col);
        }
    }
    let mut result = vec![];
    for (i,col) in stacks.iter().enumerate() {
        if i == 0 {
            continue
        }
        println!("col:{} {:?}", i, col);
        result.push(match col.last() {
            None => ' ',
            Some(c) => *c,
        });
    }
    println!("{:?}", result);
    result.into_iter().collect()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    let chunks = input.split("\n\n").collect::<Vec<_>>();
    let stacks = chunks[0];
    let orders = chunks[1];
    // println!("stacks:\n{}, orders:\n{}", stacks, orders);

    let mut stacks = {
        let columns = stacks.trim().chars().last().unwrap().to_digit(10).unwrap() as usize;
        let stacks = stacks.split("\n").collect::<Vec<_>>();
        let mut result = vec![];
        result.push(vec![]); //lets push in a 0 for now, fix later
        // println!("columns: {}", columns);
        for column in 0..columns {
            // print!("column {}", column+1);
            result.push(vec![]);
            for row in (0..stacks.len()-1).rev() {
                let option = stacks[row].chars().collect::<Vec<_>>().get(column * 4 + 1).cloned();
                // print!(" ({},{}): '{:?}'", column, row, option);
                if let Some(c) = option {
                    if !c.is_whitespace() {
                        result.get_mut(column+1).unwrap().push(c);
                    }
                }

            }
            // println!()
        }
        // for (i,col) in result.iter().enumerate() {
        //     println!("col:{} {:?}", i, col);
        // }



        // stacks.split("\n")
        //     .map(|line|{
        //         // replace 3 spaces with [ ]
        //         let line = line.replace("   ","[ ]");
        //         println!("{} l:{}", line, line.len());
        //         for i in 0..=(line.len()/4) {
        //             let lower = i*4;
        //             let upper = i*4+4;
        //             print!("'{}'", &line[lower..upper]);
        //         }
        //         println!();
        //     }).collect::<Vec<_>>()
        result
    };

    let orders = {
        let regex = Lazy::new(|| Regex::new(r"move (\d*) from (\d) to (\d)").unwrap());

        let orders = regex
            .captures_iter(input)
            .map(|caps| {
                Order {
                    amount: caps.get(1).unwrap().as_str().parse().unwrap(),
                    source: caps.get(2).unwrap().as_str().parse().unwrap(),
                    destination: caps.get(3).unwrap().as_str().parse().unwrap(),
                }
            }).collect::<Vec<_>>();
        // println!("{:?}", orders);
        orders
    };

    for order in orders {
        println!("{:?}", order);
        let mut buffer = vec![];
        for _op in 0..order.amount {
            let source_stack = stacks.get_mut(order.source).unwrap();
            if source_stack.is_empty() {
                continue
            }
            let crate_in_hand = source_stack.remove(source_stack.len()-1);
            buffer.push(crate_in_hand);
            // let destination_stack = stacks.get_mut(order.destination).unwrap();
            // destination_stack.push(crate_in_hand);
        }
        let destination_stack = stacks.get_mut(order.destination).unwrap();
        for crejt in buffer.into_iter().rev() {
            destination_stack.push(crejt);
        }
        for (i,col) in stacks.iter().enumerate() {
            println!("col:{} {:?}", i, col);
        }
    }
    let mut result = vec![];
    for (i,col) in stacks.iter().enumerate() {
        if i == 0 {
            continue
        }
        println!("col:{} {:?}", i, col);
        result.push(match col.last() {
            None => ' ',
            Some(c) => *c,
        });
    }
    println!("{:?}", result);
    result.into_iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day5.txt");
        assert_ne!(part1(input), " VFRVCZM ".to_string());
        assert_ne!(part1(input), "VFRVCZM".to_string());
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2022/day5.txt");
    //     assert_eq!(part2(parse_input(input).as_slice()), 19081);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
        );

        assert_eq!(result, "CMZ")
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
        );

        assert_eq!(result, "MCD")
    }
}
