use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 9");

    let sample_input = aoc::read_input("input/day9-sample.txt");
    let real_input = aoc::read_input("input/day9.txt");

    let sample_output = explore(&sample_input);
    let real_output = explore(&real_input);

    println!("sample 1 = {}", sample_output.0);
    println!("part 1 = {}", real_output.0);

    println!("sample 2 = {}", sample_output.1);
    println!("part 2 = {}", real_output.1);
}

fn explore(input: &[String]) -> (u32, u32) {
    let mut risk_levels: Vec<u32> = Vec::new();
    let mut basin_sizes: Vec<u32> = Vec::new();

    let grid: Vec<Vec<u32>> = parse_input(input);
    let row_len = grid[0].len();

    for y in 0..grid.len() {
        for x in 0..row_len {
            if is_low_point(&grid, x, y) {
                risk_levels.push(grid[y][x] + 1);
                basin_sizes.push(find_basin_size(&grid, x, y));
            }
        }
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    (risk_levels.iter().sum(), basin_sizes.iter().fold(1, |acc, size| acc * size))
}

fn is_low_point(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let height = grid[y][x];
    let neighbours = find_neighbours(grid, x, y);
    neighbours.iter().all(|v| height < *v)
}

fn find_basin_size(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let basin_size = 1;
    
    basin_size
}

fn find_neighbours(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<u32> {
    let mut neighbours: Vec<u32> = Vec::new();

    if y > 0 { neighbours.push(grid[y-1][x]); }
    if y < grid.len() - 1 { neighbours.push(grid[y+1][x]); }
    if x > 0 { neighbours.push(grid[y][x-1]); }
    if x < grid[y].len() - 1 { neighbours.push(grid[y][x+1]); }

    neighbours
}

fn parse_input(input: &[String]) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }
    grid
}
