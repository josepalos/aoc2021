use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn load_input(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    return reader.lines().map(|l| l.unwrap());
}

struct Population {
    population: HashMap<u8, u64>,
    current_time: u64,
}

fn sum_to_hashmap(hashmap: &mut HashMap<u8, u64>, key: u8, to_sum: u64) {
    let new_count = match hashmap.get(&key) {
        Some(x) => x + to_sum,
        None => to_sum,
    };
    hashmap.insert(key, new_count);
}

impl Population {
    fn new(initial_state: Vec<u8>) -> Population {
        let mut population: HashMap<u8, u64> = HashMap::new();
        for fish in initial_state {
            let new_count = match population.get(&fish) {
                Some(x) => x + 1,
                None => 1,
            };
            population.insert(fish, new_count);            
        }

        return Population { population: population, current_time: 0 };
    }

    fn print(&self) {
        print!("After {} days -> ({}): ", self.current_time, self.count());
        for (days, count) in self.population.iter() {
            print!("{}x{},", days, count);
        }
        println!("");
    }
    
    fn tick(&mut self) {
        self.current_time = self.current_time + 1;

        let mut new_population: HashMap<u8, u64> = HashMap::new();
        for (days, count) in self.population.iter() {
            if days == &0 {
                // Spawn
                sum_to_hashmap(&mut new_population, 8, *count);
                // Reset counter
                sum_to_hashmap(&mut new_population, 6, *count);
            } else {
                sum_to_hashmap(&mut new_population, days-1, *count);
            }
        }
        self.population = new_population;
    }

    fn count(&self) -> u64 {
        return self.population.values().sum();
    }
}

fn population_from_input(filename: &str) -> Population {
    let initial_state_string = load_input(filename).next().unwrap();
    let initial_state: Vec<u8> = initial_state_string
        .split(",")
        .map(|x| x.parse().expect("Expected positive numbers"))
        .collect();

    return Population::new(initial_state);
}

fn simulate(population: &mut Population, max_day: u32, verbose: bool) {
    if verbose {
        population.print();
    }

    for i in 0..max_day {
        println!("Analyzing day: {}", i);
        population.tick();
        if verbose {
            population.print();
        }
    }

    if verbose {
        println!("After {} days we have {} fish", max_day, population.count());
    }
}

fn example() -> u64 {
    let initial_state: Vec<u8> = vec![3,4,3,1,2];
    let mut population = Population::new(initial_state);

    simulate(&mut population, 18, true);
    
    return population.count();
}

fn part1() -> u64 {
    let max_day = 80;
    let mut population = population_from_input("input");
    simulate(&mut population, max_day, false);
    return population.count();
}

fn part2() -> u64 {
    let max_day = 256;
    let mut population = population_from_input("input");
    simulate(&mut population, max_day, false);
    return population.count();
}

fn main() {
    assert_eq!(example(), 26);
    assert_eq!(part1(), 390923);
    println!("Part 2: {}", part2());
}
