use std::fmt::Debug;

type Position = (usize, usize);
type Grid = Vec<Vec<u32>>;
type ShortestPaths = Vec<Vec<Option<usize>>>;

fn parse_input(input: &str) -> (Position, Position, Grid) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    'S' => {
                        start = (row, col);
                        'a' as u32
                    }
                    'E' => {
                        end = (row, col);
                        'z' as u32
                    }
                    c => c as u32,
                })
                .collect()
        })
        .collect();
    (start, end, grid)
}

pub fn part_one(input: &str) -> usize {
    let (start, end, grid) = parse_input(input);

    get_shortest_path(start, end, &grid).unwrap()
}

fn get_shortest_path(start: Position, end: Position, grid: &Grid) -> Option<usize> {
    // I need to memoize the shortest path from the starting position to each other position. I'm just storing the number of steps, though, not the actual path
    // to do this, I need to recurse

    // println!("start: {:?}", start);
    // println!("end: {:?}", end);
    // _print_grid(&grid);

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    // I could have made the code a litte less verbose by just using usize instead of Option<usize> and setting the default to usize::MAX, but whatever
    let mut shortest_paths: ShortestPaths = vec![vec![None; num_cols]; num_rows];

    // the path to the start takes 0 steps because we're already there
    shortest_paths[start.0][start.1] = Some(0);

    recursively_solve(start, &grid, &mut shortest_paths);

    // _print_grid(&shortest_paths);

    shortest_paths[end.0][end.1]
}

fn _print_grid<T: Debug>(shortest_paths: &Vec<Vec<T>>) {
    shortest_paths.iter().for_each(|row| println!("{:?}", row));
}

fn recursively_solve(
    (current_row, current_col): Position,
    grid: &Grid,
    shortest_paths: &mut ShortestPaths,
) {
    // assume that we have a value for the shortest path to the current position
    // given current_position, look at each adjacent position
    // if the adjacent position is None, set it to shortest_paths[start.0][start.1], then recursively solve from that position
    // else if the adjacent position is Some(value), set it to Some(value.min(current + 1)) , then recursively solve from that position
    // else just return

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let current_elevation = grid[current_row][current_col];

    let num_steps_from_current_position_to_adjacent_position =
        shortest_paths[current_row][current_col].unwrap() + 1;

    let mut adjacent_positions = vec![];
    if current_row > 0 {
        adjacent_positions.push((current_row - 1, current_col));
    }
    if current_row < num_rows - 1 {
        adjacent_positions.push((current_row + 1, current_col));
    }
    if current_col > 0 {
        adjacent_positions.push((current_row, current_col - 1));
    }
    if current_col < num_cols - 1 {
        adjacent_positions.push((current_row, current_col + 1));
    }

    for (row, col) in adjacent_positions {
        let can_traverse_to_adjacent_position =
            grid[row][col].abs_diff(current_elevation) <= 1 || current_elevation > grid[row][col];

        if can_traverse_to_adjacent_position
            && shortest_paths[row][col].map_or(true, |num_steps| {
                num_steps_from_current_position_to_adjacent_position < num_steps
            })
        {
            // println!(
            //     "walking from {:?} to {:?} because {} < {:?} and we can go from {} to {}",
            //     (current_row, current_col),
            //     (row, col),
            //     num_steps_from_current_position_to_adjacent_position,
            //     shortest_paths[row][col],
            //     current_elevation,
            //     grid[row][col]
            // );

            shortest_paths[row][col] = Some(num_steps_from_current_position_to_adjacent_position);

            // _print_grid(&shortest_paths);

            recursively_solve((row, col), grid, shortest_paths);
        }
    }
}

pub fn part_two(input: &str) -> usize {
    // this is going to be basically the same as part_one except I want to iterate over all possible starting positions and find the shortest path among them. I'm not going to bother with optimizations, but one improvement would be to abort the recursive function the moment we see a favorable step to another 'a'.
    // A better strategy would probably be to compose a list of all possible starting positions and then tweak the recursive function to also return a list of all 'a' positions that were visited with the one closest to the end standing out (like the first element in the list if you sort it): if you do that, you can immediately discount all of the other starting positions that showed up in that list and reassess the shortest path to the end from that 'a' that's closest. Then you can keep iterating over the list of unseen potential start positions

    let (_start, end, grid) = parse_input(input);

    grid.iter()
        .enumerate()
        .map(|(row_index, row)| {
            // this is a problem for another day, but there are ownership problems with this higher-order-function implementation
            // row.iter()
            //     .enumerate()
            //     .filter(|(_, elevation)| **elevation == 'a' as u32)
            //     .map(|(col_index, _)| get_shortest_path((row_index, col_index), end, &grid))

            let mut v = vec![];
            for (col_index, _) in row
                .iter()
                .enumerate()
                // .inspect(|(col_index, elevation)| {
                //     println!("({}, {}): {}", row_index, col_index, elevation)
                // })
                .filter(|(_, elevation)| **elevation == 'a' as u32)
            {
                if let Some(shortest_path) = get_shortest_path((row_index, col_index), end, &grid) {
                    // println!("considering start ({}, {})", row_index, col_index);
                    v.push(shortest_path);
                }
            }
            v
        })
        .flatten()
        .min()
        .unwrap()
}
