use regex::Regex;
use std::collections::HashMap;

const MINUTES_UNTIL_ERUPTION: usize = 30;
const MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT: usize = 26;

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

fn part_two_recursively_get_max_pressure_released(
    minutes_left: usize,
    valves: &mut Vec<Valve>,
    starting_valve_indexes: [usize; 2],
    sub_solutions: &mut HashMap<(usize, Vec<Valve>, [usize; 2]), (usize, String)>,
) -> (usize, String) {
    if let Some(sub_solution) =
        sub_solutions.get(&(minutes_left, valves.clone(), starting_valve_indexes))
    {
        // println!(
        //     "{}memoized: {}",
        //     (minutes_left..MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT)
        //         .map(|_| ' ')
        //         .collect::<String>(),
        //     sub_solution
        // );

        return sub_solution.clone();
    }

    let valve_1 = valves[starting_valve_indexes[0]].clone();
    let valve_2 = valves[starting_valve_indexes[1]].clone();

    let current_minute = MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT - minutes_left + 1;
    let open_valves: Vec<&Valve> = valves.iter().filter(|v| v.open).collect();
    let open_valve_names: Vec<&String> = open_valves.iter().map(|v| &v.name).collect();
    let open_valve_flow_rate_sum: usize = open_valves.iter().map(|v| v.flow_rate).sum();
    let report = format!(
        r"== Minute {} ==
Valves {:?} are open, releasing {} pressure
First entity is at valve {}
Second entity is at valve {}
    ",
        current_minute, open_valve_names, open_valve_flow_rate_sum, valve_1.name, valve_2.name
    );

    if valves.iter().all(|v| v.open || v.flow_rate == 0) || minutes_left <= 2 {
        // println!(
        //     "{}no more pressure can be released",
        //     (minutes_left..MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT)
        //         .map(|_| ' ')
        //         .collect::<String>(),
        // );
        return (0, report);
    }

    // println!(
    //     "{}with {} minutes_left, starting at valves {} and {}",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT)
    //         .map(|_| ' ')
    //         .collect::<String>(),
    //     minutes_left,
    //     valve_1.name,
    //     valve_2.name,
    // );

    // there are four possibilities:
    // 1. both distinct valves are opened
    // 2. only valve_1 is opened
    // 2. only valve_2 is opened
    // 3. zero valves are opened

    let all_adjacent_valve_indexes_combinations: Vec<[usize; 2]> = valve_1
        .adjacent_valves
        .iter()
        .map(|(_, adjacent_valve_1_index)| {
            valve_2
                .adjacent_valves
                .iter()
                .map(|(_, adjacent_valve_2_index)| {
                    let mut next_valve_indexes = [*adjacent_valve_1_index, *adjacent_valve_2_index];
                    // it's important to sort each of these "tuples" because that way we can actually make optimal use of the memoization
                    // there's no need to care who's who between santa and the elephant; all that matters is that one is at valve X and one is at valve Y
                    // sorting allows us to care about the combination of valves that they're at instead of the permutation
                    next_valve_indexes.sort();
                    next_valve_indexes
                })
                .collect()
        })
        .collect::<Vec<Vec<[usize; 2]>>>()
        .into_iter()
        .flatten()
        .collect();

    // println!(
    //     "all_adjacent_valve_indexes_combinations: {:?}",
    //     all_adjacent_valve_indexes_combinations
    // );

    let solution_if_opening_two_valves = if !valve_1.open
        && valve_1.flow_rate > 0
        && !valve_2.open
        && valve_2.flow_rate > 0
        && valve_1.name != valve_2.name
        && minutes_left >= 2
    {
        let pressure_released_by_opening_valve_1 = valve_1.flow_rate * (minutes_left - 1);
        let pressure_released_by_opening_valve_2 = valve_2.flow_rate * (minutes_left - 1);

        valves[starting_valve_indexes[0]].open = true;
        valves[starting_valve_indexes[1]].open = true;

        let (max_pressure_released_afterward, report_afterward) =
            all_adjacent_valve_indexes_combinations
                .iter()
                .map(|adjacent_valve_indexes_combination| {
                    part_two_recursively_get_max_pressure_released(
                        minutes_left - 2,
                        valves,
                        *adjacent_valve_indexes_combination,
                        sub_solutions,
                    )
                })
                .max_by_key(|s| s.0)
                .unwrap();

        let max_pressure_released_by_opening_valves = pressure_released_by_opening_valve_1
            + pressure_released_by_opening_valve_2
            + max_pressure_released_afterward;

        valves[starting_valve_indexes[0]].open = false;
        valves[starting_valve_indexes[1]].open = false;

        (
            max_pressure_released_by_opening_valves,
            format!("{}\n{}", report, report_afterward),
        )
    } else {
        (0, "".into())
    };

    // println!(
    //     "{}max_pressure_released_if_opening_two_valves: {}",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT)
    //         .map(|_| ' ')
    //         .collect::<String>(),
    //     max_pressure_released_if_opening_two_valves
    // );

    // todo: is my minutes_left math right?
    let solution_if_opening_valve_1 = if !valve_1.open && valve_1.flow_rate > 0 && minutes_left >= 2
    {
        let pressure_released_by_opening_valve_1 = valve_1.flow_rate * (minutes_left - 1);

        valves[starting_valve_indexes[0]].open = true;

        let valve_indexes_combinations_for_valve_2_adjacent_indexes: Vec<[usize; 2]> = valve_2
            .adjacent_valves
            .iter()
            .map(|(_, adjacent_valve_2_index)| {
                let mut next_valve_indexes = [starting_valve_indexes[0], *adjacent_valve_2_index];
                // it's important to sort each of these "tuples" because that way we can actually make optimal use of the memoization
                // there's no need to care who's who between santa and the elephant; all that matters is that one is at valve X and one is at valve Y
                // sorting allows us to care about the combination of valves that they're at instead of the permutation
                next_valve_indexes.sort();
                next_valve_indexes
            })
            .collect();

        let (max_pressure_released_afterward, report_afterward) =
            valve_indexes_combinations_for_valve_2_adjacent_indexes
                .iter()
                .map(|valve_indexes_combination| {
                    part_two_recursively_get_max_pressure_released(
                        minutes_left - 1,
                        valves,
                        *valve_indexes_combination,
                        sub_solutions,
                    )
                })
                .max_by_key(|s| s.0)
                .unwrap();

        let max_pressure_released_after_opening_valve_1 =
            pressure_released_by_opening_valve_1 + max_pressure_released_afterward;

        valves[starting_valve_indexes[0]].open = false;

        (
            max_pressure_released_after_opening_valve_1,
            format!("{}\n{}", report, report_afterward),
        )
    } else {
        (0, "".into())
    };

    // println!(
    //     "{}max_pressure_released_if_opening_valve_1: {}",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT)
    //         .map(|_| ' ')
    //         .collect::<String>(),
    //     max_pressure_released_if_opening_valve_1
    // );

    // todo: is my minutes_left math right?
    let solution_if_opening_valve_2 = if !valve_2.open && valve_2.flow_rate > 0 && minutes_left >= 2
    {
        let pressure_released_by_opening_valve_2 = valve_2.flow_rate * (minutes_left - 1);

        valves[starting_valve_indexes[1]].open = true;

        let valve_indexes_combinations_for_valve_1_adjacent_indexes: Vec<[usize; 2]> = valve_1
            .adjacent_valves
            .iter()
            .map(|(_, adjacent_valve_1_index)| {
                let mut next_valve_indexes = [*adjacent_valve_1_index, starting_valve_indexes[1]];
                // it's important to sort each of these "tuples" because that way we can actually make optimal use of the memoization
                // there's no need to care who's who between santa and the elephant; all that matters is that one is at valve X and one is at valve Y
                // sorting allows us to care about the combination of valves that they're at instead of the permutation
                next_valve_indexes.sort();
                next_valve_indexes
            })
            .collect();

        let (max_pressure_released_afterward, report_afterward) =
            valve_indexes_combinations_for_valve_1_adjacent_indexes
                .iter()
                .map(|valve_indexes_combination| {
                    part_two_recursively_get_max_pressure_released(
                        minutes_left - 1,
                        valves,
                        *valve_indexes_combination,
                        sub_solutions,
                    )
                })
                .max_by_key(|s| s.0)
                .unwrap();

        let max_pressure_released_after_opening_valve_2 =
            pressure_released_by_opening_valve_2 + max_pressure_released_afterward;

        valves[starting_valve_indexes[1]].open = false;

        (
            max_pressure_released_after_opening_valve_2,
            format!("{}\n{}", report, report_afterward),
        )
    } else {
        (0, "".into())
    };

    // println!(
    //     "{}max_pressure_released_if_opening_valve_2: {}",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT)
    //         .map(|_| ' ')
    //         .collect::<String>(),
    //     max_pressure_released_if_opening_valve_2
    // );

    let mut solution_after_opening_zero_valves = all_adjacent_valve_indexes_combinations
        .iter()
        .map(|adjacent_valve_indexes_combination| {
            part_two_recursively_get_max_pressure_released(
                minutes_left - 1,
                valves,
                *adjacent_valve_indexes_combination,
                sub_solutions,
            )
        })
        .max_by_key(|s| s.0)
        .unwrap();
    solution_after_opening_zero_valves.1 =
        format!("{}\n{}", report, solution_after_opening_zero_valves.1);

    // println!(
    //     "{}max_pressure_released_after_opening_zero_valves: {}",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT)
    //         .map(|_| ' ')
    //         .collect::<String>(),
    //     max_pressure_released_after_opening_zero_valves
    // );

    // let solution = solution_if_opening_two_valves
    //     .max(solution_if_opening_valve_1)
    //     .max(solution_if_opening_valve_2)
    //     .max(solution_after_opening_zero_valves);
    let potential_solutions = vec![
        solution_if_opening_two_valves,
        solution_if_opening_valve_1,
        solution_if_opening_valve_2,
        solution_after_opening_zero_valves,
    ];
    let solution = potential_solutions
        .iter()
        .max_by_key(|(pressure_released, _)| pressure_released)
        .unwrap();

    // println!(
    //     "{}... max_pressure_released: {}",
    //     (minutes_left..MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT)
    //         .map(|_| ' ')
    //         .collect::<String>(),
    //     max_pressure_released
    // );

    sub_solutions.insert(
        (minutes_left, valves.clone(), starting_valve_indexes),
        solution.clone(),
    );

    solution.clone()
}

pub fn part_two(input: &str) -> usize {
    let (names_to_indexes, mut valves) = parse_input(input);

    let mut sub_solutions: HashMap<(usize, Vec<Valve>, [usize; 2]), (usize, String)> =
        HashMap::new();

    let starting_valve_index = *names_to_indexes.get("AA").unwrap();

    let (pressure_released, _report) = part_two_recursively_get_max_pressure_released(
        MINUTES_UNTIL_ERUPTION_WITH_ELEPHANT,
        &mut valves,
        [starting_valve_index, starting_valve_index],
        &mut sub_solutions,
    );

    // println!("{}", _report);

    pressure_released
}
