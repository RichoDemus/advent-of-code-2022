#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let input = input.trim();
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|line| {
                    line.parse::<u32>()
                        .unwrap_or_else(|_| panic!("can't parse {}", line))
                })
                .sum()
        })
        .max()
        .expect("should be a value here")
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let input = input.trim();
    let mut calories = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|line| {
                    line.parse::<u32>()
                        .unwrap_or_else(|_| panic!("can't parse {}", line))
                })
                .sum()
        })
        .collect::<Vec<u32>>();

    calories.sort_unstable();
    calories.reverse();
    calories.into_iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day1.txt");
        assert_eq!(part1(input), 68292);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day1.txt");
        assert_eq!(part2(input), 203203);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#,
        );

        assert_eq!(result, 24000)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#,
        );

        assert_eq!(result, 45000)
    }
}
