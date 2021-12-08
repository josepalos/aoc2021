use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

fn load_input(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    return reader.lines().map(|l| l.unwrap());
}

struct Mapping {
    mapping: HashMap<String, u8>,
    inverse_mapping: HashMap<u8, String>,
}

impl Mapping {
    fn new() -> Self {
        return Mapping {
            mapping: HashMap::new(),
            inverse_mapping: HashMap::new()
        };
    }

    fn get_key(&self, pattern: &String) -> String{
        let mut chars: Vec<char> = pattern.chars().collect();
        chars.sort();
        return chars.into_iter().collect();
    }

    fn add_mapping(&mut self, pattern: &String, value: u8) {
        let pattern = self.get_key(pattern);
        self.mapping.insert(pattern.to_string(), value);
        self.inverse_mapping.insert(value, pattern.to_string());
    }

    fn identify_pattern(&self, pattern: &String) -> Option<&u8> {
        let pattern = self.get_key(pattern);
        return self.mapping.get(&pattern);
    }

    fn get_pattern(&self, value: &u8) -> Option<HashSet<char>> {
        return match self.inverse_mapping.get(value) {
            Some(x) => Some(get_set_of_chars(&x)),
            None => None
        };
    }
}


fn get_set_of_chars(pattern: &String) -> HashSet<char> {
    let chars: Vec<char> = pattern.chars().collect();
    return HashSet::from_iter(chars.iter().cloned());
}

fn identify_numbers(signals: Vec<String>) -> Mapping {
    let mut mapping = Mapping::new();

    // Detect with unique length
    for signal in &signals {
        let signal = &signal.to_string();
        match signal.len() {
            2 => {
                mapping.add_mapping(signal, 1);
            },
            4 => {
                mapping.add_mapping(signal, 4);
            },
            3 => {
                mapping.add_mapping(signal, 7);
            },
            7 => {
                mapping.add_mapping(signal, 8);
            },
            _ => (),
        };
    }

    // Detect 9
    // The only one with 6 segments that uses all the signals used by 4
    let signals_at_4 = mapping.get_pattern(&4).unwrap();
    for signal in &signals {
        let signal = &signal.to_string();
        if signal.len() == 6 {
            let signal_set = get_set_of_chars(signal);
            let diff = signals_at_4.difference(&signal_set);

            if diff.count() == 0 {
                mapping.add_mapping(signal, 9);
                break;
            }
        }
    }

    // Detect 0
    // Discarding the 9, it is the only one with 6 segments that uses
    // all the signals used by 1
    let signals_at_1 = mapping.get_pattern(&1).unwrap();
    for signal in &signals {
        let signal = &signal.to_string();
        if signal.len() == 6 && mapping.identify_pattern(signal).is_none() {
            let signal_set = get_set_of_chars(signal);
            let diff = signals_at_1.difference(&signal_set);

            if diff.count() == 0 {
                mapping.add_mapping(signal, 0);
                break;
            }
        }
    }

    // Detect 6
    // Discarding 0 and 9, is the only one that uses 6 segments
    for signal in &signals {
        let signal = &signal.to_string();
        if signal.len() == 6 && mapping.identify_pattern(signal).is_none() {
            mapping.add_mapping(signal, 6);
            break;
        }
    }

    // Detect 3
    // Discarding the others found, signals for 1 are only a subset
    // of signals at 3
    for signal in &signals {
        let signal = &signal.to_string();
        if mapping.identify_pattern(signal).is_none() {
            let signal_set = get_set_of_chars(signal);
            if signals_at_1.is_subset(&signal_set) {
                mapping.add_mapping(signal, 3);
                break;
            }
        }
    }

    // Detect 5
    // union between 5 and 1 = 9
    let signals_at_9 = mapping.get_pattern(&9).unwrap();
    for signal in &signals {
        let signal = &signal.to_string();

        if mapping.identify_pattern(signal).is_none() {
            let signal_set = get_set_of_chars(signal);
            if signal_set.is_subset(&signals_at_9) {
                mapping.add_mapping(signal, 5);
                break;
            }
        }
    }

    // Detect 2
    // the last one
    for signal in &signals {
        let signal = &signal.to_string();
        if mapping.identify_pattern(signal).is_none() {
            mapping.add_mapping(signal, 2);
            break;
        }
    }


    return mapping;
}

fn parse_line(line: String) -> (Vec<String>, Vec<String>) {
    let mut iter = line.split("|");
    let signals = iter.next().unwrap()
        .split(" ")
        .map(|x| x.to_string())
        .filter(|x| x.len() > 0)
        .collect();
    let output = iter.next().unwrap()
        .split(" ")
        .map(|x| x.to_string())
        .filter(|x| x.len() > 0)
        .collect();

    return (signals, output);
}

fn main() {
    /*
    let signals = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
    let signals = signals.split(" ").map(|x| x.to_string()).collect();

    let mapping = identify_numbers(signals);
    for (pattern, value) in mapping.mapping.iter(){
        println!("{} {}", pattern, value);
    }
    return;
    */
    let mut part1_count = 0;
    let mut part2_sum: u64 = 0;
    let part1_to_count: HashSet<u8> = HashSet::from_iter(vec![1, 4, 7, 8]);

    let input = load_input("input");
    for (signals, output) in input.map(|x| parse_line(x)) {
        let mapping = identify_numbers(signals);
        let mut result_value: u64 = 0;
        for out_value in &output {
            let mapped = mapping
                .identify_pattern(out_value)
                .expect("Pattern not found :(");

            if part1_to_count.contains(mapped) {
                // Part1: count the apparitions of 1, 4, 7, and 8
                part1_count += 1;
            }

            result_value = result_value * 10 + *mapped as u64;
        }
        part2_sum += result_value;
    }

    println!("Part 1: {}", part1_count);
    println!("Part 2: {}", part2_sum);
}
