use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 25");

    let sample_input = aoc::read_input("input/day25-sample.txt");
    println!("part 1 = {}", part1(&sample_input));

    let real_input = aoc::read_input("input/day25.txt");
    println!("part 1 = {}", part1(&real_input));
}

fn part1(input: &[String]) -> usize {
    let mut grid = parse_input(input);
    let mut steps = 0;

    loop {
        let mut new_grid = grid.clone();
        let grid_len = grid.len();
        let row_len = grid[0].len();

        let mut move_rights: Vec<((usize, usize), (usize, usize))> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..row_len {
                let next_space = if x < row_len - 1 { x + 1 } else { 0 };
                if grid[y][x] == '>' && grid[y][next_space] == '.' {
                    move_rights.push(((x, y), (next_space, y)));
                }
            }
        }
        for ((x, y), (nx, ny)) in move_rights {
            new_grid[y][x] = '.';
            new_grid[ny][nx] = '>';
        }

        let mut move_downs: Vec<((usize, usize), (usize, usize))> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let next_space = if y < grid_len - 1 { y + 1 } else { 0 };
                if grid[y][x] == 'v' && new_grid[next_space][x] == '.' {
                    move_downs.push(((x, y), (x, next_space)));
                }
            }
        }
        for ((x, y), (nx, ny)) in move_downs {
            new_grid[y][x] = '.';
            new_grid[ny][nx] = 'v';
        }

        steps += 1;

        if stringify_grid(&grid) == stringify_grid(&new_grid) {
            break;
        }

        grid = new_grid;
    }

    println!("Final grid:");
    print_grid(&grid);

    steps
}

fn stringify_grid(grid: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    grid.iter().for_each(|row| result.push_str(&String::from_iter(row.into_iter())));
    result
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", String::from_iter(row.into_iter()));
    }
}

fn parse_input(input: &[String]) -> Vec<Vec<char>> {
    input.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}
