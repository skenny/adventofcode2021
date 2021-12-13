use crate::aoc;
use std::collections::HashMap;

pub fn run() {
    println!("AOC 2021 - Day 12");

    let sample_input_1 = aoc::read_input("input/day12-sample-1.txt");
    println!("sample 1 1 = {}", part1(&sample_input_1));
    println!("sample 1 2 = {}", part2(&sample_input_1));

    let sample_input_2 = aoc::read_input("input/day12-sample-2.txt");
    println!("sample 2 1 = {}", part1(&sample_input_2));
    println!("sample 2 2 = {}", part2(&sample_input_2));

    let sample_input_3 = aoc::read_input("input/day12-sample-3.txt");
    println!("sample 3 1 = {}", part1(&sample_input_3));
    println!("sample 3 2 = {}", part2(&sample_input_3));

    let real_input = aoc::read_input("input/day12.txt");
    println!("part 1 = {}", part1(&real_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    let caves = parse_input(input);
    let paths = explore_caves(&caves, "start", 1);
    paths.len()
}

fn part2(input: &[String]) -> usize {
    let caves = parse_input(input);
    let paths = explore_caves(&caves, "start", 2);
    paths.len()
}

fn explore_caves(caves: &HashMap<String, Cave>, from_cave: &str, max_small_visits: usize) -> Vec<Vec<String>> {
    explore_caves2(caves, from_cave, &mut Vec::new(), max_small_visits, 0)
}

fn explore_caves2(caves: &HashMap<String, Cave>, current_cave_name: &str, current_path: &mut Vec<String>, max_small_visits: usize, indent: usize) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();

    let current_cave = &caves[current_cave_name];
    current_path.push(current_cave_name.to_string());

    if current_cave_name == "end" {
        paths.push(current_path.to_vec());
    } else {
        for connected_cave in current_cave.connected_caves.iter() {
            if connected_cave != "start" {
                let connected_cave_visits = if caves[connected_cave].is_big() { 0 } else { count_visits(current_path, connected_cave) };
                if connected_cave_visits < max_small_visits {
                    let new_max_small_visits = if connected_cave_visits + 1 == max_small_visits { 1 } else { max_small_visits };
                    for path in explore_caves2(caves, connected_cave, current_path, new_max_small_visits, indent+1) {
                        paths.push(path.to_vec());
                    }
                }
            }
        }
    }

    current_path.pop();

    paths
}

fn count_visits(path: &Vec<String>, cave: &str) -> usize {
    path.iter().filter(|&path_cave| path_cave == cave).count()
}

fn parse_input(input: &[String]) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();

    for line in input {
        let (c1, c2) = line.split_once('-').unwrap();

        let cave1 = caves.entry(c1.to_string()).or_insert(Cave::new(c1.to_string()));
        cave1.connect_cave(c2.to_string());

        let cave2 = caves.entry(c2.to_string()).or_insert(Cave::new(c2.to_string()));
        cave2.connect_cave(c1.to_string());
    }

    caves
}

#[derive(Debug)]
struct Cave {
    name: String,
    connected_caves: Vec<String>
}

impl Cave {
    fn new(name: String) -> Cave {
        Cave {
            name: name,
            connected_caves: Vec::new()
        }
    }

    fn is_big(&self) -> bool {
        self.name.to_uppercase() == self.name
    }

    fn connect_cave(&mut self, cave_name: String) {
        self.connected_caves.push(cave_name);
    }
}
