use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 20");

    let sample_input = aoc::read_input("input/day20-sample.txt");
    part1(&sample_input);

    //let real_input = aoc::read_input("input/day20.txt");
    //part1(&real_input);
}

fn part1(input: &[String]) {
    let (_enhancement_algo, mut image) = parse_input(input);
    println!("part 1 = {}", 0);
}

fn expand(vec: &mut Vec<Vec<bool>>) {
    let original_height = vec.len();
    vec.insert(0, vec![false; original_height + 2]);
    vec.push(vec![false; original_height + 2]);
    for row in 1..=original_height {
        vec[row].insert(0, false);
        vec[row].push(false)
    }
}

fn parse_input(input: &[String]) -> (String, Vec<Vec<bool>>) {
    let enhancement_algo = input[0].to_string();
    let mut image: Vec<Vec<bool>> = Vec::new();
    for line in 2..input.len() {
        image.push(input[line].chars().map(|c| if c == '.' { false } else { true }).collect());
    }
    (enhancement_algo, image)
}

fn print_image(image: &Vec<Vec<bool>>) {
    for row in image {
        println!("{}", row.iter().map(|pixel| if *pixel { "#" } else { "." }).collect::<Vec<&str>>().join(""));
    }
}