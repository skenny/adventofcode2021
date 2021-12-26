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
    let commands: Vec<Command> = input.iter().map(|line| parse_command(line)).collect();
    turn_on_cubes(&commands, Some((-50, 50)))
}

fn part2(input: &[String]) -> usize {
    let commands: Vec<Command> = input.iter().map(|line| parse_command(line)).collect();
    let reduced_commands = reduce_commands(&commands);
    turn_on_cubes(&reduced_commands, None)
}

fn turn_on_cubes(commands: &Vec<Command>, maybe_limit: Option<(i32, i32)>) -> usize {
    let mut on_cubes: HashMap<(i32, i32, i32), bool> = HashMap::new();
    for command in commands {
        let mut x_range = command.x_range;
        let mut y_range = command.y_range;
        let mut z_range = command.z_range;
        if maybe_limit.is_some() {
            let limit = maybe_limit.unwrap();
            x_range = (std::cmp::max(x_range.0, limit.0), std::cmp::min(x_range.1, limit.1));
            y_range = (std::cmp::max(y_range.0, limit.0), std::cmp::min(y_range.1, limit.1));
            z_range = (std::cmp::max(z_range.0, limit.0), std::cmp::min(z_range.1, limit.1));
        }
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

fn reduce_commands(commands: &Vec<Command>) -> Vec<Command> {
    let new_commands: Vec<Command> = Vec::new();
    for command in commands {
        for new_command in commands {
            if new_command.x_range.0 <= command.x_range.1 && new_command.x_range.1 >= command.x_range.0 {
                if new_command.y_range.0 <= command.y_range.1 && new_command.y_range.1 >= command.y_range.0 {
                    if new_command.z_range.0 <= command.z_range.1 && new_command.z_range.1 >= command.z_range.0 {
                        //println!("command {:?} overlaps with {:?}", command, new_command);
                    }
                }
            }
        }
    }
    new_commands
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

#[derive(Debug, Clone)]
struct Command {
    on: bool,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32)
}
