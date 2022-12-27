use std::fmt::Display;
use std::fmt::Formatter;

use itertools::Itertools;

use strum::EnumString;

use crate::grid::Grid;

#[derive(EnumString, Debug, Eq, PartialEq)]
enum Block {
    Air,
    Wall,
    Sand,
    Source,
}

impl Default for Block {
    fn default() -> Self {
        Self::Air
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Air => '.',
                Self::Wall => '#',
                Self::Sand => 'o',
                Self::Source => '+',
            }
        )
    }
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    let mut grid = {
        let mut grid: Grid<Block> = Grid {
            default: Some(Block::Air),
            ..Default::default()
        };
        grid.grid.insert((500, 0), Block::Source);
        for line in input.lines() {
            let coordinates = line.split(" -> ").collect::<Vec<_>>();
            for path in coordinates.windows(2) {
                let start = path.first().unwrap();
                let (x1, y1) = start
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .next_tuple()
                    .unwrap();
                let end = path.last().unwrap();
                let (x2, y2) = end
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .next_tuple()
                    .unwrap();
                assert!(x1 == x2 || y1 == y2, "can't handle diagonal lines");
                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        grid.grid.insert((x1, y), Block::Wall);
                    }
                } else {
                    for x in x1.min(x2)..=x1.max(x2) {
                        grid.grid.insert((x, y1), Block::Wall);
                    }
                }
            }
        }
        grid
    };

    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        loop {
            // remove previous sand
            if let Some(block) = grid.grid.get_mut(&(sand_x, sand_y)) {
                if &Block::Sand == block {
                    *block = Block::Air;
                }
            }
            // write new sand
            if grid.get(&(sand_x, sand_y + 1)) == Some(&Block::Air) {
                sand_y += 1;
                grid.grid.insert((sand_x, sand_y), Block::Sand);
            } else if grid.get(&(sand_x - 1, sand_y + 1)) == Some(&Block::Air) {
                sand_y += 1;
                sand_x -= 1;
                grid.grid.insert((sand_x, sand_y), Block::Sand);
            } else if grid.get(&(sand_x + 1, sand_y + 1)) == Some(&Block::Air) {
                sand_y += 1;
                sand_x += 1;
                grid.grid.insert((sand_x, sand_y), Block::Sand);
            } else {
                // this grain can't move anymore
                grid.grid.insert((sand_x, sand_y), Block::Sand);
                break;
            }
            if sand_y > 1000 {
                return grid.grid.values().filter(|b| &&Block::Sand == b).count() - 1;
            }
        }
    }
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    let mut grid = {
        let mut grid: Grid<Block> = Grid {
            default: Some(Block::Air),
            ..Default::default()
        };
        grid.grid.insert((500, 0), Block::Source);
        for line in input.lines() {
            let coordinates = line.split(" -> ").collect::<Vec<_>>();
            for path in coordinates.windows(2) {
                let start = path.first().unwrap();
                let (x1, y1) = start
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .next_tuple()
                    .unwrap();
                let end = path.last().unwrap();
                let (x2, y2) = end
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .next_tuple()
                    .unwrap();
                assert!(x1 == x2 || y1 == y2, "can't handle diagonal lines");
                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        grid.grid.insert((x1, y), Block::Wall);
                    }
                } else {
                    for x in x1.min(x2)..=x1.max(x2) {
                        grid.grid.insert((x, y1), Block::Wall);
                    }
                }
            }
        }
        grid
    };

    let floor_y = grid.grid.keys().map(|(_x, y)| y).max().unwrap() + 2;

    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        if grid.grid.get(&(500, 0)) == Some(&Block::Sand) {
            return grid.grid.values().filter(|b| &&Block::Sand == b).count();
        }
        loop {
            // remove previous sand
            if let Some(block) = grid.grid.get_mut(&(sand_x, sand_y)) {
                if &Block::Sand == block {
                    *block = Block::Air;
                }
            }
            // write new sand
            if sand_y + 1 == floor_y {
                // this grain can't move anymore
                grid.grid.insert((sand_x, sand_y), Block::Sand);
                break;
            } else if grid.get(&(sand_x, sand_y + 1)) == Some(&Block::Air) {
                sand_y += 1;
                grid.grid.insert((sand_x, sand_y), Block::Sand);
            } else if grid.get(&(sand_x - 1, sand_y + 1)) == Some(&Block::Air) {
                sand_y += 1;
                sand_x -= 1;
                grid.grid.insert((sand_x, sand_y), Block::Sand);
            } else if grid.get(&(sand_x + 1, sand_y + 1)) == Some(&Block::Air) {
                sand_y += 1;
                sand_x += 1;
                grid.grid.insert((sand_x, sand_y), Block::Sand);
            } else {
                // this grain can't move anymore
                grid.grid.insert((sand_x, sand_y), Block::Sand);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day14.txt");
        assert_eq!(part1(input), 888);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day14.txt");
        assert_eq!(part2(input), 26461);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
        );

        assert_eq!(result, 24)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
        );

        assert_eq!(result, 93)
    }
}
