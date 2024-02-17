use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // to simply avoid double-counting the numbers, I'm going to look for numbers first and surrounding "parts" (symbols) second
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // now that I have a grid, I can scour any set of coords for parts
    // to find the numbers and their coords, I'm going ot iterate over the lines, identify the numbers therein, map the lines to the sum of their part numbers, and then sum those lines' part number sums
    Some(
        input
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                let digits_with_indices: Vec<(usize, u32)> = line
                    .char_indices()
                    .filter_map(|(i, c)| c.to_digit(10).and_then(|d| Some((i, d))))
                    .collect();
                // now that I have the digit chars and their indexes, I need to convert them to numbers with indices (I'm only going to bother storing the index at which each number starts, because I can just take the logarithm base 10 of the number to derive its last index)
                // once I have the numbers with indices, I can scour their surroundings and identify if they're part numbers
                let numbers_with_indices: Vec<(usize, u32)> =
                    digits_with_indices
                        .into_iter()
                        .fold(vec![], |mut acc, digit_with_index| {
                            // if there is a number in the growing vec, and if that number's last index contiguously precedes the digit, let's append the digit to that number
                            // otherwise, this digit must be the start of a brand new number
                            match acc.last_mut() {
                                // I'm assuming no numbers start with 0, otherwise I can't use ilog10
                                Some((number_index, number))
                                    if *number_index + (*number).ilog10() as usize + 1
                                        == digit_with_index.0 =>
                                {
                                    *number = *number * 10 + digit_with_index.1
                                }
                                _ => acc.push(digit_with_index),
                            }
                            acc
                        });

                // println!("{:?}", numbers_with_indices);

                // I'm going to assume that all the parts will just be non-ascii-alphanumeric, non-period chars
                let is_symbol = |c: &char| *c != '.' && !c.is_ascii_alphanumeric();

                // let's filter down to just the part numbers and sum them
                numbers_with_indices
                    .into_iter()
                    .filter(|(col_index, number)| {
                        // now we scour the grid using the row_index and col_index
                        let first_adjacent_col_index = if *col_index > 0 {
                            if is_symbol(&grid[row_index][col_index - 1]) {
                                return true;
                            }
                            col_index - 1
                        } else {
                            *col_index
                        };
                        let last_index_of_number = *col_index + number.ilog10() as usize;
                        let last_adjacent_col_index = if last_index_of_number + 1 < line.len() {
                            if is_symbol(&grid[row_index][last_index_of_number + 1]) {
                                return true;
                            }
                            last_index_of_number + 1
                        } else {
                            last_index_of_number
                        };

                        let grid_row_slice_contains_symbol =
                            |adjacent_row_index: usize,
                             first_adjacent_col_index: usize,
                             last_adjacent_col_index: usize| {
                                grid[adjacent_row_index]
                                    .get(first_adjacent_col_index..=last_adjacent_col_index)
                                    .unwrap()
                                    .iter()
                                    .any(is_symbol)
                            };

                        if row_index > 0 {
                            // check preceding row
                            if grid_row_slice_contains_symbol(
                                row_index - 1,
                                first_adjacent_col_index,
                                last_adjacent_col_index,
                            ) {
                                return true;
                            }
                        }
                        if row_index + 1 < grid.len() {
                            // check following row
                            // if grid[rowTK_symbol)
                            if grid_row_slice_contains_symbol(
                                row_index + 1,
                                first_adjacent_col_index,
                                last_adjacent_col_index,
                            ) {
                                return true;
                            }
                        }

                        false
                    })
                    .map(|(_, number)| number)
                    .sum::<u32>()
                // 0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // let's iterate over the lines in a similar fashion, but this time instead of recording the whole grid first, let's just write down all the numbers with their row and col indices AND all the asterisks with their own row and col indices
    let mut numbers_with_indices: Vec<(usize, usize, u32)> =
        input
            .lines()
            .enumerate()
            .fold(vec![], |acc, (row_index, line)| {
                let digits_with_indices: Vec<(usize, u32)> = line
                    .char_indices()
                    .filter_map(|(i, c)| c.to_digit(10).and_then(|d| Some((i, d))))
                    .collect();
                // now that I have the digit chars and their indexes, I need to convert them to numbers with indices (I'm only going to bother storing the index at which each number starts, because I can just take the logarithm base 10 of the number to derive its last index)
                // once I have the numbers with indices, I can scour their surroundings and identify if they're part numbers
                let numbers_with_col_indices: Vec<(usize, u32)> = digits_with_indices
                    .into_iter()
                    .fold(vec![], |mut acc, digit_with_index| {
                        // if there is a number in the growing vec, and if that number's last index contiguously precedes the digit, let's append the digit to that number
                        // otherwise, this digit must be the start of a brand new number
                        match acc.last_mut() {
                            // I'm assuming no numbers start with 0, otherwise I can't use ilog10
                            Some((number_index, number))
                                if *number_index + (*number).ilog10() as usize + 1
                                    == digit_with_index.0 =>
                            {
                                *number = *number * 10 + digit_with_index.1
                            }
                            _ => acc.push(digit_with_index),
                        }
                        acc
                    });

                let numbers_with_indices: Vec<(usize, usize, u32)> = numbers_with_col_indices
                    .into_iter()
                    .map(|(col_index, number)| (row_index, col_index, number))
                    .collect();

                [acc, numbers_with_indices].concat()
            });

    // let mut numbers_with_indices: [(usize, usize, u32)] = numbers_with_indices.into();
    // println!("{:?}", numbers_with_indices);

    // now that I know all the numbers and where they are, I need to consider the gears
    // the naive approach is to just to iterate through the whole list of numbers every time we find an asterisk
    // but there must be a data structure that facilitates a faster way to search for numbers that are relevant to each asterisk
    // the numbers are already sorted by row index and col index respectively
    // how about a hash map whose keys are row indices and whose values are arrays of numbers with their col indices?
    let numbers_by_row_index: HashMap<usize, Vec<(usize, u32)>> =
        (0..input.lines().count()).fold(HashMap::new(), |mut acc, row_index| {
            //
            let count_of_numbers_in_given_row = numbers_with_indices
                .iter_mut()
                .take_while(|(r, _, _)| *r == row_index)
                .count();

            let (left, right) = numbers_with_indices.split_at(count_of_numbers_in_given_row);

            acc.insert(
                row_index,
                left.into_iter()
                    .map(|(_, col_index, number)| (*col_index, *number))
                    .collect(),
            );

            numbers_with_indices = right.into();

            acc
        });

    let asterisk_indices: Vec<(usize, usize)> =
        input
            .lines()
            .enumerate()
            .fold(vec![], |acc, (row_index, line)| {
                let line_asterisk_indices: Vec<(usize, usize)> = line
                    .char_indices()
                    .filter(|(_, c)| *c == '*')
                    .map(|(col_index, _)| (row_index, col_index))
                    .collect();
                [acc, line_asterisk_indices].concat()
            });
    // println!("asterisk_indices: {:?}", asterisk_indices);

    // I can easily binary search for the numbers immediately left and right of the asterisk
    // looking for the numbers above and below the asterisk are harder. There must be a cleverer way to do it, but I'm just going to iterate over the given line's numbers count the ones adjacent to each asterisk
    Some(
        asterisk_indices
            .iter()
            .map(|(row_index, col_index)| {
                let part_number_to_left =
                    part_number_to_left(*row_index, *col_index, &numbers_by_row_index);
                // println!("{:?}", part_number_to_left);
                let part_number_to_right =
                    part_number_to_right(*row_index, *col_index, &numbers_by_row_index);
                // println!("{:?}", part_number_to_right);
                let part_numbers_above =
                    part_numbers_above(*row_index, *col_index, &numbers_by_row_index);
                // println!("{:?}", part_numbers_above);
                let part_numbers_below =
                    part_numbers_below(*row_index, *col_index, &numbers_by_row_index);
                // println!("{:?}", part_numbers_below);

                let adjacent_part_numbers: Vec<u32> =
                    vec![part_number_to_left, part_number_to_right]
                        .into_iter()
                        .flatten()
                        .chain(part_numbers_above.into_iter())
                        .chain(part_numbers_below.into_iter())
                        .collect();

                // now we just have to check if it's a gear and return the gear ratio if so
                if adjacent_part_numbers.len() == 2 {
                    return adjacent_part_numbers.into_iter().product();
                }

                0
            })
            .sum(),
    )
}

fn part_number_to_left(
    row_index: usize,
    col_index: usize,
    numbers_by_row_index: &HashMap<usize, Vec<(usize, u32)>>,
) -> Option<u32> {
    if let Some(numbers_with_indices) = numbers_by_row_index.get(&row_index) {
        if let Ok(index_in_numbers_vec) = numbers_with_indices
            .binary_search_by_key(&(col_index - 1), |(index, number)| {
                index + number.ilog10() as usize
            })
        {
            return Some(numbers_with_indices[index_in_numbers_vec].1);
        }
    }
    None
}

fn part_number_to_right(
    row_index: usize,
    col_index: usize,
    numbers_by_row_index: &HashMap<usize, Vec<(usize, u32)>>,
) -> Option<u32> {
    if let Some(numbers_with_indices) = numbers_by_row_index.get(&row_index) {
        if let Ok(index_in_numbers_vec) =
            numbers_with_indices.binary_search_by_key(&(col_index + 1), |(index, _)| *index)
        {
            return Some(numbers_with_indices[index_in_numbers_vec].1);
        }
    }
    None
}

fn part_numbers_above(
    row_index: usize,
    col_index: usize,
    numbers_by_row_index: &HashMap<usize, Vec<(usize, u32)>>,
) -> Vec<u32> {
    if row_index > 0 {
        if let Some(numbers_with_indices) = numbers_by_row_index.get(&(row_index - 1)) {
            return numbers_with_indices
                .iter()
                .filter(|(index, number)| {
                    col_index + 1 >= *index && col_index <= index + number.ilog10() as usize + 1
                })
                .map(|(_, number)| *number)
                .collect();
        }
    }
    vec![]
}

fn part_numbers_below(
    row_index: usize,
    col_index: usize,
    numbers_by_row_index: &HashMap<usize, Vec<(usize, u32)>>,
) -> Vec<u32> {
    if let Some(numbers_with_indices) = numbers_by_row_index.get(&(row_index + 1)) {
        return numbers_with_indices
            .iter()
            .filter(|(index, number)| {
                col_index + 1 >= *index && col_index <= index + number.ilog10() as usize + 1
            })
            .map(|(_, number)| *number)
            .collect();
    }
    vec![]
}
// fn has_part_number_above(
//     row_index: usize,
//     col_index: usize,
//     numbers_by_row_index: &HashMap<usize, Vec<(usize, u32)>>,
// ) -> bool {
//     if row_index == 0 {
//         return false;
//     }
//     match numbers_by_row_index.get(&(row_index - 1)) {
//         Some(numbers_with_indices) => {
//             match numbers_with_indices.binary_search_by_key(&(col_index), |(index, _)| *index) {
//                 Ok(_) => true,
//                 Err(_) => false,
//             }
//         }
//         None => false,
//     }
// }
// fn has_part_number_below(
//     row_index: usize,
//     col_index: usize,
//     numbers_by_row_index: &HashMap<usize, Vec<(usize, u32)>>,
// ) -> bool {
//     match numbers_by_row_index.get(&(row_index + 1)) {
//         Some(numbers_with_indices) => {
//             match numbers_with_indices.binary_search_by_key(&(col_index), |(index, _)| *index) {
//                 Ok(_) => true,
//                 Err(_) => false,
//             }
//         }
//         None => false,
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
