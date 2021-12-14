use crate::aoc;
use std::collections::HashMap;

pub fn run() {
    println!("AOC 2021 - Day 14");

    let sample_input = aoc::read_input("input/day14-sample.txt");
    println!("sample 1 = {}", part1(&sample_input));
    println!("sample 2 = {}", part2(&sample_input));

    let real_input = aoc::read_input("input/day14.txt");
    println!("part 1 = {}", part1(&real_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    let parsed_input = parse_input(input);

    let mut template = parsed_input.0;
    let insertion_rules = parsed_input.1;
    for i in 0..10 {
        if template.len() < 50 {
            println!("After step {}: {}", i, template);
        }
        template = step(&template, &insertion_rules);
    }

    let mut distinct_chars = template.chars().collect::<Vec<char>>();
    distinct_chars.dedup();
    let mut counts: Vec<usize> = Vec::new();
    for distinct_char in distinct_chars {
        counts.push(template.matches(distinct_char).count());
    }

    let min = counts.iter().min().unwrap();
    let max = counts.iter().max().unwrap();
    max - min
}

fn part2(input: &[String]) -> usize {
    0
}

fn step(template: &str, insertion_rules: &HashMap<String, String>) -> String {
    let mut new_template = String::new();
    for i in 1..template.len() {
        let pair = &template[i-1..=i];
        if i == 1 {
            new_template += &pair[0..1];
        }
        new_template += &insertion_rules[pair];
        new_template += &pair[1..2];
    }
    new_template
}

fn parse_input(input: &[String]) -> (String, HashMap<String, String>) {
    let template = input[0].to_string();
    let mut insertion_rules: HashMap<String, String> = HashMap::new();
    for line in &input[2..] {
        for (i, o) in line.split_once(" -> ") {
            insertion_rules.insert(i.to_string(), o.to_string());
        }    
    }
    (template.to_string(), insertion_rules)
}
