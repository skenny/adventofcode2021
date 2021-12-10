use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 10");

    let sample_input = aoc::read_input("input/day10-sample.txt");
    let real_input = aoc::read_input("input/day10.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    println!("sample 2 = {}", part2(&sample_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> u64 {
    let mut illegal_chars: Vec<char> = Vec::new();
    for line in input {
        let (is_corrupt, first_illegal_char) = is_corrupt(line);
        if is_corrupt {
            illegal_chars.push(first_illegal_char);
        }
    }
    illegal_chars.iter().map(|c| get_score(&c, true)).sum()
}

fn part2(input: &[String]) -> u64 {
    let mut completion_scores: Vec<u64> = Vec::new();
    let valid_lines: Vec<&String> = input.iter().filter(|line| !is_corrupt(line).0).collect();
    for line in valid_lines {
        let mut open_chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if is_open_char(c) {
                open_chars.push(c);
            } else {
                open_chars.pop();
            }
        }
        completion_scores.push(open_chars.iter().rev().map(|c| get_closing_char(c)).fold(0, |acc, c| (5 * acc) + get_score(&c, false)));
    }
    completion_scores.sort();
    completion_scores[completion_scores.len() / 2]
}

fn is_corrupt(line: &str) -> (bool, char) {
    let mut open_chars: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_open_char(c) {
            open_chars.push(c);
        } else {
            let last_open_char = open_chars.pop().or(Some(' ')).unwrap();
            if !matches(last_open_char, c) {
                return (true, c);
            }
        }
    }

    (false, ' ')
}

fn is_open_char(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn matches(open_char: char, close_char: char) -> bool {
    close_char == get_closing_char(&open_char)
}

fn get_closing_char(open_char: &char) -> char {
    match open_char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => ' '
    }
}

fn get_score(c: &char, error: bool) -> u64 {
    match c {
        ')' => if error { 3 } else { 1 },
        ']' => if error { 57 } else { 2 },
        '}' => if error { 1197 } else { 3 },
        '>' => if error { 25137 } else { 4 },
        _ => 0
    }
}
