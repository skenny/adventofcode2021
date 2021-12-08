use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run() {
    println!("AOC 2021 - Day 8");

    let sample_input = read_input("input/day8-sample.txt");
    let real_input = read_input("input/day8.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    //println!("sample 2 = {}", part2(&sample_input));
    //println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    let mut unique_output_value_count: usize = 0;
    for line in input {
        let (_, output_values) = line.split_once('|').unwrap();
        let unique_output_values: Vec<&str> = output_values
            .split_whitespace()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .collect();
        unique_output_value_count += unique_output_values.len();
    }
    unique_output_value_count
}

fn part2(input: &[String]) -> i64 {
    0
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
