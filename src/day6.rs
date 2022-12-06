use std::collections::HashSet;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    for i in 0..(chars.len() - 4) {
        let mut set = HashSet::new();
        set.insert(*chars.get(i).unwrap());
        set.insert(*chars.get(i + 1).unwrap());
        set.insert(*chars.get(i + 2).unwrap());
        set.insert(*chars.get(i + 3).unwrap());
        if set.len() == 4 {
            return i + 4;
        }
    }
    panic!()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    for i in 0..(chars.len() - 4) {
        let mut set = HashSet::new();
        for j in i..(i + 14) {
            set.insert(*chars.get(j).unwrap());
        }
        if set.len() == 14 {
            return i + 14;
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day6.txt");
        assert_eq!(part1(input), 1909);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day6.txt");
        assert_eq!(part2(input), 3380);
    }

    #[test]
    fn part1_provided_example() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2_provided_example() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
