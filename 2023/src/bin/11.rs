advent_of_code::solution!(11);

struct Point {
    r: usize,
    c: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    // _print_grid(&grid);
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
    // _print_grid(&grid);

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

    let mut sum_of_shortest_distances_between_pairs = 0;
    for a in 0..points.len() - 1 {
        for b in a + 1..points.len() {
            sum_of_shortest_distances_between_pairs +=
                points[a].r.abs_diff(points[b].r) + points[a].c.abs_diff(points[b].c);
        }
    }

    Some(sum_of_shortest_distances_between_pairs)
}

fn _print_grid(grid: &[Vec<bool>]) {
    grid.iter().for_each(|row| {
        row.iter()
            .for_each(|&e| print!("{}", if e { '#' } else { '.' }));
        println!();
    });
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    // _print_grid(&grid);
    let rows_without_galaxies: Vec<usize> = (0..grid.len())
        .filter(|&r| grid[r].iter().all(|&e| !e))
        .collect();
    // println!("rows_without_galaxies: {rows_without_galaxies:?}");
    let cols_without_galaxies: Vec<usize> = (0..grid[0].len())
        .filter(|&c| grid.iter().all(|row| !row[c]))
        .collect();
    // println!("cols_without_galaxies: {cols_without_galaxies:?}");

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

    // let empty_space_multiplier = 10;
    // let empty_space_multiplier = 100;
    let empty_space_multiplier = 1000000;
    let mut sum_of_shortest_distances_between_pairs = 0;
    for a in 0..points.len() - 1 {
        for b in a + 1..points.len() {
            let row_diff: usize = (points[a].r..points[b].r)
                .map(|r| {
                    if rows_without_galaxies.contains(&r) {
                        empty_space_multiplier
                    } else {
                        1
                    }
                })
                .sum();
            // println!("row_diff between {} and {} is {}", a, b, row_diff);
            let lesser_col = points[a].c.min(points[b].c);
            let greater_col = points[a].c.max(points[b].c);
            let col_diff: usize = (lesser_col..greater_col)
                .map(|c| {
                    if cols_without_galaxies.contains(&c) {
                        empty_space_multiplier
                    } else {
                        1
                    }
                })
                .sum();
            // println!("col_diff between {} and {} is {}", a, b, col_diff);
            sum_of_shortest_distances_between_pairs += row_diff + col_diff;
        }
    }

    Some(sum_of_shortest_distances_between_pairs)
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

        //// if the multiplier is 10
        // assert_eq!(result, Some(1030));

        //// if the multiplier is 100
        assert_eq!(result, Some(8410));
    }
}
