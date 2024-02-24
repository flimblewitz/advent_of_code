advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    // so I really just need to count the number of steps in the loop and then divide by two
    // there are random pipes scattered all over the input, so I can't just count them all
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut observed = vec![vec![false; grid[0].len()]; grid.len()];

    let (s_row_index, s_col_index) = grid
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .find_map(|(col_index, c)| (*c == 'S').then(|| (row_index, col_index)))
        })
        .unwrap();
    // println!("{}, {}", start_row, start_col);

    observed[s_row_index][s_col_index] = true;

    let (mut current_row_index, mut current_col_index) =
        get_next_pipe_indices(&grid, &mut observed, s_row_index, s_col_index).unwrap();

    while let Some((next_row_index, next_col_index)) =
        get_next_pipe_indices(&grid, &mut observed, current_row_index, current_col_index)
    {
        current_row_index = next_row_index;
        current_col_index = next_col_index;
    }
    // observed.iter().enumerate().for_each(|(row_index, row)| {
    //     row.iter().enumerate().for_each(|(col_index, col)| {
    //         print!(
    //             "{}",
    //             if *col {
    //                 grid[row_index][col_index]
    //             } else {
    //                 ' '
    //             }
    //         )
    //     });
    //     println!();
    // });

    Some(
        observed
            .iter()
            .map(|row| row.iter().filter(|o| **o).count() as u32)
            .sum::<u32>()
            / 2,
    )
}

fn get_next_pipe_indices(
    grid: &[Vec<char>],
    observed: &mut [Vec<bool>],
    current_row_index: usize,
    current_col_index: usize,
) -> Option<(usize, usize)> {
    observed[current_row_index][current_col_index] = true;

    let current_pipe = grid[current_row_index][current_col_index];

    let above = current_row_index
        .checked_sub(1)
        .and_then(|row_index| grid[row_index].get(current_col_index))
        .and_then(|c| match (current_pipe, *c) {
            ('S' | '|' | 'L' | 'J', '|' | '7' | 'F') => {
                Some(observed[current_row_index - 1][current_col_index])
            }
            _ => None,
        });
    let below = grid
        .get(current_row_index + 1)
        .and_then(|row| row.get(current_col_index))
        .and_then(|c| match (current_pipe, *c) {
            ('S' | '|' | '7' | 'F', '|' | 'L' | 'J') => {
                Some(observed[current_row_index + 1][current_col_index])
            }
            _ => None,
        });
    let left = current_col_index
        .checked_sub(1)
        .and_then(|col_index| grid[current_row_index].get(col_index))
        .and_then(|c| match (current_pipe, *c) {
            ('S' | '-' | '7' | 'J', '-' | 'L' | 'F') => {
                Some(observed[current_row_index][current_col_index - 1])
            }
            _ => None,
        });
    let right = grid[current_row_index]
        .get(current_col_index + 1)
        .and_then(|c| match (current_pipe, *c) {
            ('S' | '-' | 'L' | 'F', '-' | '7' | 'J') => {
                Some(observed[current_row_index][current_col_index + 1])
            }
            _ => None,
        });

    match (above, below, left, right) {
        (Some(false), ..) => Some((current_row_index - 1, current_col_index)),
        (_, Some(false), ..) => Some((current_row_index + 1, current_col_index)),
        (_, _, Some(false), ..) => Some((current_row_index, current_col_index - 1)),
        (_, _, _, Some(false)) => Some((current_row_index, current_col_index + 1)),
        _ => {
            // println!(
            //     "stopping at {}, {} because there are no more neighbors",
            //     current_row_index, current_col_index
            // );
            None
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // so I really just need to count the number of steps in the loop and then divide by two
    // there are random pipes scattered all over the input, so I can't just count them all
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut observed = vec![vec![false; grid[0].len()]; grid.len()];

    let (s_row_index, s_col_index) = grid
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .find_map(|(col_index, c)| (*c == 'S').then(|| (row_index, col_index)))
        })
        .unwrap();
    // println!("{}, {}", start_row, start_col);

    observed[s_row_index][s_col_index] = true;

    let (mut current_row_index, mut current_col_index) =
        get_next_pipe_indices(&grid, &mut observed, s_row_index, s_col_index).unwrap();

    while let Some((next_row_index, next_col_index)) =
        get_next_pipe_indices(&grid, &mut observed, current_row_index, current_col_index)
    {
        current_row_index = next_row_index;
        current_col_index = next_col_index;
    }

    _print_observed_grid(&observed, &grid);

    // let's iterate over the observed grid again. The plan is to hit every tile on the edge and then do either a depth or breadth first search through all of the non-observed tiles, marking them as observed as we go. The answer will be the list of unobserved tiles afterward

    // let mut observed = vec![vec![false; grid[0].len()]; grid.len()];

    let row_count = grid.len();
    let col_count = grid[0].len();
    for col_index in 0..col_count {
        traverse_region(&mut observed, 0, col_index);
        traverse_region(&mut observed, row_count - 1, col_index);
    }
    // try to traverse the left and right edges
    for row_index in 0..row_count {
        traverse_region(&mut observed, row_index, 0);
        traverse_region(&mut observed, row_index, col_count - 1);
    }

    _print_observed_grid(&observed, &grid);

    // iterate over observed one more time
    // count all the tiles for which there is an even number of perpendicular loop_grid tiles in each vertical and horizontal direction. So just count the '|' and '-' tiles

    // this isn't good enough because it doesn't count the "outside" tiles that still arguably surrounded by the loop
    Some(
        observed
            .iter()
            .map(|row| row.iter().filter(|col| !**col).count() as u32)
            .sum::<u32>(),
    )
}

fn traverse_region(observed: &mut [Vec<bool>], row_index: usize, col_index: usize) {
    if observed[row_index][col_index] {
        return;
    }

    observed[row_index][col_index] = true;

    let first_row_index = if row_index > 0 {
        row_index - 1
    } else {
        row_index
    };
    let last_row_index = if row_index + 1 < observed.len() {
        row_index + 1
    } else {
        row_index
    };
    let first_col_index = if col_index > 0 {
        col_index - 1
    } else {
        col_index
    };
    let last_col_index = if col_index + 1 < observed[0].len() {
        col_index + 1
    } else {
        col_index
    };
    for r in first_row_index..=last_row_index {
        for c in first_col_index..=last_col_index {
            traverse_region(observed, r, c);
        }
    }
}

fn _print_observed_grid(observed: &[Vec<bool>], grid: &[Vec<char>]) {
    println!();
    observed.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(col_index, col)| {
            print!(
                "{}",
                if *col {
                    grid[row_index][col_index]
                } else {
                    ' '
                }
            )
        });
        println!();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
        // let result = part_two(&advent_of_code::template::read_file_part(
        //     "examples", DAY, 2,
        // ));
        // assert_eq!(result, Some(8));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(10));
    }
}
