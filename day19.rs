use crate::aoc;
use std::collections::VecDeque;

pub fn run() {
    println!("AOC 2021 - Day 19");

    let sample_1 = aoc::read_input("input/day19-sample-1.txt");
    println!("--- Sample ---");
    align_scanners(&sample_1);

    let real_input = aoc::read_input("input/day19.txt");
    println!("--- Input ---");
    align_scanners(&real_input);
}

fn align_scanners(input: &[String]) {
    let mut scanners = parse_input(input);

    let (reference_scanner, other_scanners) = scanners.split_first_mut().unwrap();

    let mut remaining_scanners: VecDeque<&mut Scanner> = VecDeque::from_iter(other_scanners);
    let mut scanner_positions: Vec<Point> = Vec::new();

    while !remaining_scanners.is_empty() {
        let compare_scanner = remaining_scanners.pop_front().unwrap();
        let maybe_overlap = reference_scanner.check_overlap(compare_scanner);

        if maybe_overlap.is_none() {
            remaining_scanners.push_back(compare_scanner);
        } else {
            let (scanner_position, scanner_orientation) = maybe_overlap.unwrap();
            println!("found {} at {:?}", compare_scanner.label, &scanner_position);

            let translated_scanner_beacons: Vec<Point> = scanner_orientation.beacons.iter().map(|op| op.translate(&scanner_position)).collect();
            for other_point in translated_scanner_beacons.iter() {
                if !reference_scanner.beacons.contains(&other_point) {
                    reference_scanner.beacons.push(other_point.clone());
                }
            }

            scanner_positions.push(scanner_position);
        }
    }

    println!("part 1 = {}", reference_scanner.beacons.len());

    let mut max_distance = 0;
    for scanner_pos in scanner_positions.iter() {
        for scanner_pos_2 in scanner_positions.iter() {
            max_distance = std::cmp::max(max_distance, scanner_pos.find_offset(scanner_pos_2).manhattan_distance());
        }
    }

    println!("part 2 = {}", max_distance);
}

fn parse_input(input: &[String]) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();

    for line in input {
        if line.starts_with("---") {
            scanners.push(Scanner { label: line.replace("---", "").trim().to_string(), beacons: Vec::new() })
        } else if line.is_empty() {
            continue;
        } else {
            let current_scanner: &mut Scanner = scanners.last_mut().unwrap();
            current_scanner.beacons.push(Point::parse(line));
        }
    }
    
    scanners
}

#[derive(Debug)]
struct Scanner {
    label: String,
    beacons: Vec<Point>
}

impl Scanner {
    fn rotations(&self) -> Vec<Scanner> {
        let mut rotations = Vec::with_capacity(24);

        rotations.push(self.rotate([[1, 0, 0], [0, 1, 0], [0, 0, 1]])); // identity
        rotations.push(self.rotate([[1, 0, 0], [0, 0, -1], [0, 1, 0]]));
        rotations.push(self.rotate([[1, 0, 0], [0, -1, 0], [0, 0, -1]]));
        rotations.push(self.rotate([[1, 0, 0], [0, 0, 1], [0, -1, 0]]));

        rotations.push(self.rotate([[0, -1, 0], [1, 0, 0], [0, 0, 1]]));
        rotations.push(self.rotate([[0, 0, 1], [1, 0, 0], [0, 1, 0]]));
        rotations.push(self.rotate([[0, 1, 0], [1, 0, 0], [0, 0, -1]]));
        rotations.push(self.rotate([[0, 0, -1], [1, 0, 0], [0, -1, 0]]));

        rotations.push(self.rotate([[-1, 0, 0], [0, -1, 0], [0, 0, 1]]));
        rotations.push(self.rotate([[-1, 0, 0], [0, 0, -1], [0, -1, 0]]));
        rotations.push(self.rotate([[-1, 0, 0], [0, 1, 0], [0, 0, -1]]));
        rotations.push(self.rotate([[-1, 0, 0], [0, 0, 1], [0, 1, 0]]));

        rotations.push(self.rotate([[0, 1, 0], [-1, 0, 0], [0, 0, 1]]));
        rotations.push(self.rotate([[0, 0, 1], [-1, 0, 0], [0, -1, 0]]));
        rotations.push(self.rotate([[0, -1, 0], [-1, 0, 0], [0, 0, -1]]));
        rotations.push(self.rotate([[0, 0, -1], [-1, 0, 0], [0, 1, 0]]));

        rotations.push(self.rotate([[0, 0, -1], [0, 1, 0], [1, 0, 0]]));
        rotations.push(self.rotate([[0, 1, 0], [0, 0, 1], [1, 0, 0]]));
        rotations.push(self.rotate([[0, 0, 1], [0, -1, 0], [1, 0, 0]]));
        rotations.push(self.rotate([[0, -1, 0], [0, 0, -1], [1, 0, 0]]));
        
        rotations.push(self.rotate([[0, 0, -1], [0, -1, 0], [-1, 0, 0]]));
        rotations.push(self.rotate([[0, -1, 0], [0, 0, 1], [-1, 0, 0]]));
        rotations.push(self.rotate([[0, 0, 1], [0, 1, 0], [-1, 0, 0]]));
        rotations.push(self.rotate([[0, 1, 0], [0, 0, -1], [-1, 0, 0]]));
        
        rotations
    }

    fn rotate(&self, matrix: [[i32; 3]; 3]) -> Scanner {
        Scanner { 
            label: self.label.clone(), 
            beacons: self.beacons.iter().map(|p| p.rotate(matrix)).collect() 
        }
    }

    fn check_overlap(&mut self, scanner: &Scanner) -> Option<(Point, Scanner)> {
        for scanner_orientation in scanner.rotations() {
            for scanner_ref_point in scanner_orientation.beacons.iter() {
                for self_ref_point in self.beacons.iter() {
                    let offset_point = self_ref_point.find_offset(scanner_ref_point);
                    let translated_scanner_points: Vec<Point> = scanner_orientation.beacons.iter().map(|op| op.translate(&offset_point)).collect();
                    let overlapping_beacons: Vec<&Point> = translated_scanner_points.iter().filter(|tp| self.beacons.contains(tp)).collect();
                    if overlapping_beacons.len() >= 12 {
                        return Some((offset_point, scanner_orientation));
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Point {
    fn parse(input: &str) -> Point {
        // 404,-588,-901
        let vals: Vec<&str> = input.split(",").collect();
        Point {
            x: i32::from_str_radix(vals[0], 10).unwrap(),
            y: i32::from_str_radix(vals[1], 10).unwrap(),
            z: i32::from_str_radix(vals[2], 10).unwrap()
        }
    }

    fn rotate(&self, matrix: [[i32; 3]; 3]) -> Point {
        Point {
            x: self.x * matrix[0][0] + self.y * matrix[0][1] + self.z * matrix[0][2],
            y: self.x * matrix[1][0] + self.y * matrix[1][1] + self.z * matrix[1][2], 
            z: self.x * matrix[2][0] + self.y * matrix[2][1] + self.z * matrix[2][2] 
        }
    }

    fn find_offset(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y, 
            z: self.z - other.z
        }
    }

    fn translate(&self, offset: &Point) -> Point {
        Point {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

}
