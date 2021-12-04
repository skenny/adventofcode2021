use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run() {
    println!("AOC 2021 - Day 3");

    let sample_input = read_input("input/day3-sample.txt");
    let real_input = read_input("input/day3.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    //println!("sample 2 = {}", part2(&sample_input));
    //println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> i32 {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    let num_bits = input[0].len();
    for i in 0..num_bits {
        let bit_count = count_bits(i, &input);
        let zeroes = bit_count.0;
        let ones = bit_count.1;
        gamma.push(if zeroes > ones { '0' } else { '1' });
        epsilon.push(if zeroes < ones { '0' } else { '1' });
    }

    let gamma_int = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = i32::from_str_radix(&epsilon, 2).unwrap();

    gamma_int * epsilon_int
}

fn part2(input: &[String]) -> i32 {
    0
}

fn count_bits(bit_pos: usize, inputs: &[String]) -> (i32, i32) {
    let mut zeroes = 0;
    let mut ones = 0;

    for input in inputs {
        let chars: Vec<char> = input.chars().collect();
        if chars[bit_pos] == '0' {
            zeroes += 1;
        } else {
            ones += 1;
        }
    }

    (zeroes, ones)
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
