use std::{cmp::Ordering, fmt::Debug};

type Packet = Vec<PacketElement>;

#[derive(Debug, Clone)]
enum PacketElement {
    // let's assume they have to be positive numbers even though the problem says integers
    Number(usize),
    List(Packet),
}

impl PartialEq for PacketElement {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (PacketElement::Number(n), PacketElement::Number(other_n)) => n == other_n,
            (PacketElement::Number(n), PacketElement::List(other_list)) => {
                vec![PacketElement::Number(*n)] == *other_list
            }
            (PacketElement::List(list), PacketElement::Number(other_n)) => {
                *list == vec![PacketElement::Number(*other_n)]
            }
            (PacketElement::List(list), PacketElement::List(other_list)) => list == other_list,
        }
    }
}
impl Eq for PacketElement {}
impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self, other) {
            (PacketElement::Number(n), PacketElement::Number(other_n)) => Some(n.cmp(other_n)),
            (PacketElement::Number(n), PacketElement::List(other_list)) => {
                Some(vec![PacketElement::Number(*n)].cmp(other_list))
            }
            (PacketElement::List(list), PacketElement::Number(other_n)) => {
                Some(list.cmp(&vec![PacketElement::Number(*other_n)]))
            }
            (PacketElement::List(list), PacketElement::List(other_list)) => {
                Some(list.cmp(other_list))
            }
        }
    }
}
impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, other) {
            (PacketElement::Number(n), PacketElement::Number(other_n)) => n.cmp(other_n),
            (PacketElement::Number(n), PacketElement::List(other_list)) => {
                vec![PacketElement::Number(*n)].cmp(other_list)
            }
            (PacketElement::List(list), PacketElement::Number(other_n)) => {
                list.cmp(&vec![PacketElement::Number(*other_n)])
            }
            (PacketElement::List(list), PacketElement::List(other_list)) => list.cmp(other_list),
        }
    }
}

fn parse_input_as_packet_pairs(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|two_lines| {
            let mut lines = two_lines.lines();
            (
                parse_line(lines.next().unwrap()),
                parse_line(lines.next().unwrap()),
            )
        })
        .collect()
}

fn parse_line(line: &str) -> Packet {
    // println!("parsing line: {}", line);

    let mut chars = line.chars();

    // let's skip the opening [ because the recursive nature of the following function kind of depends on every opening [ already being "consumed" (immediately preceding the first index of the given slice)
    chars.next();

    recursively_parse_packet_line_with_iterator(&mut chars)
}

fn recursively_parse_packet_line_with_iterator(chars: &mut dyn Iterator<Item = char>) -> Packet {
    let mut packet = Packet::new();
    // assume a [ was just consumed from the iterator before this function was called
    // this should always terminate on its own by finding a closing ]
    // I'm going to track next as a dedicated variable because I want to be able to pick up where I left off whenever I hit a "dead end" in the parsing performed inside the while loop
    let mut next = chars.next();
    while let Some(c) = next {
        // println!("char {} is a digit: {}", c, c.is_digit(10));
        match c {
            ']' => return packet,
            '[' => {
                let list = recursively_parse_packet_line_with_iterator(chars);
                packet.push(PacketElement::List(list));
                // we need to keep iterating over chars
                next = chars.next();
            }
            // we need to keep iterating over chars
            ',' => next = chars.next(),
            _ if c.is_digit(10) => {
                let mut digit_chars = vec![c];
                // even if next doesn't turn out to be another digit, we'll have successfully updated it for the next iteration of the parent while loop
                next = chars.next();
                while let Some(c) = next {
                    // println!("checking if {} is also a digit", c);
                    if c.is_digit(10) {
                        digit_chars.push(c);
                        next = chars.next();
                    } else {
                        break;
                    }
                }
                let number_string: String = digit_chars.iter().collect();
                let number: usize = number_string.parse().unwrap();
                packet.push(PacketElement::Number(number))
            }
            _ => panic!("unrecognized char {}", c),
        }
    }
    panic!("an input line didn't have a ] to match a [");
}

pub fn part_one(input: &str) -> usize {
    let packet_pairs = parse_input_as_packet_pairs(input);

    // packet_pairs
    //     .iter()
    //     .for_each(|(first, second)| println!("{:?}\n{:?}\n", first, second));

    packet_pairs
        .iter()
        .enumerate()
        .map(|(index, (first, second))| if first < second { index + 1 } else { 0 })
        .sum()
}

fn parse_input_as_unpaired_packets(input: &str) -> Vec<Packet> {
    input
        .replace("\n\n", "\n")
        .lines()
        .map(|line| parse_line(line))
        .collect()
}

pub fn part_two(input: &str) -> usize {
    let mut packets = parse_input_as_unpaired_packets(input);

    let divider_packet_1 = vec![PacketElement::List(vec![PacketElement::Number(2)])];
    packets.push(divider_packet_1.clone());
    let divider_packet_2 = vec![PacketElement::List(vec![PacketElement::Number(6)])];
    packets.push(divider_packet_2.clone());

    packets.sort();

    // packets.iter().for_each(|packet| println!("{:?}", packet));

    packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| **packet == divider_packet_1 || **packet == divider_packet_2)
        .map(|(index, _)| index + 1)
        .product()
}
