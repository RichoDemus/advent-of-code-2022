use pathfinding::prelude::bfs;

use crate::grid::Grid;

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    let grid: Grid<char> = Grid::from_non_delim_block(input);
    let start = *grid
        .grid
        .iter()
        .find(|((_x, _y), value)| value == &&'S')
        .unwrap()
        .0;
    let end = *grid
        .grid
        .iter()
        .find(|((_x, _y), value)| value == &&'E')
        .unwrap()
        .0;
    // println!("{}", grid);
    // println!("start: {:?}, end: {:?}", start, end);

    let result = bfs(
        &start,
        |(x, y)| {
            let current_height = *grid.grid.get(&(*x, *y)).unwrap();
            let current_height = if current_height == 'S' {
                'a'
            } else {
                current_height
            };
            grid.calc_in_bounds_four_way_neighbours(*x, *y)
                .into_iter()
                .filter(|(n_x, n_y)| {
                    let neighbour_height = *grid.grid.get(&(*n_x, *n_y)).unwrap();
                    let neighbour_height = if neighbour_height == 'E' {
                        'z'
                    } else {
                        neighbour_height
                    };
                    // println!("we're at {},{}:{}({}) looking at {},{}:{}({})", x, y, current_height, current_height as u32, n_x, n_y, neighbour_height, neighbour_height as u32);
                    let current_height = current_height as u32;
                    let neighbour_height = neighbour_height as u32;
                    current_height + 1 >= neighbour_height
                })
                .collect::<Vec<_>>()
        },
        |(x, y)| x == &end.0 && y == &end.1,
    )
    .unwrap();

    // println!("path: {:?}", result);
    result.len() - 1
}

#[allow(clippy::flat_map_option)]
#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    let grid: Grid<char> = Grid::from_non_delim_block(input);
    let start = grid
        .grid
        .iter()
        .filter(|((_x, _y), value)| value == &&'S' || value == &&'a')
        .map(|(pos, _)| pos)
        .copied()
        .collect::<Vec<_>>();
    let end = *grid
        .grid
        .iter()
        .find(|((_x, _y), value)| value == &&'E')
        .unwrap()
        .0;
    // println!("{}", grid);
    // println!("start: {:?}, end: {:?}", start, end);
    let rounds = start.len();
    println!("{rounds} starts");

    let shortest = start
        .into_iter()
        .enumerate()
        .flat_map(|(i, (start_x, start_y))| {
            let result = bfs(
                &(start_x, start_y),
                |(x, y)| {
                    let current_height = *grid.grid.get(&(*x, *y)).unwrap();
                    let current_height = if current_height == 'S' {
                        'a'
                    } else {
                        current_height
                    };
                    grid.calc_in_bounds_four_way_neighbours(*x, *y)
                        .into_iter()
                        .filter(|(n_x, n_y)| {
                            let neighbour_height = *grid.grid.get(&(*n_x, *n_y)).unwrap();
                            let neighbour_height = if neighbour_height == 'E' {
                                'z'
                            } else {
                                neighbour_height
                            };
                            // println!("we're at {},{}:{}({}) looking at {},{}:{}({})", x, y, current_height, current_height as u32, n_x, n_y, neighbour_height, neighbour_height as u32);
                            let current_height = current_height as u32;
                            let neighbour_height = neighbour_height as u32;
                            current_height + 1 >= neighbour_height
                        })
                        .collect::<Vec<_>>()
                },
                |(x, y)| x == &end.0 && y == &end.1,
            );

            // println!("path: {:?}", result);
            // println!("len: {:?}", result);
            println!("{i}/{rounds}");
            result
        })
        .map(|path| path.len())
        .min()
        .unwrap();
    shortest - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day12.txt");
        assert_eq!(part1(input), 534);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day12.txt");
        assert_eq!(part2(input), 525);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#,
        );

        assert_eq!(result, 31)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#,
        );

        assert_eq!(result, 29)
    }
}
