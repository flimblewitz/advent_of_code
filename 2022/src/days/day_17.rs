#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

#[derive(Clone, Debug)]
enum Rock {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}
impl Rock {
    fn width(&self) -> usize {
        match self {
            Rock::Horizontal => 4,
            Rock::Plus => 3,
            Rock::L => 3,
            Rock::Vertical => 1,
            Rock::Square => 2,
        }
    }
    fn fits(&self, x: usize, y: usize, chamber: &Vec<[bool; 7]>) -> bool {
        x + self.width() - 1 < 7
            && match self {
                Rock::Horizontal => !chamber[y][x..x + 4].iter().any(|occupied| *occupied),
                Rock::Plus => {
                    !chamber[y + 1][x..x + 3].iter().any(|occupied| *occupied)
                        && !chamber[y..y + 3].iter().any(|row| row[x + 1])
                }
                Rock::L => {
                    !chamber[y][x..x + 3].iter().any(|occupied| *occupied)
                        && !chamber[y..y + 3].iter().any(|row| row[x + 2])
                }
                Rock::Vertical => !chamber[y..y + 4].iter().any(|row| row[x]),
                Rock::Square => {
                    !chamber[y][x..x + 2].iter().any(|occupied| *occupied)
                        && !chamber[y + 1][x..x + 2].iter().any(|occupied| *occupied)
                }
            }
    }

    fn settle_in_chamber(&self, x: usize, y: usize, chamber: &mut Vec<[bool; 7]>) {
        match self {
            Rock::Horizontal => chamber[y][x..x + 4]
                .iter_mut()
                .for_each(|occupied| *occupied = true),
            Rock::Plus => {
                chamber[y + 1][x..x + 3]
                    .iter_mut()
                    .for_each(|occupied| *occupied = true);
                chamber[y..y + 3]
                    .iter_mut()
                    .for_each(|row| row[x + 1] = true);
            }
            Rock::L => {
                chamber[y][x..x + 3]
                    .iter_mut()
                    .for_each(|occupied| *occupied = true);
                chamber[y..y + 3]
                    .iter_mut()
                    .for_each(|row| row[x + 2] = true);
            }
            Rock::Vertical => chamber[y..y + 4].iter_mut().for_each(|row| row[x] = true),
            Rock::Square => {
                chamber[y][x..x + 2]
                    .iter_mut()
                    .for_each(|occupied| *occupied = true);
                chamber[y + 1][x..x + 2]
                    .iter_mut()
                    .for_each(|occupied| *occupied = true)
            }
        }
    }
}

fn height_of_tower(chamber: &Vec<[bool; 7]>) -> usize {
    chamber.iter().filter(|row| row.iter().any(|b| *b)).count()

    // the latter is faster but harder to read

    // chamber
    //     .iter()
    //     .enumerate()
    //     .rev()
    //     .find(|(_, row)| row.iter().any(|b| *b))
    //     .map(|(index, _)| index + 1)
    //     .unwrap_or(0)
}
fn _print_chamber(chamber: &Vec<[bool; 7]>) {
    let tile_as_char = |b: bool| if b { '#' } else { '.' };
    chamber.iter().rev().for_each(|row| {
        print!("|");
        row.iter().for_each(|b| print!("{}", tile_as_char(*b)));
        print!("|");
        println!("");
    });
    println!("+-------+");
    println!("");
}

fn parse_input(input: &str) -> (impl Iterator<Item = Jet> + '_, impl Iterator<Item = Rock>) {
    let jets = input
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            c => panic!("unrecognized input char {c}"),
        })
        .cycle();

    let rocks = vec![
        Rock::Horizontal,
        Rock::Plus,
        Rock::L,
        Rock::Vertical,
        Rock::Square,
    ]
    .into_iter()
    .cycle();

    (jets, rocks)
}

pub fn part_one(input: &str) -> usize {
    let (mut jets, mut rocks) = parse_input(input);
    // println!("{:?}", jets);

    // we're going to use cartesian coordinates for this
    // x is left to right
    // y is bottom to top
    let mut chamber = vec![[false; 7]; 4];

    for rock in (0..2022).map(|_| rocks.next().unwrap()) {
        // Each rock appears so that
        // - its left edge is two units away from the left wall
        // - its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one)
        let mut x = 2;
        let mut y = 3 + height_of_tower(&chamber);
        loop {
            // println!("rock {:?} is at {}, {}", rock, x, y);
            match jets.next().unwrap() {
                Jet::Left => {
                    // println!("left");
                    if x > 0 && rock.fits(x - 1, y, &chamber) {
                        // println!("left!!!");
                        x -= 1
                    }
                }
                Jet::Right => {
                    // println!("right");
                    if rock.fits(x + 1, y, &chamber) {
                        // println!("right!!!");
                        x += 1
                    }
                }
            }
            if y > 0 && rock.fits(x, y - 1, &chamber) {
                y -= 1;
            } else {
                rock.settle_in_chamber(x, y, &mut chamber);
                // let's maintain a buffer for more rocks to spawn in
                // 3 is the amount of space needed between the top of the tower and the next rock
                // 4 is the max height of any rock
                if height_of_tower(&chamber) + 3 + 4 > chamber.len() {
                    (0..4).for_each(|_| chamber.push([false; 7]))
                }
                // _print_chamber(&chamber);
                break;
            }
        }
    }

    // _print_chamber(&chamber);
    height_of_tower(&chamber)
}

pub fn part_two(input: &str) -> usize {
    let (mut jets, mut rocks) = parse_input(input);
    todo!("I tried discarding excess memory by greedily lopping off the bottom of the tower continuously starting at height 2000 just to see what would happen, but, well 1000000000000 is a big number. The solution must involve identifying a cycle in the ongoing structure of the tower and then doing something analogous to modulo division to find the answer without having to churn through every iteration of rock-laying")
}
