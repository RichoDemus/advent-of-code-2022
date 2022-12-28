use once_cell::sync::Lazy;
use regex::Regex;

use crate::grid::Grid;

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    solve_part1(input, 2_000_000)
}

#[derive(Debug)]
struct Reading {
    sensor_x: i64,
    sensor_y: i64,
    beacon_x: i64,
    beacon_y: i64,
}

fn solve_part1(input: &str, desired_y: i64) -> usize {
    let regex = Lazy::new(|| {
        Regex::new(r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)").unwrap()
    });

    let readings = regex
        .captures_iter(input)
        .map(|caps| Reading {
            sensor_x: caps.get(1).unwrap().as_str().parse().unwrap(),
            sensor_y: caps.get(2).unwrap().as_str().parse().unwrap(),
            beacon_x: caps.get(3).unwrap().as_str().parse().unwrap(),
            beacon_y: caps.get(4).unwrap().as_str().parse().unwrap(),
        })
        .collect::<Vec<_>>();
    // println!("{readings:?}");

    let mut grid: Grid<char> = Grid {
        default: Some('.'),
        ..Default::default()
    };
    // println!("Inserting");
    for reading in readings {
        let bx = reading.beacon_x;
        let by = reading.beacon_y;
        let sx = reading.sensor_x;
        let sy = reading.sensor_y;
        grid.grid.insert((bx, by), 'B');
        grid.grid.insert((sx, sy), 'S');

        // if sx == 8 && sy == 7 {
        let distance = (sx - bx).abs() + (sy - by).abs();
        // println!("\tDistance: {distance}");
        for y in (sy - distance)..=(sy + distance) {
            if y != desired_y {
                continue;
            }
            let mut num_xes = distance - (y - sy).abs();
            num_xes *= 2;
            num_xes += 1;
            let half = num_xes / 2;
            // println!("\trow{y} of {distance} Xs: {num_xes}");

            for dx in 0..num_xes {
                let x = sx - half + dx;
                grid.grid.entry((x, y)).or_insert('#');
            }
            // }
        }
        // println!("{i}/{readings_len}")
    }

    // println!("done inserting");
    // println!("{grid}");
    grid.grid
        .iter()
        .filter(|((_x, y), c)| *y == desired_y && **c == '#')
        .count()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> i64 {
    solve_part2_v2(input, 4_000_000)
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    distance: i64,
}

fn solve_part2_v2(input: &str, xy_max: i64) -> i64 {
    let regex = Lazy::new(|| {
        Regex::new(r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)").unwrap()
    });

    let sensors = regex
        .captures_iter(input)
        .map(|caps| Reading {
            sensor_x: caps.get(1).unwrap().as_str().parse().unwrap(),
            sensor_y: caps.get(2).unwrap().as_str().parse().unwrap(),
            beacon_x: caps.get(3).unwrap().as_str().parse().unwrap(),
            beacon_y: caps.get(4).unwrap().as_str().parse().unwrap(),
        })
        .map(|reading| Sensor {
            x: reading.sensor_x,
            y: reading.sensor_y,
            distance: (reading.sensor_x - reading.beacon_x).abs()
                + (reading.sensor_y - reading.beacon_y).abs(),
        })
        .collect::<Vec<_>>();
    // println!("{sensors:?}");

    let mut y_candidate = 0;
    while y_candidate <= xy_max {
        // sensors.sort_unstable_by_key(|s|(s.x - xy_max/2).abs() + (s.y - y_candidate).abs());
        // panic!("{sensors:?}");
        let mut x_candidate = 0;
        'next: while x_candidate <= xy_max {
            // println!("Checking {x_candidate},{y_candidate}");
            for Sensor { x, y, distance } in &sensors {
                if (x_candidate - x).abs() + (y_candidate - y).abs() <= *distance {
                    // println!("{x_candidate},{y_candidate} hit {x},{y} {distance}");
                    let y_distance = (y_candidate - y).abs();
                    // println!("we're {y_distance} away");
                    let x_steps = distance - y_distance;
                    // println!("It's {x_steps} wide, it's edge should be at {},{}", x + x_steps, y_candidate);
                    x_candidate = x + x_steps + 1;
                    continue 'next;
                }
            }
            // if we get here it means we're outside all sensors?
            return x_candidate * 4_000_000 + y_candidate;
        }
        // if y_candidate % 10000 == 0 {
        //     println!("{y_candidate}/{xy_max}");
        //
        // }
        y_candidate += 1;
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day15.txt");
        assert_eq!(part1(input), 4879972);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day15.txt");
        assert_eq!(part2(input), 12525726647448);
    }

    #[test]
    fn part1_provided_example() {
        let result = solve_part1(
            r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#,
            10,
        );

        assert_eq!(result, 26)
    }

    #[test]
    fn part2_provided_example() {
        let result = solve_part2_v2(
            r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#,
            20,
        );

        assert_eq!(result, 56000011)
    }
}
