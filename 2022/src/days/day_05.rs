fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let num_stacks = (input.lines().next().unwrap().len() + 1) / 4;

    let mut lines = input.lines();

    let mut supply_crate_stacks = vec![vec![]; num_stacks];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut chars = line.chars();
        chars.next(); // [
        let mut i = 0;
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                break;
            }
            if c != ' ' {
                supply_crate_stacks[i].insert(0, c);
            }
            i += 1;
            chars.next(); // ]
            chars.next(); // space
            chars.next(); // [
        }
    }

    let mut crane_instructions = vec![];
    while let Some(line) = lines.next() {
        let mut instructions = line
            .split_whitespace()
            .filter_map(|sub_str| sub_str.parse::<usize>().ok());
        crane_instructions.push((
            instructions.next().unwrap(),
            instructions.next().unwrap(),
            instructions.next().unwrap(),
        ));
    }

    (supply_crate_stacks, crane_instructions)
}

pub fn part_one(input: &str) -> String {
    let (mut supply_crate_stacks, crane_instructions) = parse_input(input);

    for (quantity, from, to) in crane_instructions {
        (0..quantity).for_each(|_| {
            let supply_crate = supply_crate_stacks[from - 1].pop().unwrap();
            supply_crate_stacks[to - 1].push(supply_crate);
        });
    }

    supply_crate_stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

pub fn part_two(input: &str) -> String {
    let (mut supply_crate_stacks, crane_instructions) = parse_input(input);

    for (quantity, from, to) in crane_instructions {
        let from_stack = &mut supply_crate_stacks[from - 1];
        let mut supply_crates = from_stack.split_off(from_stack.len() - quantity);
        supply_crate_stacks[to - 1].append(&mut supply_crates);
    }

    supply_crate_stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}
