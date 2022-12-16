use regex::Regex;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}
#[derive(Clone, Debug, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> (Vec<Sensor>, Vec<Beacon>) {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let sensors_and_beacons: (Vec<Sensor>, Vec<Beacon>) =
        re.captures_iter(input)
            .fold((vec![], vec![]), |(mut sensors, mut beacons), captures| {
                // println!("regex recognizes a line! {:?}", captures);
                let sensor_x: i32 = captures[1].parse().unwrap();
                let sensor_y: i32 = captures[2].parse().unwrap();
                let beacon_x: i32 = captures[3].parse().unwrap();
                let beacon_y: i32 = captures[4].parse().unwrap();

                let manhattan_distance =
                    (sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y)) as i32;

                sensors.push(Sensor {
                    x: sensor_x,
                    y: sensor_y,
                    range: manhattan_distance,
                });

                let beacon = Beacon {
                    x: beacon_x,
                    y: beacon_y,
                };
                if !beacons.contains(&beacon) {
                    beacons.push(beacon);
                }

                (sensors, beacons)
            });

    sensors_and_beacons
}

pub fn part_one(input: &str) -> u32 {
    let (sensors, beacons) = parse_input(input);
    // println!("sensors: {:?}", sensors);
    // println!("beacons: {:?}", beacons);

    // so now that I have a list of sensors, I can take the given row of interest, 2000000, and map each sensor to the range of x indexes in that row that are within range of that sensor
    // let y_of_interest: i32 = 10;
    let y_of_interest: i32 = 2000000;

    let ranges_of_x_indexes_visible_to_sensors = sensors.iter().filter_map(|s| {
        let abs_y_distance = y_of_interest.abs_diff(s.y) as i32;
        let x_range_on_row_of_interest = s.range - abs_y_distance;
        match abs_y_distance <= s.range {
            true => {
                // println!(
                //     "range visible by sensor at ({}, {}) with range {}: {} to {}",
                //     s.x,
                //     s.y,
                //     s.range,
                //     s.x - x_range_on_row_of_interest,
                //     s.x + x_range_on_row_of_interest
                // );
                Some((
                    s.x - x_range_on_row_of_interest,
                    s.x + x_range_on_row_of_interest,
                ))
            }
            false => None,
        }
    });

    // let's combine the ones that are overlapping
    let disparate_ranges_of_x_indexes_visible_to_sensors: Vec<(i32, i32)> =
        ranges_of_x_indexes_visible_to_sensors.fold(
            vec![],
            |mut ranges, range_of_x_indexes_visible_to_a_particular_sensor| {
                let mut new_range_to_consider = range_of_x_indexes_visible_to_a_particular_sensor;

                loop {
                    if let Some((i, _)) = ranges.iter().enumerate().find(|(_, &range)| {
                        range.0 <= new_range_to_consider.1 && range.1 >= new_range_to_consider.0
                    }) {
                        // if there's some overlapping range in the vec, pull it out, merge it with this new one, and loop again with this new melded range in mind
                        let overlapping_range = ranges.remove(i);
                        new_range_to_consider = (
                            overlapping_range.0.min(new_range_to_consider.0),
                            overlapping_range.1.max(new_range_to_consider.1),
                        );
                        continue;
                    }
                    break;
                }

                ranges.push(new_range_to_consider);

                ranges
            },
        );
    // println!(
    //     "disparate_ranges_of_x_indexes_visible_to_sensors: {:?}",
    //     disparate_ranges_of_x_indexes_visible_to_sensors
    // );

    let number_of_x_indexes_visible_to_sensors = disparate_ranges_of_x_indexes_visible_to_sensors
        .into_iter()
        .fold(0, |acc, (min_x, max_x)| acc + max_x.abs_diff(min_x) + 1);
    // println!(
    //     "number_of_x_indexes_visible_to_sensors: {}",
    //     number_of_x_indexes_visible_to_sensors
    // );

    let number_of_beacons_at_y_of_interest =
        beacons.iter().filter(|b| b.y == y_of_interest).count() as u32;
    // println!(
    //     "number_of_beacons_at_y_of_interest: {}",
    //     number_of_beacons_at_y_of_interest
    // );

    number_of_x_indexes_visible_to_sensors - number_of_beacons_at_y_of_interest
}

pub fn part_two(input: &str) -> i32 {
    let (sensors, _) = parse_input(input);

    // let max_x_and_y = 20;
    let max_x_and_y = 4000000;

    panic!("part two isn't yet working for real input");

    for x in 0..=max_x_and_y {
        for y in 0..=max_x_and_y {
            if !sensors.iter().any(|s| {
                let is_in_range = ((s.x.abs_diff(x) + s.y.abs_diff(y)) as i32) <= s.range;
                println!(
                    "({}, {}) in range of sensor at ({}, {}) with range {}: {}",
                    x, y, s.x, s.y, s.range, is_in_range
                );
                is_in_range
            }) {
                // println!(
                //     "coordinate ({}, {}) is outside the range of any sensor",
                //     x, y
                // );
                return x * 4000000 + y;
            }
        }
    }

    panic!("failed to find the only possible position for the distress beacon")
}
