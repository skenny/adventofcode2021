use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run() {
    println!("AOC 2021 - Day 1");

    let sample_input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let real_input = read_input("input/day1.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    println!("sample 2 = {}", part2(&sample_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[i32]) -> i32 {
    let mut inc_counter = 0;
    for i in 0..input.len() {
        if i > 0 && input[i] > input[i - 1] {
            inc_counter += 1
        }
    }
    inc_counter
}

fn part2(input: &[i32]) -> i32 {
    let mut inc_counter = 0;
    for i in 3..input.len() {
        // this is gross, but whatever
        let prev_window = &input[i-3..i];
        let cur_window = &input[i-2..i+1];
        let prev_window_sum = sum(&prev_window);
        let cur_window_sum = sum(&cur_window);

        //println!("i={}, prev={:?}, prev_sum={}, cur={:?}, cur_sum={}", i, prev_window, prev_window_sum, cur_window, cur_window_sum);
        
        if cur_window_sum > prev_window_sum {
            inc_counter += 1
        }
    }
    inc_counter
}

fn sum(input: &[i32]) -> i32 {
    let mut sum = 0;
    for i in input {
        sum += i;
    }
    sum
}

fn read_input(filename: &str) -> Vec<i32> {
    let mut v = Vec::new();
    let file = File::open(filename).expect("cannot find file!");
    let reader = BufReader::new(file);

    // very inefficient, creates a new String for each line in the file
    // tutorial has better examples of how to read lines from a file more efficiently
    for line in reader.lines() {
        v.push(line.unwrap().parse::<i32>().unwrap());
    }

    v
}
