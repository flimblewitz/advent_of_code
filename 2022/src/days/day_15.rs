use regex::Regex;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    range: i64,
}
impl Sensor {
    fn can_sense(&self, (x, y): (i64, i64)) -> bool {
        self.range >= (self.x.abs_diff(x) + self.y.abs_diff(y)) as i64
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Beacon {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> (Vec<Sensor>, Vec<Beacon>) {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let sensors_and_beacons: (Vec<Sensor>, Vec<Beacon>) =
        re.captures_iter(input)
            .fold((vec![], vec![]), |(mut sensors, mut beacons), captures| {
                // println!("regex recognizes a line! {:?}", captures);
                let sensor_x: i64 = captures[1].parse().unwrap();
                let sensor_y: i64 = captures[2].parse().unwrap();
                let beacon_x: i64 = captures[3].parse().unwrap();
                let beacon_y: i64 = captures[4].parse().unwrap();

                let manhattan_distance =
                    (sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y)) as i64;

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

pub fn part_one(input: &str) -> u64 {
    let (sensors, beacons) = parse_input(input);
    // println!("sensors: {:?}", sensors);
    // println!("beacons: {:?}", beacons);

    // so now that I have a list of sensors, I can take the given row of interest, 2000000, and map each sensor to the range of x indexes in that row that are within range of that sensor
    // let y_of_interest: i64 = 10;
    let y_of_interest: i64 = 2000000;

    let ranges_of_x_indexes_visible_to_sensors = sensors.iter().filter_map(|s| {
        let abs_y_distance = y_of_interest.abs_diff(s.y) as i64;
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
    let disparate_ranges_of_x_indexes_visible_to_sensors: Vec<(i64, i64)> =
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
        beacons.iter().filter(|b| b.y == y_of_interest).count() as u64;
    // println!(
    //     "number_of_beacons_at_y_of_interest: {}",
    //     number_of_beacons_at_y_of_interest
    // );

    number_of_x_indexes_visible_to_sensors - number_of_beacons_at_y_of_interest
}

// const MAX_X_AND_Y: i64 = 20;
const MAX_X_AND_Y: i64 = 4000000;

fn get_tuning_value_of_coord_if_hidden(coord: (i64, i64), sensors: &[Sensor]) -> Option<i64> {
    if coord.0 >= 0 && coord.0 <= MAX_X_AND_Y && coord.1 >= 0 && coord.1 <= MAX_X_AND_Y {
        // println!("considering coord {:?}", coord);
        if !sensors.iter().any(|s| {
            let sensed = s.can_sense(coord);
            // println!("sensor {:?} can sense it: {}", s, sensed);
            sensed
        }) {
            return Some(get_tuning_value(coord));
        }
    }
    None
}

fn get_tuning_value((x, y): (i64, i64)) -> i64 {
    x * 4000000 + y
}

pub fn part_two(input: &str) -> i64 {
    let (sensors, _) = parse_input(input);

    // if there's just one coord outside of every sensor's range, that means we don't have to actually scan over 16 million coords: we only have to scan over all the coords immediately adjacent to the ones at the edge of each sensor's range
    // for each sensor, for each in-bounds coord immediately adjacent to its range, check if it's in range of any other sensor
    // if it's not in range of any other sensor, we've found our hidden coord and can return its tuning value
    // else, if we've just looked at every coord adjacent to the given sensor's range, remove the sensor from consideration and move onto the next sensor
    for sensor in sensors.iter() {
        // println!("considering sensor {:?}", sensor);
        for y in 0.max(sensor.y - sensor.range)..=MAX_X_AND_Y.min(sensor.y + sensor.range) {
            let y_dist_from_sensor = sensor.y.abs_diff(y) as i64;
            // println!("considering y_dist_from_sensor: {}", y_dist_from_sensor);

            // check both x coords at this y
            let x_dist_from_sensor = sensor.range - y_dist_from_sensor;
            // println!("considering x_dist_from_sensor: {}", x_dist_from_sensor);

            let left_x = sensor.x - x_dist_from_sensor;
            // println!("considering in-range left coord ({}, {})", left_x, y);
            let coord_left_of_left_x = (left_x - 1, y);

            let right_x = sensor.x + x_dist_from_sensor;
            // println!("considering in-range right coord ({}, {})", right_x, y);
            let coord_right_of_right_x = (right_x + 1, y);

            let coords = vec![coord_left_of_left_x, coord_right_of_right_x];
            // println!("considering coords {:?}", coords);

            if let Some(tuning_value) = coords
                .iter()
                .filter_map(|coord| get_tuning_value_of_coord_if_hidden(*coord, &sensors))
                .next()
            {
                return tuning_value;
            }
        }
    }

    panic!("failed to find the only possible position for the distress beacon")
}
