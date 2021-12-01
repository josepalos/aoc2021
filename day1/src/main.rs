use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> u16{
    let mut last_depth:Option<u16> = None;
    let mut increases = 0;

    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let current_depth: u16 = line.trim().parse().expect("Input was not a number");

        if last_depth.is_none(){
            println!("{} (N/A - no previous measurement)", current_depth)
        } else {
            match current_depth.cmp(&last_depth.unwrap()) {
                Ordering::Less => println!("{} (decreased)", current_depth),
                Ordering::Greater => {
                    println!("{} (increased)", current_depth);
                    increases = increases + 1;
                },
                Ordering::Equal => println!("{} (equal)", current_depth),
            }
        }
        last_depth = Some(current_depth);
    }
    println!("Increased a total of {} times", increases);
    return increases;
}

fn sum_n_last_values(history: &Vec<u16>, nvalues: &u16) -> u16{
    let mut sum = 0;
    for n in 1..(*nvalues + 1) {
        let idx = usize::from(n);
        sum += history[history.len() - idx];
    }
    return sum;
}

fn part2() -> u16{
    let window_size: u16 = 3;
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut history: Vec<u16> = Vec::new();
    let mut increases = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let value: u16 = line.trim().parse().expect("Input was not a number");

        if history.len() < window_size.into() {
            history.push(value);
            continue;
        }
        let last_window = sum_n_last_values(&history, &window_size);
        history.push(value);
        let current_window = sum_n_last_values(&history, &window_size);

        match current_window.cmp(&last_window) {
            Ordering::Less => println!("{} (decreased)", value),
            Ordering::Greater => {
                println!("{} (increased)", value);
                increases = increases + 1;
            },
            Ordering::Equal => println!("{} (equal)", value),
        }
    }

    println!("Increases {} times", increases);
    return increases;
}

fn main() {
    let increases1 = part1();
    let increases2 = part2();
    
    println!("Solution for part1 is {}", increases1);
    println!("Solution for part2 is {}", increases2);
}
