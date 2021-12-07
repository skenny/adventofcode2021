use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run() {
    println!("AOC 2021 - Day 6");

    let sample_input = read_input("input/day6-sample.txt");
    let real_input = read_input("input/day6.txt");

    println!("sample 1 = {}", simulate(&sample_input, 80));
    println!("part 1 = {}", simulate(&real_input, 80));

    println!("sample 2 = {}", simulate(&sample_input, 256));
    println!("part 2 = {}", simulate(&real_input, 256));
}

fn simulate(input: &[String], num_days: i32) -> i64 {
    let mut fish_counts: [i64; 9] = [0; 9];
    for fish in parse_input(input) {
        fish_counts[fish] += 1;
    }

    for day in 0..num_days {
        let mut new_fish = 0;
        for c in 0..8 {
            if c == 0 {
                new_fish = fish_counts[c];
            }
            fish_counts[c] = fish_counts[c+1];
        }
        fish_counts[6] += new_fish;
        fish_counts[8] = new_fish;
    }

    fish_counts.iter().sum()
}

fn parse_input(input: &[String]) -> Vec<usize> {
    let mut fishes: Vec<usize> = Vec::new();
    for line in input {
        line.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse().unwrap())
            .for_each(|f| fishes.push(f));
    }
    fishes
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
