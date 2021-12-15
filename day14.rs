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
    apply_steps(input, 10)
}

fn part2(input: &[String]) -> usize {
    apply_steps(input, 40)
}

fn apply_steps(input: &[String], num_steps: usize) -> usize {
    let (template, insertion_rules) = parse_input(input);

    let mut pair_counts: HashMap<String, usize> = HashMap::new();
    for pair in chunk_input(&template) {
        *pair_counts.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..num_steps {
        pair_counts = apply_step(pair_counts, &insertion_rules);
    }

    // count characters
    let mut char_counts = count_chars(&pair_counts, &insertion_rules);

    // add one for the last char in the template
    let last_template_char = template.chars().last().unwrap().to_string();
    char_counts.iter_mut().for_each(|(ch, count)| if *ch == last_template_char { *count += 1; });

    let counts: Vec<usize> = char_counts.iter().map(|(_, count)| *count).collect();
    let min = counts.iter().min().unwrap();
    let max = counts.iter().max().unwrap();
    max - min
}

fn apply_step(pair_counts: HashMap<String, usize>, insertion_rules: &HashMap<String, String>) -> HashMap<String, usize> {
    let mut new_pair_counts: HashMap<String, usize> = HashMap::new();
    for (pair, count) in pair_counts.iter() {
        let insertion_char = &insertion_rules[pair];
        let left_pair = pair[0..1].to_owned() + insertion_char;
        let right_pair = insertion_char.to_owned() + &pair[1..2];
        *new_pair_counts.entry(left_pair).or_insert(0) += count;
        *new_pair_counts.entry(right_pair).or_insert(0) += count;
    }
    new_pair_counts
}

fn chunk_input(template: &String) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    for i in 1..template.len() {
        chunks.push(template[i-1..=i].to_string());
    }
    chunks
}

fn count_chars(pair_counts: &HashMap<String, usize>, insertion_rules: &HashMap<String, String>) -> Vec<(String, usize)> {
    let mut distinct_chars = insertion_rules.values().collect::<Vec<&String>>();
    distinct_chars.sort();
    distinct_chars.dedup();
    distinct_chars.iter().map(|ch| (ch.to_string(), pair_counts.iter().filter(|(pair, _)| pair.starts_with(*ch)).fold(0, |acc, (_, count)| acc + count))).collect()
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
