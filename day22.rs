use crate::aoc;
use std::collections::HashMap;

pub fn run() {
    println!("AOC 2021 - Day 22");

    let sample_input_1 = aoc::read_input("input/day22-sample-1.txt");
    let start = std::time::Instant::now();
    println!("sample 1 1 = {} ({}ms)", part1(&sample_input_1), start.elapsed().as_millis());
    println!("sample 1 2 = {}", part2(&sample_input_1));

    let sample_input_2 = aoc::read_input("input/day22-sample-2.txt");
    let start = std::time::Instant::now();
    println!("sample 2 1 = {} ({}ms)", part1(&sample_input_2), start.elapsed().as_millis());
    println!("sample 2 2 = {}", part2(&sample_input_2));

    let real_input = aoc::read_input("input/day22.txt");
    let start = std::time::Instant::now();
    println!("part 1 = {} ({}ms)", part1(&real_input), start.elapsed().as_millis());
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    let mut on_cubes: HashMap<(i32, i32, i32), bool> = HashMap::new();
    for line in input {
        let command = parse_command(line);

        // limit all ranges to -50..50
        let x_range = (std::cmp::max(command.x_range.0, -50), std::cmp::min(command.x_range.1, 50));
        let y_range = (std::cmp::max(command.y_range.0, -50), std::cmp::min(command.y_range.1, 50));
        let z_range = (std::cmp::max(command.z_range.0, -50), std::cmp::min(command.z_range.1, 50));
        
        for x in x_range.0..=x_range.1 {
            for y in y_range.0..=y_range.1 {
                for z in z_range.0..=z_range.1 {
                    let cube = (x, y, z);
                    if command.on {
                        on_cubes.insert(cube, true);
                    } else {
                        on_cubes.remove(&cube);
                    }
                }
            }
        }
    }
    on_cubes.len()
}

fn part2(input: &[String]) -> i32 {
    0
}

fn parse_command(input: &str) -> Command {
    // on x=10..12,y=10..12,z=10..12
    // off x=9..11,y=9..11,z=9..11

    // initialize now so the compiler doesn't complain about possibly uninitialized vars later when we create a Command
    let mut x_range: (i32, i32) = (0, 0);
    let mut y_range: (i32, i32) = (0, 0);
    let mut z_range: (i32, i32) = (0, 0);
    
    let (action, ranges) = input.split_once(" ").unwrap();

    for range in ranges.split(",") {
        let (axis, range_values) = range.split_once("=").unwrap();
        let (start, end) = range_values.split_once("..").unwrap();
        match axis {
            "x" => x_range = (start.parse().unwrap(), end.parse().unwrap()),
            "y" => y_range = (start.parse().unwrap(), end.parse().unwrap()),
            "z" => z_range = (start.parse().unwrap(), end.parse().unwrap()),
            &_ => {}
        } 
    }

    Command {
        on: if action == "on" { true } else { false },
        x_range: x_range,
        y_range: y_range,
        z_range: z_range
    }
}

struct Command {
    on: bool,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32)
}