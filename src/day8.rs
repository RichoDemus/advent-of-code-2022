use std::collections::HashMap;

use crate::grid::Grid;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let grid: Grid<u8> = Grid::from_non_delim_block(input);
    let mut result_grid = HashMap::new();
    // println!("{}", grid);
    let mut visibles = 0;
    for x in 0..=grid.x_max() {
        for y in 0..=grid.y_max() {
            let height = *grid.grid.get(&(x, y)).unwrap();
            // println!("considering {}@({},{})", height,x,y);
            let mut visible = 4;
            for n in 0..x {
                if *grid.grid.get(&(n, y)).unwrap() >= height {
                    // println!("\t{}@({},{}) is blocking from west", *grid.grid.get(&(n,y)).unwrap(), n,y);
                    visible -= 1;
                    break;
                }
            }
            for n in 0..y {
                if *grid.grid.get(&(x, n)).unwrap() >= height {
                    // println!("\t{}@({},{}) is blocking from north", *grid.grid.get(&(x,n)).unwrap(), x,n);
                    visible -= 1;
                    break;
                }
            }
            for n in (x + 1)..=grid.x_max() {
                if *grid.grid.get(&(n, y)).unwrap() >= height {
                    // println!("\t{}@({},{}) is blocking from east", *grid.grid.get(&(n,y)).unwrap(), n,y);
                    visible -= 1;
                    break;
                }
            }
            for n in (y + 1)..=grid.y_max() {
                if *grid.grid.get(&(x, n)).unwrap() >= height {
                    // println!("\t{}@({},{}) is blocking from south", other_height, n, y);
                    visible -= 1;
                    break;
                }
            }
            if visible > 0 {
                // println!("{}@({},{}) is {}", height,x,y,"visible".bold());
                visibles += 1;
                result_grid.insert((x, y), (height, true));
            } else {
                result_grid.insert((x, y), (height, false));
            }
        }
    }

    // for row in 0..=grid.y_max() {
    //     for column in 0..=grid.x_max(){
    //         let (height, visible) = result_grid.get(&(column, row)).unwrap();
    //         if column == 39 && row == 73 {
    //             print!("{}", height.to_string().white())
    //         } else if *visible {
    //             print!("{}", height.to_string().green())
    //         } else {
    //             print!("{}", height.to_string().red())
    //         }
    //     }
    //     println!()
    // }

    visibles
}

#[allow(clippy::similar_names)]
#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let grid: Grid<u8> = Grid::from_non_delim_block(input);
    // let mut result_grid = HashMap::new();
    // println!("{}", grid);
    let mut best_score = 0;
    for x in 0..=grid.x_max() {
        for y in 0..=grid.y_max() {
            let height = *grid.grid.get(&(x, y)).unwrap();
            // println!("considering {}@({},{})", height,x,y);
            let mut west_score = 0;
            for n in (0..x).rev() {
                west_score += 1;
                if *grid.grid.get(&(n, y)).unwrap() >= height {
                    // println!("\t{}@({},{}) is blocking from west", *grid.grid.get(&(n,y)).unwrap(), n,y);
                    break;
                }
            }
            let mut north_score = 0;
            for n in (0..y).rev() {
                north_score += 1;
                if *grid.grid.get(&(x, n)).unwrap() >= height {
                    // println!("\t{}@({},{}) is blocking from north", *grid.grid.get(&(x,n)).unwrap(), x,n);
                    // visible -=1;
                    break;
                }
            }
            let mut east_score = 0;
            for n in (x + 1)..=grid.x_max() {
                east_score += 1;
                if *grid.grid.get(&(n, y)).unwrap() >= height {
                    // println!("\t{}@({},{}) is blocking from east", *grid.grid.get(&(n,y)).unwrap(), n,y);
                    // visible -=1;
                    break;
                }
            }
            let mut south_score = 0;
            for n in (y + 1)..=grid.y_max() {
                south_score += 1;
                if *grid.grid.get(&(x, n)).unwrap() >= height {
                    // println!("\t{}@({},{}) is blocking from south", *grid.grid.get(&(x, n)).unwrap(), n, y);
                    // visible -=1;
                    break;
                }
            }
            let score = north_score * west_score * east_score * south_score;
            if score >= best_score {
                // println!("new high: {}@({},{}) with {}", height, x,y, score);
                best_score = score;
            }
        }
    }

    // for row in 0..=grid.y_max() {
    //     for column in 0..=grid.x_max(){
    //         let (height, visible) = result_grid.get(&(column, row)).unwrap();
    //         if column == 39 && row == 73 {
    //             print!("{}", height.to_string().white())
    //         } else if *visible {
    //             print!("{}", height.to_string().green())
    //         } else {
    //             print!("{}", height.to_string().red())
    //         }
    //     }
    //     println!()
    // }

    best_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day8.txt");
        assert_eq!(part1(input), 1676);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day8.txt");
        assert_eq!(part2(input), 313200);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"30373
25512
65332
33549
35390"#,
        );

        assert_eq!(result, 21)
    }

    #[test]
    fn part1_edge_case() {
        let result = part1(
            r#"999
999
999"#,
        );

        assert_eq!(result, 8)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"30373
25512
65332
33549
35390"#,
        );

        assert_eq!(result, 8)
    }
}
