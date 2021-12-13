use crate::aoc;
use std::collections::HashMap;

pub fn run() {
    println!("AOC 2021 - Day 12");

    let sample_input_1 = aoc::read_input("input/day12-sample-1.txt");
    println!("sample 1 1 = {}", part1(&sample_input_1));
    //println!("sample 1 2 = {}", part2(&sample_input_1));

    let sample_input_2 = aoc::read_input("input/day12-sample-2.txt");
    println!("sample 2 1 = {}", part1(&sample_input_2));

    let sample_input_3 = aoc::read_input("input/day12-sample-3.txt");
    println!("sample 3 1 = {}", part1(&sample_input_3));

    let real_input = aoc::read_input("input/day12.txt");
    println!("part 1 = {}", part1(&real_input));
    //println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    let caves = parse_input(input);
    let paths = explore_caves(&caves, "start");
    paths.len()
}

fn part2(input: &[String]) -> usize {
    0
}

fn explore_caves(caves: &HashMap<String, Cave>, from_cave: &str) -> Vec<Vec<String>> {
    explore_caves2(caves, from_cave, &mut Vec::new())
}

fn explore_caves2(caves: &HashMap<String, Cave>, start_cave: &str, current_path: &mut Vec<String>) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();

    current_path.push(start_cave.to_string());

    //println!("exploring from {:?}", current_path);

    if start_cave == "end" {
        //println!("path: {:?}", current_path);
        paths.push(current_path.to_vec());
    } else {
        for connected_cave in (&caves[start_cave]).connected_caves.iter() {
            if caves[connected_cave].is_big() || !current_path.contains(connected_cave) {
                //println!("checking: {}", connected_cave);
                for path in explore_caves2(caves, connected_cave, current_path) {
                    paths.push(path.to_vec());
                }
            }
        }
    }

    current_path.pop();        

    paths
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
