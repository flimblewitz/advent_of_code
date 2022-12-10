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

    let trimmed_input = raw_input.trim_end();

    match day {
        1 => solve!(day_01, &trimmed_input),
        2 => solve!(day_02, &trimmed_input),
        3 => solve!(day_03, &trimmed_input),
        4 => solve!(day_04, &trimmed_input),
        5 => solve!(day_05, &trimmed_input),
        6 => solve!(day_06, &trimmed_input),
        // 7 => solve!(day_07, &trimmed_input),
        8 => solve!(day_08, &trimmed_input),
        // 9 => solve!(day_09, &trimmed_input),
        // 10 => solve!(day10, &trimmed_input),
        // 11 => solve!(day11, &trimmed_input),
        // 12 => solve!(day12, &trimmed_input),
        // 13 => solve!(day13, &trimmed_input),
        // 14 => solve!(day14, &trimmed_input),
        // 15 => solve!(day15, &trimmed_input),
        // 16 => solve!(day16, &trimmed_input),
        // 17 => solve!(day17, &trimmed_input),
        // 18 => solve!(day18, &trimmed_input),
        // 19 => solve!(day19, &trimmed_input),
        // 20 => solve!(day20, &trimmed_input),
        // 21 => solve!(day21, &trimmed_input),
        // 22 => solve!(day22, &trimmed_input),
        // 23 => solve!(day23, &trimmed_input),
        // 24 => solve!(day24, &trimmed_input),
        // 25 => solve!(day25, &trimmed_input),
        _ => println!("unrecognized day: {}", day),
    }
}
