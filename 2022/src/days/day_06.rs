pub fn part_one(input: &str) -> usize {
    get_index_after_first_unique_window(input, 4)
}

pub fn part_two(input: &str) -> usize {
    get_index_after_first_unique_window(input, 14)
}

fn get_index_after_first_unique_window(input: &str, window_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let window_start_index = chars
        .windows(window_size)
        .enumerate()
        .find(|(_window_start_index, window)| {
            (0..(window_size - 1)).all(|i| !window[(i + 1)..window_size].contains(&window[i]))
        })
        .unwrap()
        .0;
    window_start_index + window_size
}
