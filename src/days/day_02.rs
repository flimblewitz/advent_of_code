use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u64},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

// this is overkill, but I wanted an excuse to attain a rudimentary understanding of a parser crate
fn parse_command(line: &str) -> Command {
    // I have to use this intermediate variable for a compiler reason that that still escapes me
    let result: IResult<&str, (&str, _, u64)> =
        tuple((alt((tag("forward"), tag("down"), tag("up"))), space1, u64))(line);
    let (_input, command_tuple) = result.unwrap();

    match command_tuple {
        ("forward", _, value) => Command::Forward(value),
        ("down", _, value) => Command::Down(value),
        ("up", _, value) => Command::Up(value),
        _ => panic!("unrecognized command: {command_tuple:?}"),
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(parse_command).collect()
}

pub fn part_one(input: &str) -> i64 {
    let commands = parse_input(input);

    let mut horizontal_position: i64 = 0;
    let mut depth: i64 = 0;

    commands.iter().for_each(|command| match command {
        Command::Forward(v) => horizontal_position += *v as i64,
        Command::Down(v) => depth += *v as i64,
        Command::Up(v) => depth -= *v as i64,
    });
    horizontal_position * depth
}

pub fn part_two(input: &str) -> i64 {
    let commands = parse_input(input);
    let mut horizontal_position: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;
    commands.iter().for_each(|command| match command {
        Command::Forward(v) => {
            horizontal_position += *v as i64;
            depth += *v as i64 * aim;
        }
        Command::Down(v) => aim += *v as i64,
        Command::Up(v) => aim -= *v as i64,
    });
    horizontal_position * depth
}
