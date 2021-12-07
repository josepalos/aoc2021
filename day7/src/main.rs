use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::max;

fn load_input(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    return reader.lines().map(|l| l.unwrap());
}

fn median(numbers: &Vec<i64>) -> i64 {
	let mut vals = numbers.clone();
	vals.sort();
	let mid = (vals.len() / 2) as usize;
	return vals[mid];
}

fn part1() -> i64 {
	let values: Vec<i64> = load_input("input")
		.next().unwrap() // first line
		.split(",")
		.map(|x| x.parse().expect("Input should be only numerical values"))
		.collect();
	
	let best_point = median(&values);
	let mut fuel = 0;
	for value in values {
		fuel = fuel + (value - best_point).abs();
	}
	println!("{}", fuel);
	return fuel;
}

fn part2_cost_function(optimal: i64, positions: &Vec<i64>) -> i64 {
	let mut cost = 0;

	for position in positions{
		let diff = (position - optimal).abs();
		let crab_cost = (diff * (diff + 1)) / 2;
		cost = cost + crab_cost;
	}

	return cost;
}

fn part2() -> i64 {
	let values: Vec<i64> = load_input("input")
		.next().unwrap() // first line
		.split(",")
		.map(|x| x.parse().expect("Input should be only numerical values"))
		.collect();

	let mut max_value = 0;
	for value in &values {
		max_value = max(*value, max_value);
	}

	// Brute force the costs
	let mut best_cost = i64::pow(2, 60);

	for position in 0..max_value+1 {
		let cost = part2_cost_function(position, &values);
		if best_cost > cost {
			best_cost = cost;
		}
	}

	println!("Best cost is {}", best_cost);
	return best_cost;
}

fn main() {
	part1();
	part2();
}
