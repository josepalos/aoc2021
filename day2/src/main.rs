use std::fs::File;
use std::io::{BufRead, BufReader};


struct Position {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Position[x={},y={}]", self.x, self.y)
    }
}


fn part1() -> i32{
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut current_position = Position { x: 0, y: 0 };


    for line in  reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let ammount = split.next().unwrap()
            .parse::<u16>()
            .expect("Ammount expected to be an integer");

        match (direction, ammount) {
            ("forward", dx) => {
                current_position.x = current_position.x + dx as i32;
            },
            ("up", dy) => {
                // Note that up is down, as y is the depth :D
                current_position.y = current_position.y - dy as i32;
            },
            ("down", dy) => {
                current_position.y = current_position.y + dy as i32;
            }
            _ => {
                println!("Invalid command {}", direction);
            }

        }
        //println!("{} {}", direction, ammount);
    }
    
    println!("{}", current_position);
    return current_position.x * current_position.y;
}

fn part2() -> i32{
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut current_position = Position { x: 0, y: 0 };
    let mut aim: i32 = 0;

    for line in  reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let ammount = split.next().unwrap()
            .parse::<u16>()
            .expect("Ammount expected to be an integer");

        match (direction, ammount) {
            ("forward", dx) => {
                current_position.x = current_position.x + dx as i32;
                current_position.y = current_position.y + dx as i32 * aim; 
            },
            ("up", daim) => {
                // Note that up is down, as y is the depth :D
                aim -= daim as i32;
            },
            ("down", daim) => {
                aim += daim as i32;
            }
            _ => {
                println!("Invalid command {}", direction);
            }
        }
        println!("{}", line);
        println!("{}, {}", current_position, aim);
    }
    
    println!("{}", current_position);
    return current_position.x * current_position.y;
}

fn main() {
    let part1_result = part1();
    let part2_result = part2();
    println!("Part 1 result is {}", part1_result);
    println!("Part 2 result is {}", part2_result);
}
