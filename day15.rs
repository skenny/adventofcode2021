use crate::aoc;

const NEIGHBOURS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

static mut best_path: i64 = i64::MAX;

pub fn run() {
    println!("AOC 2021 - Day 15");

    let sample_input = aoc::read_input("input/day15-sample.txt");
    println!("sample 1 = {}", part1(&sample_input));
    //println!("sample 2 = {}", part2(&sample_input));

    let real_input = aoc::read_input("input/day15.txt");
    println!("part 1 = {}", part1(&real_input));
    // println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> i64 {
    let (grid, grid_size) = parse_input(input);
    unsafe {
        best_path = i64::MAX;
    }
    for (nx, ny) in find_neighbours(0, 0, grid_size) {
        println!("from start, going into {},{}", nx, ny);
        let mut visited = vec![(0, 0)];
        enter(nx, ny, &grid, grid_size, &mut visited);
    }
    unsafe {
        return best_path;
    }
}

fn part2(input: &[String]) -> i64 {
    0
}

fn enter(x: usize, y: usize, grid: &Vec<Vec<i8>>, grid_size: usize, visited: &mut Vec<(usize, usize)>) {
    visited.push((x, y));
    
    unsafe {

        let current_path_risk = calc_risk(visited, grid);
        if current_path_risk < best_path {
            if x == grid_size-1 && y == grid_size-1 {
                best_path = current_path_risk;
            } else {
                let mut neighbours = find_neighbours(x, y, grid_size);

                // remove all neighbours we've already visited
                neighbours.retain(|p| !visited.contains(p));

                for (nx, ny) in neighbours {
                    enter(nx, ny, grid, grid_size, visited);
                }
            }
        }

    }

    visited.pop();
}

fn calc_risk(path: &Vec<(usize, usize)>, grid: &Vec<Vec<i8>>) -> i64 {
    path.iter().fold(0, |acc, (x, y)| grid[*y][*x] as i64 + acc) - (grid[0][0] as i64)
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

fn parse_input(input: &[String]) -> (Vec<Vec<i8>>, usize) {
    let mut grid: Vec<Vec<i8>> = Vec::new();
    let mut grid_size = 0;
    for (y, line) in input.iter().enumerate() {
        if y == 0 { grid_size = line.len(); }
        let mut row = vec![0; grid_size];
        for (x, c) in line.chars().enumerate() {
            row[x] = c.to_digit(10).unwrap() as i8;
        }
        grid.push(row);
    }
    //print_grid(&grid);
    (grid, grid_size)
}
