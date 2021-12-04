use std::fs::File;
use std::io::{BufRead, BufReader};


fn bit_count_to_integer(bit_count: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for x in bit_count {
        result = result << 1;
        if x > &0 {
            result = result | 1;
        }
    }
    return result;
}

fn calculate_bit_count<C: IntoIterator<Item = String>>(values: C) -> Vec<i32> {
    let mut bit_count: Vec<i32> = Vec::new();

    for value in values {
        let length = value.chars().count();
        while bit_count.len() < length {
            bit_count.push(0);
        }
        for (i, c) in value.chars().enumerate() {
            if c == '1' {
                bit_count[i] = bit_count[i] + 1;
            } else {
                bit_count[i] = bit_count[i] - 1;
            }
        }
    }

    return bit_count;
}

fn complementary(value: i32) -> i32{
    let mask: u32 = !0 >> value.leading_zeros();
    return value ^ mask as i32;
}

fn part1() -> i32 {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let bit_count = calculate_bit_count(
        reader.lines().map(|l| l.unwrap())
    );

    let gamma = bit_count_to_integer(&bit_count);
    let epsilon = complementary(gamma);

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    return gamma * epsilon;
}


enum Gas {
    Oxygen,
    CO2
}

fn find_gas<'a, C>(gas: Gas, values: C, bit_to_consider: usize) -> i64
        where C: IntoIterator<Item = &'a String> {
    let mut to_scrub_if_1: Vec<&String> = Vec::new();
    let mut to_scrub_if_0: Vec<&String> = Vec::new();

    for value in values {
        let lastbit = value.chars().nth(bit_to_consider).unwrap();
        if lastbit == '1' {
            to_scrub_if_1.push(value);
        } else {
            to_scrub_if_0.push(value);
        }
    }
    
    let to_scrub: Vec<&String>;
    match gas {
        Gas::Oxygen => {
            if to_scrub_if_0.len() > to_scrub_if_1.len() {
                to_scrub = to_scrub_if_0;
            } else {
                to_scrub = to_scrub_if_1;
            }
        },
        Gas::CO2 => {
            if to_scrub_if_0.len() <= to_scrub_if_1.len() {
                to_scrub = to_scrub_if_0;
            } else {
                to_scrub = to_scrub_if_1;
            }
        }
    }
    if to_scrub.len() == 1{
        let mut result = 0;
        for character in to_scrub[0].chars() {
            let current_digit: i64 = character.to_string().parse().unwrap();
            result = (result << 1) + current_digit;
        }
        return result;
    }
    return find_gas(gas, to_scrub.into_iter(), bit_to_consider + 1);
}

fn part2() -> i64 {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let values: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let oxygen = find_gas(
        Gas::Oxygen,
        &values,
        0
    );
    let co2 = find_gas(
        Gas::CO2,
        &values,
        0
    );
    println!("O: {}", oxygen);
    println!("CO2: {}", co2);

    return oxygen * co2;
}

fn main() {
    let p1_result = part1();
    let p2_result = part2();

    println!("Part 1: {}", p1_result);
    println!("Part 2: {}", p2_result);
}
