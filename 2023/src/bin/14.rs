use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().fold(vec![], |mut acc, line| {
        acc.push(line.chars().collect());
        acc
    });

    // _print_grid(&grid);

    for row_index in 0..grid.len() {
        for col_index in 0..grid[0].len() {
            if grid[row_index][col_index] == 'O' {
                // "move" the 'O' up as far as possible
                if let Some(row_index_to_roll_into) = (0..row_index)
                    .rev()
                    .take_while(|r| grid[*r][col_index] == '.')
                    .last()
                {
                    grid[row_index_to_roll_into][col_index] = 'O';
                    grid[row_index][col_index] = '.';
                }
            }
        }
    }

    // _print_grid(&grid);

    Some(
        grid.iter()
            .rev()
            .enumerate()
            .map(|(row_index, row)| (row_index + 1) * row.iter().filter(|c| **c == 'O').count())
            .sum(),
    )
}

fn _print_grid(grid: &[Vec<char>]) {
    println!();
    grid.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{}", c));
        println!()
    });
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().fold(vec![], |mut acc, line| {
        acc.push(line.chars().collect());
        acc
    });
    // I could rely entirely on part one's algorithm to roll rocks north and just rotate the whole grid clockwise before rolling rocks "north" again, but that's going to be expensive since the actual input data is big and I'm probably going to have to do it a lot of times

    // the clear trap to fall into here is that I have to identify some sort of looping pattern that the cycles fall into and use that to extrapolate the final position instead of actually running 1000000000 of them
    // but how do I remember what states I've fallen into after the successive cycles, and how many cycles it took to get there originally?
    // I know there will be n distinct cycles in the pattern. I have to memorize all of them
    // I can use a Vec to record the arrangement for each new cycle, and I can use a hashmap to match the arrangement to a cycle number (an index in the array)
    // using a Vec<Vec<char>> as a hash key or value seems ghastly, but I won't worry about that unless necessary (it turned out to be fine)
    let mut arrangements_by_cycle: Vec<Vec<Vec<char>>> = vec![];
    let mut cycles_by_arrangement: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let cycles = 1000000000;
    let mut first_cycle_of_pattern = 0;
    for cycle in 0..cycles {
        tilt(&mut grid, Direction::North);
        tilt(&mut grid, Direction::West);
        tilt(&mut grid, Direction::South);
        tilt(&mut grid, Direction::East);
        // println!("cycle: {}", cycle);
        // _print_grid(&grid);
        // println!();
        if let Some(previous_cycle) = cycles_by_arrangement.get(&grid) {
            // we've found our pattern. There are only as many distinct cycles in the pattern as there are items in cycles_by_arrangement
            first_cycle_of_pattern = *previous_cycle;
            break;
        }
        cycles_by_arrangement.insert(grid.clone(), cycle);
        arrangements_by_cycle.push(grid.clone());
    }
    // println!("first_cycle_of_pattern: {}", first_cycle_of_pattern,);

    let cycles_in_pattern = arrangements_by_cycle.len() - first_cycle_of_pattern;
    // println!("cycles_in_pattern: {}", cycles_in_pattern,);

    let cycles_after_pattern_begins = cycles - first_cycle_of_pattern;
    // println!(
    //     "cycles_after_pattern_begins: {}",
    //     cycles_after_pattern_begins
    // );

    let repeating_arrangements = &arrangements_by_cycle[first_cycle_of_pattern..];

    let final_grid = &repeating_arrangements[(cycles_after_pattern_begins - 1) % cycles_in_pattern];

    // _print_grid(&final_grid);

    Some(
        final_grid
            .iter()
            .rev()
            .enumerate()
            .map(|(row_index, row)| (row_index + 1) * row.iter().filter(|c| **c == 'O').count())
            .sum(),
    )
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn tilt(grid: &mut [Vec<char>], direction: Direction) {
    let row_count = grid.len();
    let col_count = grid[0].len();
    // using closures like this means I can reuse these expressions in each match arm without having to clone the produced iterators
    let get_row_index_iterator = || 0..row_count;
    let get_col_index_iterator = || 0..col_count;

    // this is how I would get the indexes if I wanted to avoid Box and dyn shenanigans
    // let stone_indexes_ordered_for_traversal: Vec<(usize, usize)> = match direction {
    //     Direction::North => get_row_index_iterator()
    //         .flat_map(|row_index| {
    //             get_col_index_iterator().map(move |col_index| (row_index, col_index))
    //         })
    //         .filter(|(row_index, col_index)| grid[*row_index][*col_index] == 'O')
    //         .collect(),
    //     Direction::West => get_col_index_iterator()
    //         .flat_map(|col_index| {
    //             get_row_index_iterator().map(move |row_index| (row_index, col_index))
    //         })
    //         .filter(|(row_index, col_index)| grid[*row_index][*col_index] == 'O')
    //         .collect(),
    //     Direction::South => get_row_index_iterator()
    //         .rev()
    //         .flat_map(|row_index| {
    //             get_col_index_iterator().map(move |col_index| (row_index, col_index))
    //         })
    //         .filter(|(row_index, col_index)| grid[*row_index][*col_index] == 'O')
    //         .collect(),
    //     Direction::East => get_col_index_iterator()
    //         .rev()
    //         .flat_map(|col_index| {
    //             get_row_index_iterator().map(move |row_index| (row_index, col_index))
    //         })
    //         .filter(|(row_index, col_index)| grid[*row_index][*col_index] == 'O')
    //         .collect(),
    // };

    // I'd like to just apply the .filter() in/on this match statement, but I don't think I can since I have to wrestle with the iterator type like this with Box<dyn ...>
    // another approach is to have the match statement output a collected Vec, but then I have to copy+paste this .filter() and the .collect() for each match arm, and that's a pain to read
    // I'm not going to worry about the performance backlash of heap allocation from using Box like this; it's probably not a big deal
    let indexes_ordered_for_traversal: Box<dyn Iterator<Item = (usize, usize)>> = match direction {
        Direction::North => Box::new(get_row_index_iterator().flat_map(|row_index| {
            get_col_index_iterator().map(move |col_index| (row_index, col_index))
        })),
        Direction::West => Box::new(get_col_index_iterator().flat_map(|col_index| {
            get_row_index_iterator().map(move |row_index| (row_index, col_index))
        })),
        Direction::South => Box::new(get_row_index_iterator().rev().flat_map(|row_index| {
            get_col_index_iterator().map(move |col_index| (row_index, col_index))
        })),
        Direction::East => Box::new(get_col_index_iterator().rev().flat_map(|col_index| {
            get_row_index_iterator().map(move |row_index| (row_index, col_index))
        })),
    };

    // I'd like to not have to collect the Vec, but if I don't, then a problem arises below when the rocks are rolled into place because we're mutably borrowing grid inside the for loop despite the fact that the iterator is also immutably borrowing grid
    let stone_indexes_ordered_for_traversal: Vec<(usize, usize)> = indexes_ordered_for_traversal
        .filter(|(row_index, col_index)| grid[*row_index][*col_index] == 'O')
        .collect();

    for (row_index, col_index) in stone_indexes_ordered_for_traversal {
        let indexes_to_roll_into: Option<(usize, usize)> = match direction {
            Direction::North => (0..row_index)
                .rev()
                .take_while(|r| grid[*r][col_index] == '.')
                .last()
                .map(|r| (r, col_index)),
            Direction::West => (0..col_index)
                .rev()
                .take_while(|c| grid[row_index][*c] == '.')
                .last()
                .map(|c| (row_index, c)),
            Direction::South => (row_index + 1..)
                .take_while(|r| *r < row_count && grid[*r][col_index] == '.')
                .last()
                .map(|r| (r, col_index)),
            Direction::East => (col_index + 1..)
                .take_while(|c| *c < col_count && grid[row_index][*c] == '.')
                .last()
                .map(|c| (row_index, c)),
        };
        if let Some((row_index_to_roll_into, col_index_to_roll_into)) = indexes_to_roll_into {
            grid[row_index_to_roll_into][col_index_to_roll_into] = 'O';
            grid[row_index][col_index] = '.';
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}