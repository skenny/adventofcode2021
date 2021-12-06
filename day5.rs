use std::collections::HashMap;
use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct LineSegment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl LineSegment {
    fn new(p1: (i32, i32), p2: (i32, i32)) -> LineSegment {
        LineSegment {
            x1: p1.0, y1: p1.1,
            x2: p2.0, y2: p2.1
        }
    }

    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn points(&self) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = Vec::new();

        if self.is_horizontal() {
            for x in cmp::min(self.x1, self.x2)..=cmp::max(self.x1, self.x2) {
                points.push((x, self.y1));
            }
        } else if self.is_vertical() {
            for y in cmp::min(self.y1, self.y2)..=cmp::max(self.y1, self.y2) {
                points.push((self.x1, y));
            }
        } else if self.is_diagonal() {
            let steps = i32::abs(self.x2 - self.x1);
            let x_mod = if self.x1 < self.x2 { 1 } else { -1 };
            let y_mod = if self.y1 < self.y2 { 1 } else { -1 };

            for i in 0..=steps {
                points.push((self.x1 + i * x_mod, self.y1 + i * y_mod));
            }
        }

        points
    }
}

impl std::fmt::Debug for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)
    }
}

pub fn run() {
    println!("AOC 2021 - Day 5");

    let sample_input = parse_input(&read_input("input/day5-sample.txt"));
    let real_input = parse_input(&read_input("input/day5.txt"));

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    println!("sample 2 = {}", part2(&sample_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(line_segments: &Vec<LineSegment>) -> usize {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    for line_segment in line_segments {
        if !line_segment.is_diagonal() {
            for point in line_segment.points() {
                let point_count = grid.entry(point).or_insert(0);
                *point_count += 1;
            }
        }
    }

    grid.into_values().filter(|c| *c > 1).count()
}

fn part2(line_segments: &Vec<LineSegment>) -> usize {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    for line_segment in line_segments {
        for point in line_segment.points() {
            let point_count = grid.entry(point).or_insert(0);
            *point_count += 1;
        }
    }

    grid.into_values().filter(|c| *c > 1).count()
}

fn parse_input(input: &[String]) -> Vec<LineSegment> {
    let mut line_segments: Vec<LineSegment> = Vec::new();
    for line in input {
        let points: Vec<(i32, i32)> = line.split(" -> ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| parse_point(&s))
            .collect();
        line_segments.push(LineSegment::new(points[0], points[1]));
    }
    line_segments
}

fn parse_point(str: &str) -> (i32, i32) {
    let coords: Vec<&str> = str.split(",").collect();
    (coords[0].parse().unwrap(), coords[1].parse().unwrap())
}

fn read_input(filename: &str) -> Vec<String> {
    let mut v = Vec::new();
    let file = File::open(filename).expect("cannot find file!");
    let reader = BufReader::new(file);

    // very inefficient, creates a new String for each line in the file
    // tutorial has better examples of how to read lines from a file more efficiently
    for line in reader.lines() {
        v.push(line.unwrap());
    }

    v
}
