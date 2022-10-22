use std::env;
use std::fs;
mod days;

use days::*;

macro_rules! solve {
    ($day:path, $input:expr) => {{
        use $day::{part_one, part_two};
        println!("part 1: {}", part_one($input));
        println!("part 2: {}", part_two($input));
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u8 = args[1].parse().unwrap();

    let raw_input = fs::read_to_string(format!("src/days/day_{day:02}_input.txt")).unwrap();

    let trimmed_input = raw_input.trim();

    match day {
        1 => solve!(day_01, &trimmed_input),
        // 2 => solve!(day_02, input),
        // 3 => solve!(day_03, input),
        // 4 => solve!(day_04, input),
        // 5 => solve!(day_05, input),
        // 6 => solve!(day_06, input),
        // 7 => solve!(day_07, input),
        // 8 => solve!(day_08, input),
        // 9 => solve!(day_09, input),
        // 10 => solve!(day10, input),
        // 11 => solve!(day11, input),
        // 12 => solve!(day12, input),
        // 13 => solve!(day13, input),
        // 14 => solve!(day14, input),
        // 15 => solve!(day15, input),
        // 16 => solve!(day16, input),
        // 17 => solve!(day17, input),
        // 18 => solve!(day18, input),
        // 19 => solve!(day19, input),
        // 20 => solve!(day20, input),
        // 21 => solve!(day21, input),
        // 22 => solve!(day22, input),
        // 23 => solve!(day23, input),
        // 24 => solve!(day24, input),
        // 25 => solve!(day25, input),
        _ => println!("unrecognized day: {}", day),
    }
}
