use crate::aoc;
use std::collections::HashMap;

pub fn run() {
    println!("AOC 2021 - Day 14");

    let sample_input = aoc::read_input("input/day14-sample.txt");
    println!("sample 1 = {}", part1(&sample_input));
    //println!("sample 2 = {}", part2(&sample_input));

    let real_input = aoc::read_input("input/day14.txt");
    println!("part 1 = {}", part1(&real_input));
    //println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    apply_steps(input, 10)
}

fn part2(input: &[String]) -> usize {
    apply_steps(input, 40)
}

fn apply_steps(input: &[String], num_steps: usize) -> usize {
    let parsed_input = parse_input(input);

    let mut template = parsed_input.0;
    let insertion_rules = parsed_input.1;
    
    let mut inst = std::time::Instant::now();
    for step in 0..num_steps {
        log_step(step, &template, inst.elapsed());
        inst = std::time::Instant::now();
        template = apply_step(&template, &insertion_rules);
    }

    let char_counts = count_chars(&template, &insertion_rules);
    let min = char_counts.iter().min().unwrap();
    let max = char_counts.iter().max().unwrap();
    max - min
}

fn apply_step(template: &String, insertion_rules: &HashMap<String, String>) -> String {
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

fn count_chars(template: &String, insertion_rules: &HashMap<String, String>) -> Vec<usize> {
    let mut distinct_chars = insertion_rules.values().collect::<Vec<&String>>();
    distinct_chars.sort();
    distinct_chars.dedup();
    distinct_chars.iter().map(|c| template.matches(*c).count()).collect()
}

fn log_step(step: usize, template: &String, duration: std::time::Duration) {
    let cutoff = std::cmp::min(template.len(), 96);
    let ellipsis = if cutoff == template.len() { "   " } else { "..." };
    println!("After step {:>2}: {:<96}{} (len {}, {}ms)", step, &template[0..cutoff], ellipsis, template.len(), duration.as_millis());
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
