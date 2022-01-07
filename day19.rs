use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 19");

    let sample_1 = aoc::read_input("input/day19-sample-1.txt");
    println!("sample 1 1 = {}", part1(&sample_1));
    // println!("sample 1 2 = {}", part2(&sample_1));

    // let sample_2 = aoc::read_input("input/day19-sample-2.txt");
    // println!("sample 2 1 = {}", part1(&sample_2));
    // println!("sample 2 2 = {}", part2(&sample_2));

    // let real_input = aoc::read_input("input/day19.txt");
    // println!("part 1 = {}", part1(&real_input));
    // println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> i32 {
    let fields = parse_input(input);
    //fields.iter().for_each(|f| println!("{:?}", &f));
    0
}

fn part2(input: &[String]) -> i32 {
    0
}

fn parse_input(input: &[String]) -> Vec<Field> {
    let mut fields: Vec<Field> = Vec::new();

    for line in input {
        if line.starts_with("---") {
            fields.push(Field { label: line.clone(), points: Vec::new() })
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
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn rotate(&self, matrix: [[i32; 3]; 3]) -> Point {
        Point {
            x: self.x * matrix[0][0] + self.y * matrix[0][1] + self.z * matrix[0][2],
            y: self.x * matrix[1][0] + self.y * matrix[1][1] + self.z * matrix[1][2], 
            z: self.x * matrix[2][0] + self.y * matrix[2][1] + self.z * matrix[2][2] 
        }
    }

    fn parse(input: &str) -> Point {
        // 404,-588,-901
        let vals: Vec<&str> = input.split(",").collect();
        Point {
            x: i32::from_str_radix(vals[0], 10).unwrap(),
            y: i32::from_str_radix(vals[1], 10).unwrap(),
            z: i32::from_str_radix(vals[2], 10).unwrap()
        }
    }
}
