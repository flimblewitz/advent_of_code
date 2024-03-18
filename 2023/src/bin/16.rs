advent_of_code::solution!(16);

enum Tile {
    Empty,
    Slash,
    Backslash,
    Vertical,
    Horizontal,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            _ => panic!("um excuse me what is this char? {c}"),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::new).collect())
        .collect();

    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut energized = vec![vec![false; col_count]; row_count];

    // these beams totally loop, so we need to make sure we detect repetition and drop it
    let mut beam_history: Vec<Vec<Option<Vec<Direction>>>> = vec![vec![None; col_count]; row_count];

    // the first usize is the row index, the second usize is the column index
    let mut beam_origins: Vec<(Direction, usize, usize)> = vec![(Direction::Right, 0, 0)];

    while let Some((mut direction, mut r, mut c)) = beam_origins.pop() {
        loop {
            if let Some(tile_beam_history) = &mut beam_history[r][c] {
                if tile_beam_history.contains(&direction) {
                    // we've already been here
                    break;
                }
                tile_beam_history.push(direction.clone());
            } else {
                beam_history[r][c] = Some(vec![direction.clone()]);
            }

            energized[r][c] = true;

            let can_go_up = r > 0;
            let can_go_down = r + 1 < row_count;
            let can_go_left = c > 0;
            let can_go_right = c + 1 < col_count;

            // it's probably more efficient to use functions with arguments than closures, but whatever
            // fun fact: I first tried using "FnMut" closures that directly mutated direction, r, and c, but that doesn't work because those variables are all immutably borrowed in the match statement below
            let go_up = || (Direction::Up, r - 1, c);
            let go_down = || (Direction::Down, r + 1, c);
            let go_left = || (Direction::Left, r, c - 1);
            let go_right = || (Direction::Right, r, c + 1);

            // it's moments like these that make me remember why I like "match" so much
            (direction, r, c) = match (&grid[r][c], direction.clone()) {
                (Tile::Empty | Tile::Vertical, Direction::Up)
                | (Tile::Slash, Direction::Right)
                | (Tile::Backslash, Direction::Left)
                    if can_go_up =>
                {
                    go_up()
                }
                (Tile::Empty | Tile::Vertical, Direction::Down)
                | (Tile::Slash, Direction::Left)
                | (Tile::Backslash, Direction::Right)
                    if can_go_down =>
                {
                    go_down()
                }
                (Tile::Empty | Tile::Horizontal, Direction::Left)
                | (Tile::Slash, Direction::Down)
                | (Tile::Backslash, Direction::Up)
                    if can_go_left =>
                {
                    go_left()
                }
                (Tile::Empty | Tile::Horizontal, Direction::Right)
                | (Tile::Slash, Direction::Up)
                | (Tile::Backslash, Direction::Down)
                    if can_go_right =>
                {
                    go_right()
                }
                (Tile::Vertical, Direction::Left | Direction::Right) => {
                    if can_go_up {
                        beam_origins.push(go_up());
                    }
                    if can_go_down {
                        beam_origins.push(go_down());
                    }
                    break;
                }
                (Tile::Horizontal, Direction::Up | Direction::Down) => {
                    if can_go_left {
                        beam_origins.push(go_left());
                    }
                    if can_go_right {
                        beam_origins.push(go_right());
                    }
                    break;
                }
                _ => break,
            };
        }
    }

    Some(
        energized
            .iter()
            // .inspect(|row| {
            //     row.iter()
            //         .for_each(|e| if *e { print!("#") } else { print!(".") });
            //     println!()
            // })
            .map(|row| row.iter().filter(|e| **e).count())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::new).collect())
        .collect();

    let row_count = grid.len();
    let col_count = grid[0].len();

    // all I had to do was factor out the energized tile counting into a function and run it for every possible starting position
    // this smells like a dynamic programming problem at first, but I just don't see a feasible way to break it into subproblems
    // so brute force it is. At least it's only 4*log(n) beam origins to consider

    Some(
        (0..row_count)
            .map(|r| (Direction::Right, r, 0))
            .chain((0..row_count).map(|r| (Direction::Left, r, col_count - 1)))
            .chain((0..col_count).map(|c| (Direction::Down, 0, c)))
            .chain((0..col_count).map(|c| (Direction::Up, row_count - 1, c)))
            .map(
                |(starting_direction, starting_row_index, starting_col_index)| {
                    count_energized_tiles(
                        &grid,
                        starting_direction,
                        starting_row_index,
                        starting_col_index,
                    )
                },
            )
            .max()
            .unwrap(),
    )
}

fn count_energized_tiles(
    grid: &[Vec<Tile>],
    starting_direction: Direction,
    starting_row_index: usize,
    starting_col_index: usize,
) -> usize {
    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut energized = vec![vec![false; col_count]; row_count];

    // these beams totally loop, so we need to make sure we detect repetition and drop it
    let mut beam_history: Vec<Vec<Option<Vec<Direction>>>> = vec![vec![None; col_count]; row_count];

    // the first usize is the row index, the second usize is the column index
    let mut beam_origins: Vec<(Direction, usize, usize)> =
        vec![(starting_direction, starting_row_index, starting_col_index)];

    while let Some((mut direction, mut r, mut c)) = beam_origins.pop() {
        loop {
            if let Some(tile_beam_history) = &mut beam_history[r][c] {
                if tile_beam_history.contains(&direction) {
                    // we've already been here
                    break;
                }
                tile_beam_history.push(direction.clone());
            } else {
                beam_history[r][c] = Some(vec![direction.clone()]);
            }

            energized[r][c] = true;

            let can_go_up = r > 0;
            let can_go_down = r + 1 < row_count;
            let can_go_left = c > 0;
            let can_go_right = c + 1 < col_count;

            // it's probably more efficient to use functions with arguments than closures, but whatever
            // fun fact: I first tried using "FnMut" closures that directly mutated direction, r, and c, but that doesn't work because those variables are all immutably borrowed in the match statement below
            let go_up = || (Direction::Up, r - 1, c);
            let go_down = || (Direction::Down, r + 1, c);
            let go_left = || (Direction::Left, r, c - 1);
            let go_right = || (Direction::Right, r, c + 1);

            // it's moments like these that make me remember why I like "match" so much
            (direction, r, c) = match (&grid[r][c], direction.clone()) {
                (Tile::Empty | Tile::Vertical, Direction::Up)
                | (Tile::Slash, Direction::Right)
                | (Tile::Backslash, Direction::Left)
                    if can_go_up =>
                {
                    go_up()
                }
                (Tile::Empty | Tile::Vertical, Direction::Down)
                | (Tile::Slash, Direction::Left)
                | (Tile::Backslash, Direction::Right)
                    if can_go_down =>
                {
                    go_down()
                }
                (Tile::Empty | Tile::Horizontal, Direction::Left)
                | (Tile::Slash, Direction::Down)
                | (Tile::Backslash, Direction::Up)
                    if can_go_left =>
                {
                    go_left()
                }
                (Tile::Empty | Tile::Horizontal, Direction::Right)
                | (Tile::Slash, Direction::Up)
                | (Tile::Backslash, Direction::Down)
                    if can_go_right =>
                {
                    go_right()
                }
                (Tile::Vertical, Direction::Left | Direction::Right) => {
                    if can_go_up {
                        beam_origins.push(go_up());
                    }
                    if can_go_down {
                        beam_origins.push(go_down());
                    }
                    break;
                }
                (Tile::Horizontal, Direction::Up | Direction::Down) => {
                    if can_go_left {
                        beam_origins.push(go_left());
                    }
                    if can_go_right {
                        beam_origins.push(go_right());
                    }
                    break;
                }
                _ => break,
            };
        }
    }

    energized
        .iter()
        // .inspect(|row| {
        //     row.iter()
        //         .for_each(|e| if *e { print!("#") } else { print!(".") });
        //     println!()
        // })
        .map(|row| row.iter().filter(|e| **e).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
