use crate::aoc;

const NEIGHBOURS: [(i8, i8); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];

pub fn run() {
    println!("AOC 2021 - Day 15");

    let sample_input = aoc::read_input("input/day15-sample.txt");
    println!("sample 1 = {}", part1(&sample_input));
    //println!("sample 2 = {}", part2(&sample_input));

    // let real_input = aoc::read_input("input/day15.txt");
    // println!("part 1 = {}", part1(&real_input));
    // println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> u64 {
    let grid = parse_input(input);
    print_grid(&grid);
    0
}

fn part2(input: &[String]) -> u64 {
    0
}

fn find_neighbours(x: usize, y: usize, grid_size: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    for vector in NEIGHBOURS {
        let nx = x as i8 + vector.0;
        let ny = y as i8 + vector.1;
        if nx >= 0 && nx < grid_size as i8 && ny >= 0 && ny < grid_size as i8 {
            neighbours.push((nx as usize, ny as usize));
        }
    }

    neighbours
}

fn print_grid(grid: &Vec<Vec<i8>>) {
    for line in grid {
        println!("{}", line.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""));
    }
}

fn parse_input(input: &[String]) -> Vec<Vec<i8>> {
    let mut grid: Vec<Vec<i8>> = Vec::new();
    for line in input.iter() {
        let mut row = vec![0; line.len()];
        for (x, c) in line.chars().enumerate() {
            row[x] = c.to_digit(10).unwrap() as i8;
        }
        grid.push(row);
    }
    grid
}
