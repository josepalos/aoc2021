use std::fs::File;
use std::io::{BufReader, BufRead};

fn load_input(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    return reader.lines().map(|l| l.unwrap());
}

struct Population {
    population: Vec<u8>,
    current_time: u64,
}

impl Population {
    fn new(initial_state: Vec<u8>) -> Population {
        let population = initial_state;
        return Population { population: population, current_time: 0 };
    }

    fn print(&self) {
        print!("After {} days -> ({}): ", self.current_time, self.population.len());
        for fish in self.population.iter() {
            print!("{},", fish);
        }
        println!("");
    }

    fn tick(&mut self) {
        self.current_time = self.current_time + 1;

        for i in 0..self.population.len() {
            if self.population[i] > 0 {
                self.population[i] = self.population[i] - 1;
            } else {
                self.spawn(i);
            }
        }
    }

    fn spawn(&mut self, from: usize) {
        self.population[from] = 6;
        self.population.push(8);
    }

    fn count(&self) -> usize{
        return self.population.len();
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
        println!("After {} days we have {} fish", max_day, population.population.len());
    }
}

fn part1() -> usize {
    let max_day = 80;

    let mut population = population_from_input("input");

    simulate(&mut population, max_day, true);
    
    return population.count();
}

fn main() {
    //let initial_state: Vec<u8> = vec![3,4,3,1,2];
    //let mut population = Population::new(initial_state);
    //
    assert_eq!(part1(), 390923);
}
