fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().fold(vec![vec![]], |mut acc, line| {
        match line.parse::<usize>() {
            Ok(num) => acc.last_mut().unwrap().push(num),
            _ => acc.push(vec![]),
        }
        acc
    })
}

pub fn part_one(input: &str) -> usize {
    let lists_of_food_items = parse_input(input);
    lists_of_food_items
        .iter()
        .map(|list| list.iter().sum())
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> usize {
    let lists_of_food_items = parse_input(input);
    let mut lists_of_total_calories: Vec<usize> = lists_of_food_items
        .iter()
        .map(|list| list.iter().sum())
        .collect();
    lists_of_total_calories.sort_unstable();
    lists_of_total_calories.iter().rev().take(3).sum()
}
