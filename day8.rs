use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 8");

    let sample_input = aoc::read_input("input/day8-sample.txt");
    let real_input = aoc::read_input("input/day8.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    println!("sample 2 = {}", part2(&sample_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    let unique_segment_counts = vec!(2, 3, 4, 7);  // digits 1, 7, 4, 8
    let mut known_output_value_count: usize = 0;
    for (_, output_values) in parse_input(input) {
        let known_output_values: Vec<&str> = output_values
            .split_whitespace()
            .filter(|s| unique_segment_counts.contains(&s.len()))
            .collect();
        known_output_value_count += known_output_values.len();
    }
    known_output_value_count
}

fn part2(input: &[String]) -> usize {
    let unique_segment_counts = vec!(2, 3, 4, 7);  // digits 1, 7, 4, 8
    let mut output_value_nums: Vec<usize> = Vec::new();

    for (signals, output_values) in parse_input(input) {
        let mut digit_patterns: [&str; 10] = [""; 10];
        while digit_patterns.iter().any(|v| v.len() == 0) {
            for signal in signals.split_whitespace() {
                let signal_len = &signal.len();
                if unique_segment_counts.contains(signal_len) {
                    match signal_len {
                        2 => digit_patterns[1] = signal,
                        3 => digit_patterns[7] = signal,
                        4 => digit_patterns[4] = signal,
                        7 => digit_patterns[8] = signal,
                        &_ => ()
                    }
                } else {
                    if signal_len == &5 && digit_patterns[9].len() > 0 {   // 2, 3, 5
                        if match_chars(signal, digit_patterns[7]) {
                            digit_patterns[3] = signal;
                        } else if match_chars(digit_patterns[9], signal) {
                            digit_patterns[5] = signal;
                        } else {
                            digit_patterns[2] = signal;
                        }
                    } else {    // 0, 6, 9
                        if match_chars(signal, digit_patterns[4]) {
                            digit_patterns[9] = signal;
                        } else if match_chars(signal, digit_patterns[7]) {
                            digit_patterns[0] = signal;
                        } else {
                            digit_patterns[6] = signal;
                        }
                    }
                }
            }
        }

        let mut output_digits: [usize; 4] = [0; 4];
        for (d_i, output_value) in output_values.split_whitespace().enumerate() {
            let digit_matches: Vec<(usize, &&str)> = digit_patterns
                .iter()
                .enumerate()
                .filter(|(_, pattern)| pattern.len() == output_value.len() && match_chars(pattern, output_value))
                .collect();
            output_digits[d_i] = digit_matches[0].0;
        }
        let output_digit_str = output_digits
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("");

        output_value_nums.push(usize::from_str_radix(&output_digit_str, 10).unwrap());
    }

    output_value_nums.iter().sum()
}

fn parse_input(input: &[String]) -> Vec<(&str, &str)> {
    input
        .iter()
        .map(|s| s.split_once('|').unwrap())
        .collect()
}

fn match_chars(signal: &str, candidate: &str) -> bool {
    candidate.chars().all(|c| signal.contains(c))
}
