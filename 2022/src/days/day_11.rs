struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    test_divisor: usize, // I hope this helps part 2
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_block| {
            let mut lines = monkey_block.lines().skip(1);
            let starting_items: Vec<usize> = lines
                .next()
                .unwrap()
                .trim_start_matches("  Starting items: ")
                .replace(",", "")
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let mut operation_tokens = lines
                .next()
                .unwrap()
                .trim_start_matches("  Operation: new = old ")
                .split_whitespace();
            let operator = operation_tokens.next().unwrap();
            let operand = operation_tokens.next().unwrap();
            let operation: Box<dyn Fn(usize) -> usize> =
                match (operator, operand == "old", operand.parse::<usize>()) {
                    ("*", true, _) => Box::new(|x| x * x),
                    ("*", _, Ok(c)) => Box::new(move |x| x * c),
                    ("+", true, _) => Box::new(|x| x + x),
                    ("+", _, Ok(c)) => Box::new(move |x| x + c),
                    _ => panic!("nani?"),
                };
            let test_divisor: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("  Test: divisible by ")
                .parse()
                .unwrap();
            let test_true_monkey_index: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("    If true: throw to monkey ")
                .parse()
                .unwrap();
            let test_false_monkey_index: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("    If false: throw to monkey ")
                .parse()
                .unwrap();
            let test = Box::new(move |x: usize| {
                if x % test_divisor == 0 {
                    test_true_monkey_index
                } else {
                    test_false_monkey_index
                }
            });
            Monkey {
                items: starting_items,
                operation,
                test,
                test_divisor,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let mut monkeys = parse_input(input);

    // monkey inspects its first item
    // monkey performs operation to increase the item's worry value
    // we divide the worry value by 3
    // monkey performs test to get index of monkey to receive the item
    // monkey throws item - with its mutated worry value - to the monkey whose index was determined by the test
    // the monkey repeats all of that for its next items until it has no items
    // repeat all of that for each monkey

    let mut monkey_actions = vec![0; monkeys.len()];

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            // println!("\nmonkey {i}");
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.remove(0);
                // println!("inspects {item}");
                item = (*monkeys[i].operation)(item) / 3;
                // println!("worry {item}");
                let index_of_receiving_monkey = (*monkeys[i].test)(item);
                // println!("throw to {index_of_receiving_monkey}");
                monkeys[index_of_receiving_monkey].items.push(item);
                monkey_actions[i] += 1;
            }
        }
    }

    // let mi: Vec<Vec<usize>> = monkeys.iter().map(|m| m.items.clone()).collect();
    // println!("{:?}", mi);

    monkey_actions.sort();
    monkey_actions.iter().rev().take(2).product()
    // .fold(1, |acc, e| acc * e)
}

pub fn part_two(input: &str) -> usize {
    let mut monkeys = parse_input(input);

    // monkey inspects its first item
    // monkey performs operation to increase the item's worry value
    // unlike part one, we don't divide the worry value by 3. But we need to avoid overflow without inhibiting the monkeys' tests. To do that, let's modulo divide the worry value by product_of_all_test_divisors! This should work because each monkey's test is just doing a modulo division of its own with some factor of that product, and if we can subtract some multiple of that product from a quantity, the result will be something for which "part of the work is already done" when the monkey's test performs a modulo division with its own divisor
    // monkey performs test to get index of monkey to receive the item
    // monkey throws item - with its mutated worry value - to the monkey whose index was determined by the test
    // the monkey repeats all of that for its next items until it has no items
    // repeat all of that for each monkey

    let product_of_all_test_divisors: usize = monkeys.iter().map(|m| m.test_divisor).product();
    // println!(
    //     "product_of_all_test_divisors: {}",
    //     product_of_all_test_divisors
    // );

    let mut monkey_actions = vec![0; monkeys.len()];

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            // println!("\nmonkey {i}");
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.remove(0);
                // println!("inspects {item}");
                item = (*monkeys[i].operation)(item);

                item = item % product_of_all_test_divisors;

                // println!("worry {item}");
                let index_of_receiving_monkey = (*monkeys[i].test)(item);
                // println!("throw to {index_of_receiving_monkey}");
                monkeys[index_of_receiving_monkey].items.push(item);
                monkey_actions[i] += 1;
            }
        }
        // println!("round {_round} monkey_actions: {:?}", monkey_actions);
    }

    // let mi: Vec<Vec<usize>> = monkeys.iter().map(|m| m.items.clone()).collect();
    // println!("{:?}", mi);

    monkey_actions.sort();

    // println!("final monkey_actions: {:?}", monkey_actions);

    monkey_actions.iter().rev().take(2).product()
    // .fold(1, |acc, e| acc * e)
}
