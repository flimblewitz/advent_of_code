advent_of_code::solution!(5);

#[derive(Clone, Debug)]
struct MapRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}
#[derive(Clone, Debug)]
struct Map(Vec<MapRange>);
impl Map {
    fn new(lines: &[&str]) -> Self {
        Self(
            lines
                .iter()
                .skip(1)
                .map(|line| {
                    let mut numbers = line
                        .split_whitespace()
                        .map(|number| number.parse().unwrap());
                    // //println!("numbers: {:?}", numbers);
                    MapRange {
                        destination_range_start: numbers.next().unwrap(),
                        source_range_start: numbers.next().unwrap(),
                        range_length: numbers.next().unwrap(),
                    }
                })
                .collect(),
        )
    }

    fn get_destination(&self, source: u64) -> u64 {
        if let Some(mr) = self.0.iter().find(|mr| {
            source >= mr.source_range_start && source <= mr.source_range_start + mr.range_length - 1
        }) {
            mr.destination_range_start + (source - mr.source_range_start)
        } else {
            source
        }
    }

    fn get_source(&self, destination: u64) -> u64 {
        if let Some(mr) = self.0.iter().find(|mr| {
            destination >= mr.destination_range_start
                && destination < mr.destination_range_start + mr.range_length
        }) {
            mr.source_range_start + (destination - mr.destination_range_start)
        } else {
            destination
        }
    }
}

fn get_seed_location(seed: u64, map_chain: &[Map]) -> u64 {
    map_chain
        .iter()
        .fold(seed, |acc, map| map.get_destination(acc))
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let mut blocks = lines.split(|line| line.is_empty());
    // blocks
    //     .clone()
    //     .for_each(|block| //println!("block: {:?}", block));

    let seeds: Vec<u64> = blocks
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();
    //println!("seeds: {:?}", seeds);

    let seed_to_soil = Map::new(blocks.next().unwrap());
    //println!("{:?}", seed_to_soil);

    let soil_to_fertilizer = Map::new(blocks.next().unwrap());
    //println!("{:?}", soil_to_fertilizer);

    let fertilizer_to_water = Map::new(blocks.next().unwrap());
    //println!("{:?}", fertilizer_to_water);

    let water_to_light = Map::new(blocks.next().unwrap());
    //println!("{:?}", water_to_light);

    let light_to_temperature = Map::new(blocks.next().unwrap());
    //println!("{:?}", light_to_temperature);

    let temperature_to_humidity = Map::new(blocks.next().unwrap());
    //println!("{:?}", temperature_to_humidity);

    let humidity_to_location = Map::new(blocks.next().unwrap());
    //println!("{:?}", humidity_to_location);

    let map_chain = vec![
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];
    Some(
        seeds
            .into_iter()
            .map(|seed| get_seed_location(seed, &map_chain))
            .min()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let mut blocks = lines.split(|line| line.is_empty());
    // blocks
    //     .clone()
    //     .for_each(|block| //println!("block: {:?}", block));

    let numbers: Vec<u64> = blocks
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    let seed_ranges: Vec<_> = numbers
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();
    //println!("seed_ranges: {:?}", seed_ranges);

    let seed_to_soil = Map::new(blocks.next().unwrap());
    //println!("{:?}", seed_to_soil);

    let soil_to_fertilizer = Map::new(blocks.next().unwrap());
    //println!("{:?}", soil_to_fertilizer);

    let fertilizer_to_water = Map::new(blocks.next().unwrap());
    //println!("{:?}", fertilizer_to_water);

    let water_to_light = Map::new(blocks.next().unwrap());
    //println!("{:?}", water_to_light);

    let light_to_temperature = Map::new(blocks.next().unwrap());
    //println!("{:?}", light_to_temperature);

    let temperature_to_humidity = Map::new(blocks.next().unwrap());
    //println!("{:?}", temperature_to_humidity);

    let humidity_to_location = Map::new(blocks.next().unwrap());
    //println!("{:?}", humidity_to_location);

    let mut map_chain = vec![
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    // let's assume that all of the given maps have contiguous ranges. So the only omitted ranges are possibly 0:N (where N is one less than the lowest source number among the map's ranges, assuming that lowest source number is greater than 0) and M:infinity (where M is the first source number that lies outside of any of the map's ranges)

    // since the actual seed ranges are enormous, I can't brute force this and iterate over every possible seed
    // instead, I suspect that I can make things more reasonable by working BACKWARDS
    // this is still brute force, but it doesn't take unreasonably long (27.6 seconds for me)

    map_chain.reverse();

    (0u64..).find(|location| {
        let seed = map_chain
            .iter()
            .fold(*location, |destination, map| map.get_source(destination));
        seed_ranges.iter().any(|sr| sr.contains(&seed))
    })

    // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
