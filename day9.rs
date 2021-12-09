use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 9");

    let sample_input = aoc::read_input("input/day9-sample.txt");
    let real_input = aoc::read_input("input/day9.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    //println!("sample 2 = {}", part2(&sample_input));
    //println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> u32 {
    let mut low_points: Vec<u32> = Vec::new();
    let grid: Vec<Vec<u32>> = parse_input(input);
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            let height = row[x];
            let height_up = if y > 0 { grid[y-1][x] } else { u32::MAX };
            let height_down = if y < grid.len() - 1 { grid[y+1][x] } else { u32::MAX };
            let height_left = if x > 0 { grid[y][x-1] } else { u32::MAX };
            let height_right = if x < row.len() - 1 { grid[y][x+1] } else { u32::MAX };
            if height < height_up && height < height_down && height < height_left && height < height_right {
                low_points.push(height);
            }
        }
    }

    // why rust, why
    let sum: u32 = low_points.iter().sum();
    let size: usize = low_points.len();
    return size as u32 + sum;
}

fn part2(input: &[String]) -> usize {
    0
}

fn parse_input(input: &[String]) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }
    grid
}
