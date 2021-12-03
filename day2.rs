use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run() {
    println!("AOC 2021 - Day 2");

    let sample_input = read_input("input/day2-sample.txt");
    let real_input = read_input("input/day2.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    println!("sample 2 = {}", part2(&sample_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> i32 {
    let mut x = 0;
    let mut depth = 0;

    for command in input {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let action: &str = &parts.get(0).unwrap();
        let distance: &i32 = &parts.get(1).unwrap().parse().unwrap();

        match action {
            "forward" => x += distance,
            "up" => depth -= distance,
            "down" => depth += distance,
            &_ => ()
        }
    }

    //println!("x={}, depth={}", x, depth);

    x * depth
}

fn part2(input: &[String]) -> i32 {
    let mut x = 0;
    let mut aim = 0;
    let mut depth = 0;

    for command in input {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let action: &str = &parts.get(0).unwrap();
        let distance: &i32 = &parts.get(1).unwrap().parse().unwrap();

        match action {
            "forward" => { x += distance; depth += aim * distance; },
            "up" => aim -= distance,
            "down" => aim += distance,
            &_ => ()
        }

        //println!("x={}, aim={}, depth={}", x, aim, depth);
    }

    //println!("x={}, aim={}, depth={}", x, aim, depth);

    x * depth
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
