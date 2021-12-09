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

fn explore(input: &[String]) -> (u32, u64) {
    let mut risk_levels: Vec<u32> = Vec::new();
    let mut basin_sizes: Vec<u64> = Vec::new();

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
    (risk_levels.iter().sum(), basin_sizes.iter().take(3).fold(1, |acc, size| acc * size))
}

fn is_low_point(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let height = grid[y][x];
    let neighbours = find_neighbours(grid, x, y);
    neighbours.iter().all(|(x, y)| height < grid[*y][*x])
}

fn find_basin_size(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u64 {
    let mut basin_points: Vec<(usize, usize)> = Vec::new();
    recurse_basin(grid, x, y, &mut basin_points);
    basin_points.len() as u64
}

fn recurse_basin(grid: &Vec<Vec<u32>>, x: usize, y: usize, checked_points: &mut Vec<(usize, usize)>) {
    checked_points.push((x, y));
    let height = grid[y][x];

    for (n_x, n_y) in find_neighbours(grid, x, y) {
        if !checked_points.contains(&(n_x, n_y)) {
            let neighbour_height = grid[n_y][n_x];
            if neighbour_height < 9 && neighbour_height > height {
                recurse_basin(grid, n_x, n_y, checked_points);
            }
        }
    }
}

fn find_neighbours(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    if y > 0 { neighbours.push((x, y-1)); }
    if y < grid.len() - 1 { neighbours.push((x, y+1)); }
    if x > 0 { neighbours.push((x-1, y)); }
    if x < grid[y].len() - 1 { neighbours.push((x+1, y)); }

    neighbours
}

fn parse_input(input: &[String]) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }
    grid
}
