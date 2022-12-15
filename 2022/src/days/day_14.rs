#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}
impl Tile {
    fn _print(&self) {
        let c = match self {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
        };
        print!("{}", c);
    }
}

// let's produce a grid of air and rocks and the starting x coord for sand to drop (I know it's supposed to be 500, but I want to normalize it)
fn parse_input(input: &str) -> (Vec<Vec<Tile>>, usize) {
    let mut point_sequences: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut points = point.split(",");
                    (
                        points.next().unwrap().parse().unwrap(),
                        points.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();
    // println!("original point_sequences");
    // point_sequences
    //     .iter()
    //     .for_each(|point_sequence| println!("{:?}", point_sequence));

    let smallest_x = *point_sequences
        .iter()
        .map(|point_sequence| point_sequence.iter().map(|(x, _)| x).min().unwrap())
        .min()
        .unwrap()
        .min(&500); // we want to include the sand-dropping x coord
                    // println!("smallest_x: {}", smallest_x);

    let largest_x = *point_sequences
        .iter()
        .map(|point_sequence| point_sequence.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap();
    // println!("largest_x: {}", largest_x);

    // let smallest_y = *point_sequences
    //     .iter()
    //     .map(|point_sequence| point_sequence.iter().map(|(_, y)| y).min().unwrap())
    //     .min()
    //     .unwrap(); // we want to include the sand-dropping x coord
    // println!("smallest_y: {}", smallest_y);

    let largest_y = *point_sequences
        .iter()
        .map(|point_sequence| point_sequence.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap();
    // println!("largest_y: {}", largest_y);

    let normalized_x_from_which_sand_falls = 500 - smallest_x;

    let mut grid = vec![vec![Tile::Air; largest_x - smallest_x + 1]; largest_y + 1];

    // let's normalize those x coords
    point_sequences.iter_mut().for_each(|point_sequence| {
        point_sequence
            .iter_mut()
            .for_each(|point| point.0 -= smallest_x)
    });

    // point_sequences.iter().for_each(|point_sequence| {
    //     point_sequence
    //         .iter()
    //         .for_each(|(x, y)| grid[*y][*x - smallest_x] = Tile::Rock)
    // });
    point_sequences.iter().for_each(|point_sequence| {
        (1..point_sequence.len()).for_each(|i| match (point_sequence[i - 1], point_sequence[i]) {
            ((x_1, y_1), (x_2, y_2)) if x_1 == x_2 => {
                // println!("iterating y from {} to {} with fixed x {}", y_1, y_2, x_1);
                (y_1.min(y_2)..=y_1.max(y_2)).for_each(|y| grid[y][x_1] = Tile::Rock)
            }
            ((x_1, y_1), (x_2, y_2)) if y_1 == y_2 => {
                // println!("iterating x from {} to {} with fixed y {}", x_1, x_2, y_1);
                (x_1.min(x_2)..=x_1.max(x_2)).for_each(|x| grid[y_1][x] = Tile::Rock)
            }
            _ => panic!("a point sequence doesn't represent horizontal/vertical lines"),
        });
    });

    (grid, normalized_x_from_which_sand_falls)
}

fn pour_sand(grid: &mut Vec<Vec<Tile>>, x_from_which_sand_falls: usize) {
    'pour_sand_grains: loop {
        // grid.iter().for_each(|row| {
        //     row.iter().for_each(|tile| tile._print());
        //     println!("");
        // });

        let (mut x, mut y) = (x_from_which_sand_falls, 0);

        // if sand can no longer drop, abort
        if grid[y][x] != Tile::Air {
            break;
        }
        // println!("a new grain of sand!");
        loop {
            // println!("x: {}, y: {}", x, y);
            // sand has settled when there is rock/sand below it AND (its left is out of bounds OR there is rock/sand below and left) AND (its right is out of bounds OR there is rock/sand below and right)
            let tile_down = grid.get(y + 1).and_then(|row| row.get(x));
            let tile_down_left = grid.get(y + 1).and_then(|row| {
                x.checked_sub(1)
                    .and_then(|x_minus_one| row.get(x_minus_one))
            });
            let tile_down_right = grid.get(y + 1).and_then(|row| row.get(x + 1));
            match (tile_down_left, tile_down, tile_down_right) {
                (_, Some(Tile::Air), _) => y += 1,
                (Some(Tile::Air), _, _) => {
                    y += 1;
                    x -= 1;
                }
                (_, _, Some(Tile::Air)) => {
                    y += 1;
                    x += 1;
                }
                // the grain of sand has settled if sand/rock makes up every tile below us
                (
                    Some(Tile::Rock) | Some(Tile::Sand),
                    Some(Tile::Rock) | Some(Tile::Sand),
                    Some(Tile::Rock) | Some(Tile::Sand),
                ) => {
                    grid[y][x] = Tile::Sand;
                    break;
                }
                // anything else should mean sand is pouring into the abyss
                _ => break 'pour_sand_grains,
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let (mut grid, x_from_which_sand_falls) = parse_input(input);

    // println!(
    //     "normalized_x_from_which_sand_falls: {}",
    //     normalized_x_from_which_sand_falls
    // );

    pour_sand(&mut grid, x_from_which_sand_falls);

    grid.iter()
        .map(|row| row.iter().filter(|tile| **tile == Tile::Sand).count())
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let (mut grid, mut x_from_which_sand_falls) = parse_input(input);

    // the grid has to be expanded to a width of 1+2*(num_rows+2), and the x from which sand falls has to be reset to the middle of it
    // I need to make sure that x_from_which_sand_falls becomes the middle x of the grid (odd number of columns)
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let num_cols_desired_left_of_sand_drop_x = (num_rows + 2) - 1;
    // this will blow up if there's "useless" rock way off to the left, but let's assume that's not the case
    let num_cols_to_add_left = num_cols_desired_left_of_sand_drop_x - x_from_which_sand_falls;

    let num_cols_desired_right_of_sand_drop_x = (num_rows + 2) - 1;
    let num_cols_right_of_sand_drop_x = num_cols - x_from_which_sand_falls - 1;
    // this will blow up if there's "useless" rock way off to the left, but let's assume that's not the case
    let num_cols_to_add_right =
        num_cols_desired_right_of_sand_drop_x - num_cols_right_of_sand_drop_x;

    // add the empty columns to the left and right of the grid
    grid.iter_mut().for_each(|row| {
        (0..num_cols_to_add_left).for_each(|_| row.insert(0, Tile::Air));
        (0..num_cols_to_add_right).for_each(|_| row.push(Tile::Air));
    });

    // the num_cols has now changed
    let num_cols = grid[0].len();

    // let's add the empty rows of air and rock respectively
    grid.push(vec![Tile::Air; num_cols]);
    grid.push(vec![Tile::Rock; num_cols]);

    x_from_which_sand_falls = num_cols / 2;
    // println!(
    //     "x_from_which_sand_falls has been adjusted: {}",
    //     x_from_which_sand_falls
    // );

    pour_sand(&mut grid, x_from_which_sand_falls);

    grid.iter()
        .map(|row| row.iter().filter(|tile| **tile == Tile::Sand).count())
        .sum()
}
