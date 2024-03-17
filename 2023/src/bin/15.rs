advent_of_code::solution!(15);

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim().split(',').map(|step| hash(step)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lens_boxes: Vec<Option<Vec<(String, usize)>>> = vec![None; 256];
    // I could use a fold here, but I have a hunch that it might be inefficient for each iteration of the fold to keep moving the whole vec from one iteration to the next, over and over
    input.trim().split(',').for_each(|step| {
        let mut step_split_around_operation = step.split(|c| c == '-' || c == '=');
        let label = step_split_around_operation.next().unwrap();
        let operation = step.chars().skip(label.len()).next().unwrap();
        let box_index = hash(&label);
        match operation {
            '-' => {
                if let Some(lens_box) = &mut lens_boxes[box_index] {
                    if let Some(i) = (0..lens_box.len()).find(|i| lens_box[*i].0 == label) {
                        lens_box.remove(i);
                        // it's not really necessary to "clean up" like this, but it'll make things nicer to print and may potentially reduce storage costs (it's a gamble for negligible benefit)
                        if lens_box.is_empty() {
                            lens_boxes[box_index] = None;
                        }
                    }
                }
            }
            '=' => {
                let focal_length = step_split_around_operation.next().unwrap().parse().unwrap();
                let lens = (label.to_owned(), focal_length);
                match &mut lens_boxes[box_index] {
                    Some(lens_box) => {
                        if let Some(i) = (0..lens_box.len()).find(|i| lens_box[*i].0 == label) {
                            lens_box[i] = lens;
                        } else {
                            lens_box.push(lens)
                        }
                    }
                    None => lens_boxes[box_index] = Some(vec![lens]),
                };
            }
            _ => panic!("unexpected operation {} for step {}", operation, step),
        }
    });

    // lens_boxes
    //     .iter()
    //     .enumerate()
    //     .filter(|(_, lens_box)| lens_box.is_some())
    //     .for_each(|(i, lens_box)| println!("Box {}: {:?}", i, lens_box));

    Some(
        lens_boxes
            .into_iter()
            .enumerate()
            .filter_map(|(box_index, lens_box)| {
                lens_box.map(|lens_box| {
                    lens_box
                        .into_iter()
                        .enumerate()
                        .map(|(i, (_, focal_length))| (1 + box_index) * (1 + i) * focal_length)
                        .sum::<usize>()
                })
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
