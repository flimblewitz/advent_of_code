use regex::Regex;
use std::collections::HashMap;

const MINUTES_UNTIL_ERUPTION: usize = 30;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    open: bool,
    flow_rate: usize,
    adjacent_valves: Vec<(String, usize)>,
}

fn parse_input(input: &str) -> (HashMap<String, usize>, Vec<Valve>) {
    let regex_for_line =
        Regex::new(r"Valve (\S\S) has flow rate=(\d+); (?:tunnels lead|tunnel leads) to valves? (\S\S)((?:, \S\S)*)")
            .unwrap();

    let regex_for_extra_adjacent_valves = Regex::new(r", (\S\S)").unwrap();

    let (names_to_indexes, mut valves) = regex_for_line.captures_iter(input).enumerate().fold(
        (HashMap::new(), vec![]),
        |(mut names_to_indexes, mut valves), (index, line_captures)| {
            // println!("regex recognizes a line! {:?}", line_captures);
            let name = line_captures[1].to_owned();

            let flow_rate: usize = line_captures[2].parse().unwrap();

            let mut adjacent_valve_names = vec![line_captures[3].to_owned()];

            // println!("line_captures[3]: {}", &line_captures[4]);

            regex_for_extra_adjacent_valves
                .captures_iter(&line_captures[4].to_owned())
                .for_each(|extra_adjacent_valve_captures| {
                    adjacent_valve_names.push(extra_adjacent_valve_captures[1].to_owned())
                });

            names_to_indexes.insert(name.clone(), index);

            let adjacent_valves_without_indexes = adjacent_valve_names
                .into_iter()
                .map(|avn| (avn, 0))
                .collect();

            valves.push(Valve {
                name,
                open: false,
                flow_rate,
                adjacent_valves: adjacent_valves_without_indexes,
            });

            (names_to_indexes, valves)
        },
    );

    valves.iter_mut().for_each(|valve| {
        valve.adjacent_valves.iter_mut().for_each(|adjacent_valve| {
            adjacent_valve.1 = *names_to_indexes.get(&adjacent_valve.0).unwrap()
        })
    });

    (names_to_indexes, valves)
}

fn part_one_recursively_get_max_pressure_released(
    minutes_left: usize,
    valves: &mut Vec<Valve>,
    starting_valve_index: usize,
    sub_solutions: &mut HashMap<(usize, Vec<Valve>, usize), usize>,
) -> usize {
    if let Some(sub_solution) =
        sub_solutions.get(&(minutes_left, valves.clone(), starting_valve_index))
    {
        // println!(
        //     "HIT: {}, {}, {:?}",
        //     minutes_left, starting_valve_index, valves
        // );
        return *sub_solution;
    }

    // I can abort the moment all valves are released
    // I can also abort if there's no time left to open any more valves
    if valves.iter().all(|v| v.open || v.flow_rate == 0) || minutes_left <= 2 {
        // println!(
        //     "{}no more pressure can be released",
        //     (minutes_left..MINUTES_UNTIL_ERUPTION)
        //         .map(|_| ' ')
        //         .collect::<String>(),
        // );
        return 0;
    }

    let Valve {
        name: _name,
        open,
        adjacent_valves,
        flow_rate,
    } = valves.get(starting_valve_index).unwrap().clone();

    // println!(
    //     "{}with {} minutes_left, starting at valve {}",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION)
    //         .map(|_| ' ')
    //         .collect::<String>(),
    //     minutes_left,
    //     _name
    // );

    // println!(
    //     "{}if we don't open the valve...",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION)
    //         .map(|_| ' ')
    //         .collect::<String>()
    // );

    let mut max_pressure_released = adjacent_valves
        .clone()
        .into_iter()
        .map(|(_, adjacent_valve_index)| {
            part_one_recursively_get_max_pressure_released(
                minutes_left - 1,
                valves,
                adjacent_valve_index,
                sub_solutions,
            )
        })
        .max()
        .unwrap();

    // only consider opening the valve if it's not already open and it can release pressure and we have time to actually get any benefit out of it
    if !open && flow_rate > 0 && minutes_left >= 2 {
        let pressure_released_by_opening_this_valve = flow_rate * (minutes_left - 1);
        // println!(
        //     "{}if we DO open the valve with {} minutes left and a flow rate of {}, we get an additional future pressure release of {}",
        //     (minutes_left..MINUTES_UNTIL_ERUPTION)
        //         .map(|_| ' ')
        //         .collect::<String>(),
        //     minutes_left,
        //     flow_rate,
        //     pressure_released_by_opening_this_valve
        // );

        // mark the valve as open so that further recursion doesn't consider opening it again
        valves[starting_valve_index].open = true;

        let max_pressure_released_after_opening_valve = adjacent_valves
            .clone()
            .iter()
            .map(|(_, adjacent_valve_index)| {
                part_one_recursively_get_max_pressure_released(
                    minutes_left - 2,
                    valves,
                    *adjacent_valve_index,
                    sub_solutions,
                )
            })
            .max()
            .unwrap();

        // now that we've explored all possible futures, let's close the valve so that other timelines can still open it
        valves[starting_valve_index].open = false;

        max_pressure_released = max_pressure_released.max(
            pressure_released_by_opening_this_valve + max_pressure_released_after_opening_valve,
        );
    }

    // println!(
    //     "{}after {} minutes, starting at valve {}, the max pressure to release is {}\n",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION)
    //         .map(|_| ' ')
    //         .collect::<String>(),
    //     minutes_left,
    //     starting_valve_name,
    //     max_pressure_released
    // );

    sub_solutions.insert(
        (minutes_left, valves.clone(), starting_valve_index),
        max_pressure_released,
    );

    max_pressure_released
}

pub fn part_one(input: &str) -> usize {
    // return 0;
    println!("be warned that part 1 takes several minutes for the real input");
    let (names_to_indexes, mut valves) = parse_input(input);
    // println!("valves: {:?}", valves);

    let mut sub_solutions: HashMap<(usize, Vec<Valve>, usize), usize> = HashMap::new();

    part_one_recursively_get_max_pressure_released(
        MINUTES_UNTIL_ERUPTION,
        &mut valves,
        *names_to_indexes.get("AA").unwrap(),
        &mut sub_solutions,
    )
}
pub fn part_two(input: &str) -> usize {
    todo!()
}
