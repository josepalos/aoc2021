use std::fs::File;
use std::io::{BufReader, BufRead};

fn load_input(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    return reader.lines().map(|l| l.unwrap());
}

struct BingoBoard {
    width: u16,
    height: u16,
    values: Vec<u16>,
    marked: Vec<bool>,
}
impl Clone for BingoBoard {
    fn clone(&self) -> BingoBoard {
        return BingoBoard {
            width: self.width,
            height: self.height,
            values: self.values.to_vec(),
            marked: self.marked.to_vec()
        }
    }
}

impl BingoBoard {
    fn from_values(values: Vec<u16>, n_rows: u16) -> BingoBoard{
        let n_cols = values.len() as u16 / n_rows;
        let size = values.len();

        return BingoBoard {
            width: n_cols,
            height: n_rows,
            values: values,
            marked: vec![false; size],
        };
    }

    fn mark_value(&mut self, value: u16) {
        for (i, board_value) in self.values.iter().enumerate() {
            if board_value == &value {
                self.marked[i] = true;
                break;
            }
        }
    }
    
    fn is_winner(&self) -> bool{
        // Assume they are complete until proven false
        let mut columns_complete = vec![true; self.width.into()];
        let mut rows_complete = vec![true; self.height.into()];

        for x in 0..self.width {
            for y in 0..self.height {
                if !self.marked[self.idx(x, y)] {
                    columns_complete[x as usize] = false;
                    rows_complete[y as usize] = false;
                }
            }
        }
        
        for column in columns_complete {
            if column {
                return true;
            }
        }
        for row in rows_complete {
            if row {
                return true;
            }
        }
        return false;
    }

    fn score(&self, winner_value: u64) -> u64 {
        let mut sum: u64 = 0;
        for (i, is_marked) in self.marked.iter().enumerate() {
            if !is_marked{
                sum = sum + self.values[i] as u64;
            }
        }
        return sum * winner_value;
    }

    fn idx(&self, x: u16, y: u16) -> usize{
        return (y + self.width * x).into();
    }

    fn print_board(&self) {
        for x in 0..self.height {
            for y in 0..self.width {
                let idx = self.idx(x, y);
                let prefix;
                match self.marked[idx] {
                    true => prefix = "+",
                    false => prefix = "-",
                }
                print!("{}{} ", prefix, self.values[idx]);
            }
            println!("");
        }
    }
}


fn load_boards<C>(input: C) -> Vec<BingoBoard>
        where C: IntoIterator<Item = String> {
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut current_values: Vec<u16> = Vec::new();
    let mut rows_count = 0;

    for line in input {
        if line == "" {
            if rows_count == 0{
                continue
            }
            let board = BingoBoard::from_values(current_values, rows_count);
            boards.push(board);
            current_values = Vec::new();
            rows_count = 0;
        } else {
            rows_count = rows_count + 1;
            let row_values = line.split(" ").collect::<Vec<&str>>();
            for value in row_values {
                if value == "" {
                    continue
                }
                current_values.push(value
                    .parse()
                    .expect(&format!("Value ..{}.. in line ..{}.. should be a number", value, line))
                );
            }
        }
    }

    return boards;
}


fn part1() -> Option<u64> {
    let mut input = load_input("input");

    let winners: Vec<u16> = input.next().unwrap().split(",").map(|l| l.parse().unwrap()).collect();

    let mut boards = load_boards(input);

    for winner in winners {
        println!("Marking value {}", winner);
        for i in 0..boards.len() {
            boards[i].mark_value(winner);

            if boards[i].is_winner(){
                println!("First board is");
                boards[i].print_board();
                let score = boards[i].score(winner as u64);
                println!("Score was {}", score);
                return Some(score);
            }
        }
    }

    return None;
}


fn part2() -> Option<u64> {
    let mut input = load_input("input");

    let winners: Vec<u16> = input.next().unwrap().split(",").map(|l| l.parse().unwrap()).collect();

    let mut boards = load_boards(input);
    let mut last_board: Option<BingoBoard> = None;
    let mut last_winner = 0;

    for winner in winners {
        boards.retain(|b| !b.is_winner());
        if boards.len() == 0 {
            break;
        }

        println!("Marking value {}", winner);
        for i in 0..boards.len() {
            boards[i].mark_value(winner);
        }
        last_board = Some(boards[0].clone());
        last_winner = winner;
    }

    println!("Last board was");
    let last_board = last_board.unwrap();
    last_board.print_board();

    return Some(last_board.score(last_winner as u64));
}

fn main() {
    let part1_result = part1().unwrap();
    let part2_result = part2().unwrap();

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}
