fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            (chars[0], chars[2])
        })
        .collect()
}

// rps 123
// win 6, draw 3, loss 0
pub fn part_one(input: &str) -> usize {
    let rounds = parse_input(input);
    rounds.iter().fold(0, |acc, round| {
        let pity_points = match round.1 {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("unrecognized value in second column"),
        };
        let victory_points = match round {
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ => panic!("jinkies"),
        };
        acc + pity_points + victory_points
    })
}

// X lose, Y draw, Z win
pub fn part_two(input: &str) -> usize {
    let rounds = parse_input(input);
    rounds.iter().fold(0, |acc, round| {
        let victory_points = match round.1 {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("unrecognized value in second column"),
        };
        let pity_points = match round {
            ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3,
            ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
            ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2,
            _ => panic!("jinkies"),
        };
        acc + victory_points + pity_points
    })
}
