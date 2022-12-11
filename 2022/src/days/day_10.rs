#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.contains("noop") {
                return Instruction::Noop;
            }
            let x: i32 = line.trim_start_matches("addx ").parse().unwrap();
            Instruction::Addx(x)
        })
        .collect()
}

fn try_update_signal_strengths(cycle: i32, x: i32, signal_strengths: &mut Vec<i32>) {
    if cycle == 20 || cycle > 0 && (cycle - 20) % 40 == 0 {
        // println!("beep: {}", cycle * x);
        signal_strengths.push(cycle * x);
    }
}
pub fn part_one(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut cycle = 1;
    let mut x = 1;
    let mut signal_strengths = vec![];
    for instruction in instructions {
        try_update_signal_strengths(cycle, x, &mut signal_strengths);
        match instruction {
            Instruction::Noop => cycle += 1,
            Instruction::Addx(delta) => {
                cycle += 1;
                try_update_signal_strengths(cycle, x, &mut signal_strengths);
                cycle += 1;
                x += delta;
            }
        }
    }

    signal_strengths.into_iter().sum()
}

fn draw(cycle: i32, x: i32, crt_screen: &mut String) {
    if (cycle) % 40 == 0 {
        crt_screen.push('\n');
    }
    if cycle % 40 >= x - 1 && cycle % 40 <= x + 1 {
        // println!("cycle: {}, x: {}: #", cycle, x);
        crt_screen.push('#');
    } else {
        // println!("cycle: {}, x: {}: .", cycle, x);
        crt_screen.push('.');
    }
}
pub fn part_two(input: &str) -> String {
    let instructions = parse_input(input);
    // 0-counting the cycle makes this a lot easier
    let mut cycle = 0;
    let mut x = 1;
    let mut crt_screen = String::new();
    for instruction in instructions {
        draw(cycle, x, &mut crt_screen);
        match instruction {
            Instruction::Noop => cycle += 1,
            Instruction::Addx(delta) => {
                cycle += 1;
                draw(cycle, x, &mut crt_screen);
                cycle += 1;
                x += delta;
            }
        }
    }
    crt_screen
}
