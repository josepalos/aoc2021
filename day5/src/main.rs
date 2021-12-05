use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::{Add,Sub};
use std::collections::HashMap;
use std::cmp;

fn load_input(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    return reader.lines().map(|l| l.unwrap());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn from_string(string: &str) -> Point {
        let mut iter = string.split(",");
        let x: i16 = iter.next().unwrap().trim().parse().unwrap();
        let y: i16 = iter.next().unwrap().trim().parse().unwrap();
        return Point {x: x, y: y};
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct VentLine {
    start: Point,
    end: Point,
}

impl VentLine {
    fn from_string(string: &str) -> VentLine {
        let mut split_iter = string.split("->");
        let start = Point::from_string(
            split_iter.next().unwrap());
        let end = Point::from_string(
            split_iter.next().unwrap());
        return VentLine { start: start, end: end };
    }

    fn direction(&self) -> Point {
        return self.end - self.start;
    }

    fn is_aligned_with_axis(&self) -> bool {
        let direction = self.direction();
        return direction.x == 0 || direction.y == 0;
    }

    fn list_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        let vec_dir = self.direction();
        match vec_dir {
            Point { x: 0, y: _ } => {
                // Vertical line
                let start = cmp::min(self.start.y, self.end.y);
                let end = cmp::max(self.start.y, self.end.y);
                for new_y in start..end+1 {
                    points.push(Point { x: self.start.x, y: new_y });
                }
            },
            Point { x: _, y: 0 } => {
                // Horizontal line
                let start = cmp::min(self.start.x, self.end.x);
                let end = cmp::max(self.start.x, self.end.x);
                for new_x in start..end+1 {
                    points.push(Point { x: new_x, y: self.start.y });
                }
            },
            Point { x, y } if x.abs() == y.abs() => {
                // 45º line
                let x_sign = x / x.abs();
                let y_sign = y / y.abs();
                for inc in 0..x.abs() + 1 {
                    let offset = Point{ x: inc * x_sign, y: inc * y_sign };

                    points.push(self.start + offset);
                }
            },
            _ => {},
        };

        return points;
    }
}

fn load_vent_lines(filename: &str) -> Vec<VentLine> {
    let mut vents: Vec<VentLine> = Vec::new();
    for line in load_input(filename) {
        vents.push(VentLine::from_string(&line));
    }
    return vents;
}

fn generate_overlap_map(vents: &Vec<&VentLine>) -> HashMap<Point, u16>{
    let mut overlap_count: HashMap<Point, u16> = HashMap::new();

    for vent in vents {
        //println!("{},{}", vent.direction().x, vent.direction().y);
        let points = vent.list_points();
        for point in points{
            //print!("{},{}", point.x, point.y);

            let current = overlap_count.get(&point);
            let new_count = match current {
                Some(count) => count + 1,
                None => 1,
            };
            //println!("...{}", new_count);
            overlap_count.insert(point, new_count);
        }
        //println!("");
    }

    return overlap_count;
}

fn print_overlap_map(overlap_map: &HashMap<Point, u16>) {
    let mut max_x = 0;
    let mut max_y = 0;

    for point in overlap_map.keys(){
        max_x = cmp::max(point.x, max_x);
        max_y = cmp::max(point.y, max_y);
    }

    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            let coord = Point { x: x, y: y };
            let count = match overlap_map.get(&coord) {
                Some(count) => count.to_string(),
                None => "-".to_string(),
            };
            print!(" {} ", count);
        }
        println!("");
    }
}

fn main() {
    let vents = load_vent_lines("input");

    let aligned_vents = vents.iter().filter(|&v| v.is_aligned_with_axis()).collect();
    let all_vents = vents.iter().collect();

    let overlap_aligned = generate_overlap_map(&aligned_vents);
    let overlap_count = generate_overlap_map(&all_vents);
    
    //println!("===============");
    //print_overlap_map(&overlap_count);

    let count_aligned = overlap_aligned.values().filter(|&x| x >= &2).count();
    let count_all = overlap_count.values().filter(|&x| x >= &2).count();

    println!("Part 1: {}", count_aligned);
    println!("Part 2: {}", count_all);
}
