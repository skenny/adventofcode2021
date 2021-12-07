use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run() {
    println!("AOC 2021 - Day 7");

    let sample_input = read_input("input/day7-sample.txt");
    let real_input = read_input("input/day7.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    println!("sample 2 = {}", part2(&sample_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> i64 {
    let positions: Vec<i64> = parse_input(input);
    let max_pos: &i64 = positions.iter().max().unwrap();

    let mut position_moves: Vec<i64> = vec!(0; (*max_pos + 1) as usize);

    for pos_candidate in 0..=*max_pos {
        for pos in &positions {
            let diff = i64::abs(pos - pos_candidate);
            let cur_total = position_moves.get_mut(pos_candidate as usize).unwrap();
            *cur_total += diff;
        }
    }

    *position_moves.iter().min().unwrap() as i64
}

fn part2(input: &[String]) -> i64 {
    let positions: Vec<i64> = parse_input(input);
    let max_pos: &i64 = positions.iter().max().unwrap();

    let mut position_moves: Vec<i64> = vec!(0; (*max_pos + 1) as usize);

    for pos_candidate in 0..=*max_pos {
        for pos in &positions {
            let mut diff = i64::abs(pos - pos_candidate);
            diff = (diff * (diff + 1)) / 2;
            let cur_total = position_moves.get_mut(pos_candidate as usize).unwrap();
            *cur_total += diff;
        }
    }

    *position_moves.iter().min().unwrap() as i64
}

fn parse_input(input: &[String]) -> Vec<i64> {
    let mut positions: Vec<i64> = Vec::new();
    for line in input {
        line.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse().unwrap())
            .for_each(|i| positions.push(i));
    }
    positions
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