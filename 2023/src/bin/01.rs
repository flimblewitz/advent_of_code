advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let digit_chars: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
                let calibration_value: String = format!(
                    "{}{}",
                    digit_chars.first().unwrap(),
                    digit_chars.last().unwrap(),
                );
                let calibration_value: u32 = calibration_value.parse().unwrap();
                // println!("{calibration_value}");
                calibration_value
            })
            .sum(),
    )
}

#[derive(Default)]
struct FirstAndLastDigitsInLine {
    first_digit_index: Option<usize>,
    first_digit: u32,
    last_digit_index: Option<usize>,
    last_digit: u32,
}

pub fn part_two(input: &str) -> Option<u32> {
    // zero isn't allowed
    let digits: Vec<(u32, &str)> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, s)| ((i + 1) as u32, s))
    .collect();

    // for each line, for each digit, let's find the indexes of their first and last occurrences
    Some(
        input
            .lines()
            .map(|line| {
                // println!();
                // println!("{line}");
                // we want to use "fold" to accumulate a best answer for the given line as we iterate through the digits
                let FirstAndLastDigitsInLine {
                    first_digit,
                    last_digit: second_digit,
                    ..
                } = digits.iter().fold(
                    FirstAndLastDigitsInLine::default(),
                    |mut acc, (digit, digit_string)| {
                        // for each digit, we have to look for its first and last occurrences in the line as just the stringified digit (e.g. '1') or as the digit's string spelling (e.g. "one")
                        // luckily rust has convenient string methods "find" and "rfind" for this
                        // we can't use the "min" and "max" Option methods to compare those speculative indexes because None is considered less than any Some(...), so we'll have to use match statements

                        // println!("looking for first occurrence of digit {digit} (or {digit_string}) in {line}");
                        let first_digit_occurrence_index = match (
                            line.find(char::from_digit(*digit, 10).unwrap()),
                            line.find(digit_string),
                        ) {
                            (None, None) => None,
                            (None, Some(i)) | (Some(i), None) => Some(i),
                            (Some(a), Some(b)) => Some(a.min(b)),
                        };
                        // println!("first_digit_occurrence_index: {first_digit_occurrence_index}");
                        if let Some(i) = first_digit_occurrence_index {
                            if acc.first_digit_index.is_none() || i < acc.first_digit_index.unwrap()
                            {
                                // println!("changing first digit to {digit} because it's at index {digit_occurrence_index}");
                                acc.first_digit_index = first_digit_occurrence_index;
                                acc.first_digit = *digit;
                            }
                        }

                        let last_digit_occurrence_index = match (
                            line.rfind(char::from_digit(*digit, 10).unwrap()),
                            line.rfind(digit_string),
                        ) {
                            (None, None) => None,
                            (None, Some(i)) | (Some(i), None) => Some(i),
                            (Some(a), Some(b)) => Some(a.max(b)),
                        };
                        // println!("last_digit_occurrence_index: {last_digit_occurrence_index}");
                        if let Some(i) = last_digit_occurrence_index {
                            if acc.last_digit_index.is_none() || i > acc.last_digit_index.unwrap() {
                                // println!("changing last digit to {digit} because it's at index {digit_occurrence_index}");
                                acc.last_digit_index = last_digit_occurrence_index;
                                acc.last_digit = *digit;
                            }
                        }
                        acc
                    },
                );
                // println!("{first_digit}{second_digit}");
                first_digit * 10 + second_digit
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
