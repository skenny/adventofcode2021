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
    let mut fields = parse_input(input);

    let (reference_field, remaining_fields) = fields.split_first_mut().unwrap();
    let mut remaining_fields_queue: VecDeque<&mut Field> = VecDeque::from_iter(remaining_fields);

    let mut scanner_positions: Vec<Point> = Vec::new();

    while !remaining_fields_queue.is_empty() {
        let compare_field = remaining_fields_queue.pop_front().unwrap();
        let maybe_scanner_position = reference_field.overlaps(compare_field);
        if maybe_scanner_position.is_none() {
            remaining_fields_queue.push_back(compare_field);
        } else {
            let scanner_position = maybe_scanner_position.unwrap();
            println!("found {} at {:?}", compare_field.label, &scanner_position);
            scanner_positions.push(scanner_position);
        }
    }

    println!("part 1 = {}", reference_field.points.len());

    let mut max_distance = 0;
    for scanner_pos in scanner_positions.iter() {
        for scanner_pos_2 in scanner_positions.iter() {
            max_distance = std::cmp::max(max_distance, scanner_pos.find_offset(scanner_pos_2).manhattan_distance());
        }
    }

    println!("part 2 = {}", max_distance);
}

fn parse_input(input: &[String]) -> Vec<Field> {
    let mut fields: Vec<Field> = Vec::new();

    for line in input {
        if line.starts_with("---") {
            fields.push(Field { label: line.replace("---", "").trim().to_string(), points: Vec::new() })
        } else if line.is_empty() {
            continue;
        } else {
            let current_field: &mut Field = fields.last_mut().unwrap();
            current_field.points.push(Point::parse(line));
        }
    }
    
    fields
}

#[derive(Debug)]
struct Field {
    label: String,
    points: Vec<Point>
}

impl Field {
    fn rotations(&self) -> Vec<Field> {
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

    fn rotate(&self, matrix: [[i32; 3]; 3]) -> Field {
        Field { 
            label: self.label.clone(), 
            points: self.points.iter().map(|p| p.rotate(matrix)).collect() 
        }
    }

    fn overlaps(&mut self, other: &Field) -> Option<Point> {
        for ref_point in self.points.iter() {
            for other_rotation in other.rotations() {
                for other_ref_point in other_rotation.points.iter() {
                    let offset_point = ref_point.find_offset(other_ref_point);
                    let translated_other_points: Vec<Point> = other_rotation.points.iter().map(|op| op.translate(&offset_point)).collect();

                    let mut overlaps: Vec<Point> = Vec::new();
                    for other_point in translated_other_points.iter() {
                        if self.points.contains(&other_point) {
                            overlaps.push(other_point.clone());
                        }
                    }

                    if overlaps.len() >= 12 {
                        for other_point in translated_other_points.iter() {
                            if !self.points.contains(&other_point) {
                                self.points.push(other_point.clone());
                            }
                        }
                        return Some(offset_point);
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
