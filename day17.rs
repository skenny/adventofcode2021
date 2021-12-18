use crate::aoc;
use std::ops::Range;
use regex::Regex;

pub fn run() {
    println!("AOC 2021 - Day 17");

    let sample_input = aoc::read_input("input/day17-sample.txt");
    launch_probes(&sample_input);

    let real_input = aoc::read_input("input/day17.txt");
    launch_probes(&real_input);
}

fn launch_probes(input: &[String]) {
    let (x_target, y_target) = parse_input(input);
    let mut good_velocities: Vec<(i32, i32)> = Vec::new();
    let mut max_height = i32::MIN;

    for ivx in 0..500 {
        for ivy in -500..500 {
            let mut probe_x = 0;
            let mut probe_y = 0;
            let mut velocity_x = ivx;
            let mut velocity_y = ivy;
            let mut launch_max_height = i32::MIN;

            loop {
                probe_x += velocity_x;
                probe_y += velocity_y;
                velocity_x = std::cmp::max(0, velocity_x - 1);
                velocity_y -= 1;
                launch_max_height = std::cmp::max(launch_max_height, probe_y);
        
                if probe_x >= x_target.start && probe_x <= x_target.end && probe_y >= y_target.start && probe_y <= y_target.end {
                    good_velocities.push((ivx, ivy));
                    max_height = std::cmp::max(max_height, launch_max_height);
                    break;
                }

                if probe_x > x_target.end || probe_y < y_target.start {
                    break;
                }
            }
        }
    }

    println!("part 1 = {}", max_height);
    println!("part 2 = {}", good_velocities.len());
}

fn parse_input(input: &[String]) -> (Range<i32>, Range<i32>) {
    let input_line = &input[0];
    let pattern: Regex = Regex::new("target area: x=(\\d*)..(\\d*), y=(-?\\d*)..(-?\\d*)").unwrap();
    assert!(pattern.is_match(input_line));

    let captures = pattern.captures(input_line).unwrap();
    let x_min = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let x_max = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let y_min = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let y_max = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

    (x_min..x_max, y_min..y_max)
}
