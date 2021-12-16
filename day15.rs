use crate::aoc;

const NEIGHBOURS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn run() {
    println!("AOC 2021 - Day 15");

    let sample_input = aoc::read_input("input/day15-sample.txt");
    println!("sample 1 = {}", part1(&sample_input));
    //println!("sample 2 = {}", part2(&sample_input));

    let real_input = aoc::read_input("input/day15.txt");
    println!("part 1 = {}", part1(&real_input));
    // println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> i32 {
    let mut cave = parse_input(input);
    cave.find_least_risky_path()
}

fn part2(input: &[String]) -> i32 {
    0
}

fn parse_input(input: &[String]) -> Cave {
    let mut grid: Vec<Vec<i8>> = Vec::new();
    let mut grid_size = 0;
    for (y, line) in input.iter().enumerate() {
        if y == 0 { 
            grid_size = line.len();
        }
        let mut row = vec![0; grid_size];
        line.chars().enumerate().for_each(|(x, c)| row[x] = c.to_digit(10).unwrap() as i8);
        grid.push(row);
    }
    Cave::new(grid, grid_size)
}

#[derive(Debug)]
struct Cave {
    chiton_risks: Vec<Vec<i8>>,
    grid_size: usize,
    lowest_risk_score: i32,
    paths: Vec<Vec<(usize, usize)>>
}

impl Cave {

    fn new(chiton_risks: Vec<Vec<i8>>, grid_size: usize) -> Cave {
        Cave {
            chiton_risks: chiton_risks,
            grid_size: grid_size,
            lowest_risk_score: i32::MAX,
            paths: Vec::new()
        }        
    }

    fn print_grid(&self) {
        self.chiton_risks
            .iter()
            .for_each(|row| println!("{}", row.iter().map(|risk| risk.to_string()).collect::<Vec<String>>().join("")));
    }

    fn find_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<(usize, usize)> = Vec::new();
        for vector in NEIGHBOURS {
            let nx = x as i8 + vector.0;
            let ny = y as i8 + vector.1;
            if nx >= 0 && nx < self.grid_size as i8 && ny >= 0 && ny < self.grid_size as i8 {
                neighbours.push((nx as usize, ny as usize));
            }
        }
        neighbours
    }

    fn calc_path_risk(&self, path: &Vec<(usize, usize)>) -> i32 {
        path.iter().fold(0, |acc, (x, y)| self.chiton_risks[*y][*x] as i32 + acc)
    }
    
    fn find_least_risky_path(&mut self) -> i32 {
        for (nx, ny) in self.find_neighbours(0, 0) {
            println!("from start, checking {},{}", nx, ny);
            let mut visited = vec![(0, 0)];
            self.enter(nx, ny, &mut visited);
        }
        self.lowest_risk_score
    }

    fn enter(&mut self, x: usize, y: usize, visited: &mut Vec<(usize, usize)>) {
        let current_path_risk = self.calc_path_risk(visited);
        if current_path_risk >= self.lowest_risk_score {
            return;
        }

        visited.push((x, y));

        if self.is_end(x, y) {
            self.paths.push(visited.clone());
            let path_count = self.paths.len();
            if path_count % 10 == 0 {
                println!("so far we've found {} paths...", path_count)
            }

            if current_path_risk < self.lowest_risk_score {
                println!("new best path! {:<3}", current_path_risk);
                self.lowest_risk_score = current_path_risk;
            }
        } else {
            let mut neighbours = self.find_neighbours(x, y);

            // remove all neighbours we've already visited
            neighbours.retain(|p| !visited.contains(p));

            let mut neighbour_risks = neighbours
                .iter()
                .map(|p| (*p, self.chiton_risks[p.1][p.0])).collect::<Vec<((usize, usize), i8)>>();
            // sort by risk, lowest first
            //neighbour_risks.sort_by(|(_, l_risk), (_, r_risk)| l_risk.cmp(r_risk));

            for ((nx, ny), _) in neighbour_risks {
                self.enter(nx, ny, visited);
            }
        }

        visited.pop();
    }

    fn is_end(&self, x: usize, y: usize) -> bool {
        (self.grid_size - 1, self.grid_size - 1) == (x, y)
    }

}
