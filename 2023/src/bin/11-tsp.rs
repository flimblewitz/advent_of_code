// originally, I misread 11 part 1 and thought that it was asking me to solve the traveling salesman problem (finding the shortest path that visits all galaxies)
// it felt like a valuable learning experience to implement, so here it is

use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(11);

struct Point {
    r: usize,
    c: usize,
}

struct Path {
    ending_point_index: usize,
    bitmask: usize, // this represents the set of points that have been traveled to get to the ending_point
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    _print_grid(&grid);
    let rows_without_galaxies: Vec<usize> = (0..grid.len())
        .filter(|&r| grid[r].iter().all(|&e| !e))
        .collect();
    // println!("rows_without_galaxies: {rows_without_galaxies:?}");
    let cols_without_galaxies: Vec<usize> = (0..grid[0].len())
        .filter(|&c| grid.iter().all(|row| !row[c]))
        .collect();
    // println!("cols_without_galaxies: {cols_without_galaxies:?}");
    let col_count = grid[0].len();
    rows_without_galaxies
        .iter()
        .rev()
        .for_each(|&r| grid.insert(r, vec![false; col_count]));
    cols_without_galaxies
        .iter()
        .rev()
        .for_each(|&c| grid.iter_mut().for_each(|row| row.insert(c, false)));
    _print_grid(&grid);

    let points: Vec<Point> = grid
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(c, e)| if *e { Some(Point { r, c }) } else { None })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    // the first index is the ending point's index
    // the second index is the bitmask representing the set of points that were traversed to get there
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    // (0..points.len()).for_each(|point_index| cost.insert((point_index,  ))
    // let mut cost: Vec<Vec<Option<usize>>> = (0..points.len())
    //     .map(|_| {
    //         (0..2_usize.pow(points.len() as u32))
    //             .map(|_| None)
    //             .collect()
    //     })
    //     .collect();

    let mut paths_to_consider: VecDeque<Path> = VecDeque::new();

    // let's initialize the stating_paths_to_consider as all of the individual points
    for point_index in 0..points.len() {
        // since this is just a path to the nth point via a path consisting of only the nth point, the bitmask is just 2^point_index, AKA 1 << point_index
        let bitmask = 2_usize.pow(point_index as u32);
        paths_to_consider.push_back(Path {
            ending_point_index: point_index,
            bitmask,
        });
        costs.insert((point_index, bitmask), 0);
    }

    while let Some(Path {
        ending_point_index,
        bitmask,
    }) = paths_to_consider.pop_front()
    {
        // println!("ending_point_index: {}", ending_point_index);
        // let's consider each possible path starting from the current path
        let ending_point = &points[ending_point_index];
        for next_point_index in 0..points.len() {
            // println!("next_point_index: {}", next_point_index);
            let next_point = &points[next_point_index];
            let distance_to_next_point =
                ending_point.r.abs_diff(next_point.r) + ending_point.c.abs_diff(next_point.c);
            let bitmask_to_next_point = bitmask | 2_usize.pow(next_point_index as u32);
            // it's safe to unwrap here because we've implicitly already considered that path
            let cost_to_ending_point = costs.get(&(ending_point_index, bitmask)).unwrap();
            let cost_to_next_point = cost_to_ending_point + distance_to_next_point;
            let existing_cost = costs
                .entry((next_point_index, bitmask_to_next_point))
                .or_insert(usize::MAX);
            // println!("existing_cost: {}", existing_cost);
            if *existing_cost > cost_to_next_point {
                println!(
                    "best way to get to {} from {} as the last of {} is now {}",
                    next_point_index, ending_point_index, bitmask_to_next_point, cost_to_next_point
                );
                *existing_cost = cost_to_next_point;
                paths_to_consider.push_back(Path {
                    ending_point_index: next_point_index,
                    bitmask: bitmask_to_next_point,
                });
            }
        }
    }

    let bitmask_for_full_path = 2_usize.pow(points.len() as u32) - 1;

    Some(
        (0..points.len())
            .map(|point_index| *costs.get(&(point_index, bitmask_for_full_path)).unwrap())
            .min()
            .unwrap() as u32,
    )
}

fn _print_grid(grid: &[Vec<bool>]) {
    grid.iter().for_each(|row| {
        row.iter()
            .for_each(|&e| print!("{}", if e { '#' } else { '.' }));
        println!();
    });
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
