use crate::aoc;

const GRID_SIZE: usize = 10;
const NEIGHBOURS: [(i8, i8); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];

pub fn run() {
    println!("AOC 2021 - Day 11");

    let sample_input = aoc::read_input("input/day11-sample.txt");
    let real_input = aoc::read_input("input/day11.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    println!("sample 2 = {}", part2(&sample_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> u64 {
    let mut grid = parse_input(input);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut grid);
    }
    flashes
}

fn part2(input: &[String]) -> u64 {
    let mut grid = parse_input(input);
    let mut step_count = 0;
    let grid_size = GRID_SIZE as u64 * GRID_SIZE as u64;
    while step(&mut grid) < grid_size {
        step_count += 1;
    }
    step_count + 1
}

fn print_grid(grid: &[[(u32, bool); GRID_SIZE]; GRID_SIZE]) {
    for y in 0..GRID_SIZE {
        let mut row: Vec<String> = Vec::new();
        for x in 0..GRID_SIZE {
            row.push(grid[y][x].0.to_string());
        }
        println!("{}", row.join(""));
    }
}

fn step(grid: &mut [[(u32, bool); GRID_SIZE]; GRID_SIZE]) -> u64 {
    // increment energy
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let octopus = grid[y][x];
            update_octopus(grid, x, y, octopus.0 + 1, octopus.1);
        }
    }

    // loop for flashes
    check_for_flashes(grid);

    // count flashes, and reset octopi
    let mut flashes = 0;
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let octopus = grid[y][x];
            if octopus.1 {
                flashes += 1;
                update_octopus(grid, x, y, 0, false);
            }
        }
    }
    flashes
}

fn check_for_flashes(grid: &mut [[(u32, bool); GRID_SIZE]; GRID_SIZE]) {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let octopus = grid[y][x];
            // if the octopus' energy > 9 and it hasn't flashed, flash
            if octopus.0 > 9 && !octopus.1 {
                flash(grid, x, y);
            }
        }
    }
}

fn flash(grid: &mut [[(u32, bool); GRID_SIZE]; GRID_SIZE], x: usize, y: usize) {
    let octopus = grid[y][x];
    update_octopus(grid, x, y, octopus.0, true);
    
    for (nx, ny) in find_neighbours(x, y) {
        let neighbour_octopus = grid[ny][nx];
        if !neighbour_octopus.1 {
            // if the neighbour octopus hasn't flashed, increase its energy
            update_octopus(grid, nx, ny, neighbour_octopus.0 + 1, neighbour_octopus.1);
        }
    }

    check_for_flashes(grid);
}

fn find_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    for vector in NEIGHBOURS {
        let nx = x as i8 + vector.0;
        let ny = y as i8 + vector.1;
        if nx >= 0 && nx < GRID_SIZE as i8 && ny >= 0 && ny < GRID_SIZE as i8 {
            neighbours.push((nx as usize, ny as usize));
        }
    }

    neighbours
}

fn update_octopus(grid: &mut [[(u32, bool); GRID_SIZE]; GRID_SIZE], x: usize, y: usize, energy: u32, flashed: bool) {
    grid[y][x] = (energy, flashed);
}

fn parse_input(input: &[String]) -> [[(u32, bool); GRID_SIZE]; GRID_SIZE] {
    let mut grid: [[(u32, bool); GRID_SIZE]; GRID_SIZE] = [[(0, false); GRID_SIZE]; GRID_SIZE];

    assert_eq!(GRID_SIZE, input.len());
    assert_eq!(GRID_SIZE, input[0].len());

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            update_octopus(&mut grid, x, y, c.to_digit(10).unwrap(), false);
        }
    }
    grid
}
