advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace();
    let records: Vec<(u32, u32)> = times
        .zip(distances)
        .map(|(time, distance)| (time.parse().unwrap(), distance.parse().unwrap()))
        .collect();
    // println!("records {:?}", records);

    Some(
        records
            .into_iter()
            .map(|(time, distance)| {
                // f(charge_duration) = charge_duration * (time - charge_duration) = distance
                // we want to find charge_duration
                // 0 = -(charge_duration)^2 + time * charge_duration - distance
                // the graph forms a downward parabola
                // this is the quadratic formula. There are two "true" answers that may be decimals, and every value between them will be a charge_duration that beats the current record's distance
                // however, we only want the integer charge_duration values, so we'll round the quadratic formula's two answers up and down respectively to the closest EXCLUSIVE integers and count how many such integers there are
                // the reason why we need to round the two answers to the closest EXCLUSIVE integers and not INCLUSIVE integers is that the two answers may coincidentally already be integers, in which case they're not going to beat the current record's distance
                let answers = solve_quadratic_formula_rounding_inward(time, distance);
                answers.0 - answers.1 + 1
            })
            .product(),
    )
}

fn solve_quadratic_formula_rounding_inward(
    time: impl Into<f64>,
    distance: impl Into<f64>,
) -> (u64, u64) {
    let time: f64 = time.into();
    let distance: f64 = distance.into();
    println!();
    // println!("time: {}, distance to beat: {}", time, distance);
    // println!("f(x) = -x^2 + {}x - {}", time, distance);
    let sqrt_b_squared_minus_4ac = (time.powi(2) - 4f64 * -1f64 * (-1f64 * distance)).sqrt();
    // println!("sqrt(b^2 - 4ac) = {}", sqrt_b_squared_minus_4ac);
    let first_quadratic_formula_answer = (-1f64 * time - sqrt_b_squared_minus_4ac) / -2f64;
    let first_quadratic_formula_answer =
        match first_quadratic_formula_answer == first_quadratic_formula_answer.floor() {
            true => first_quadratic_formula_answer - 1f64,
            false => first_quadratic_formula_answer.floor(),
        } as u64;
    // println!(
    //     "first_quadratic_formula_answer: {}",
    //     first_quadratic_formula_answer
    // );
    let second_quadratic_formula_answer = (-1f64 * time + sqrt_b_squared_minus_4ac) / -2f64;
    let second_quadratic_formula_answer =
        match second_quadratic_formula_answer == second_quadratic_formula_answer.ceil() {
            true => second_quadratic_formula_answer + 1f64,
            false => second_quadratic_formula_answer.ceil(),
        } as u64;
    // println!(
    //     "second_quadratic_formula_answer: {}",
    //     second_quadratic_formula_answer
    // );

    (
        first_quadratic_formula_answer,
        second_quadratic_formula_answer,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time: String = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let time: u32 = time.parse().unwrap();

    let distance: String = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    // I'm making this an f64 because it's too big to fit as a u32 and a u64 can't be converted into an f64
    let distance: f64 = distance.parse().unwrap();

    // None
    let answers = solve_quadratic_formula_rounding_inward(time, distance);
    Some(answers.0 - answers.1 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
