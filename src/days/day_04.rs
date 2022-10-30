type Tile = (u32, bool);
#[derive(Debug)]
struct Board(Vec<Vec<Tile>>);

impl Board {
    // todo just take the vecs and turn them into a 2d 5x5 array, blowing up if it doesn't fit
    fn new(board_string: &str) -> Self {
        Board(
            board_string
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| (x.parse().unwrap(), false))
                        .collect::<Vec<Tile>>()
                })
                .collect(),
        )
    }
    fn apply_number(&mut self, number: &u32) {
        self.0.iter_mut().for_each(|row| {
            row.iter_mut()
                .for_each(|column| column.1 = column.1 || *number == column.0)
        })
    }

    // todo use rayon to make this faster
    // todo think about fancy enumeration tricks
    fn has_won(&self) -> bool {
        // check each row, each column, and each diagonal
        let board_height = self.0.len();

        let a_row_has_won = (0..board_height)
            .any(|row_index| self.0.get(row_index).unwrap().iter().all(|column| column.1));

        // they're squares, so there's no danger in blindly using the number of rows as the number of columns
        let a_column_has_won = (0..board_height)
            .any(|column_index| self.0.iter().all(|row| row.get(column_index).unwrap().1));

        a_row_has_won || a_column_has_won
    }

    fn score(&self, winning_number: &u32) -> u32 {
        let sum_of_unmarked_board_numbers = self.0.iter().fold(0, |acc, row| {
            acc + row.iter().fold(
                0,
                |acc, &column| if !column.1 { acc + column.0 } else { acc },
            )
        });
        sum_of_unmarked_board_numbers * winning_number
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let numbers = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let boards = input
        .split("\n\n")
        // skip the numbers line
        .skip(1)
        .map(|board_string| Board::new(board_string))
        .collect();

    (numbers, boards)
}

pub fn part_one(input: &str) -> u32 {
    let (numbers, mut boards) = parse_input(input);

    let mut is_game_over = false;
    let mut final_score = 0;

    for number in numbers.iter() {
        boards.iter_mut().for_each(|board| {
            board.apply_number(number);
            if board.has_won() {
                is_game_over = true;
                final_score = board.score(number);
            }
        });

        if is_game_over {
            break;
        }
    }

    final_score
}

pub fn part_two(input: &str) -> u32 {
    let (numbers, mut boards) = parse_input(input);

    // let mut boards = HashSet::from_iter(boards);

    let mut final_score = 0;

    let mut indexes_of_winning_boards = Vec::new();
    for number in numbers.iter() {
        boards.iter_mut().enumerate().for_each(|(index, board)| {
            if !indexes_of_winning_boards.contains(&index) {
                board.apply_number(number);
                if board.has_won() {
                    final_score = board.score(number);
                    indexes_of_winning_boards.push(index);
                }
            }
        });
    }

    final_score
}
