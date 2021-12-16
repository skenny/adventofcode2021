use crate::aoc;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const NEIGHBOURS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn run() {
    println!("AOC 2021 - Day 15");

    let sample_input = aoc::read_input("input/day15-sample.txt");
    println!("sample 1 = {}", part1(&sample_input));
    println!("sample 2 = {}", part2(&sample_input));

    let real_input = aoc::read_input("input/day15.txt");
    println!("part 1 = {}", part1(&real_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    let cave = parse_input(input);
    cave.find_least_risky_path().unwrap()
}

fn part2(input: &[String]) -> usize {
    let mut cave = parse_input(input);
    cave.asplode(5);
    cave.find_least_risky_path().unwrap()
}

fn parse_input(input: &[String]) -> Cave {
    let mut chiton_risks: Vec<u8> = Vec::new();
    let mut grid_size = 0;
    for line in input.iter() {
        grid_size = line.len();
        line.chars().for_each(|ch| chiton_risks.push(ch.to_digit(10).unwrap() as u8));
    }
    Cave { chiton_risks: chiton_risks, grid_size: grid_size }
}

#[derive(Debug)]
struct Cave {
    chiton_risks: Vec<u8>,
    grid_size: usize
}

impl Cave {

    fn print_grid(&self) {
        self.chiton_risks
            .chunks(self.grid_size)
            .for_each(|row| println!("{}", row.iter().map(|risk| risk.to_string()).collect::<Vec<String>>().join("")));
    }

    fn find_neighbours(&self, pos: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let point = self.position_to_point(pos);
        for vector in NEIGHBOURS {
            let nx = point.0 as i32 + vector.0;
            let ny = point.1 as i32 + vector.1;
            if nx >= 0 && nx < self.grid_size as i32 && ny >= 0 && ny < self.grid_size as i32 {
                neighbours.push(self.point_to_position(nx as usize, ny as usize));
            }
        }
        //println!("neighbours of {} are {:?}", pos, &neighbours);
        neighbours
    }

    fn find_least_risky_path(&self) -> Option<usize> {
        // djikstra, from https://doc.rust-lang.org/std/collections/binary_heap/index.html

        let num_positions = self.grid_size * self.grid_size;
        let end = num_positions - 1;

        // compute each position's neighbours
        let position_neighbours: Vec<Vec<usize>> = (0..num_positions).map(|pos| self.find_neighbours(pos)).collect();

        let mut dist: Vec<usize> = (0..self.grid_size * self.grid_size).map(|_| usize::MAX).collect();
        let mut visited: Vec<usize> = Vec::new();
        let mut heap = BinaryHeap::new();

        dist[0] = 0;
        visited.push(0);
        heap.push(Chiton { risk: 0, position: 0 });

        let started_at = std::time::Instant::now();

        while let Some(Chiton { risk, position }) = heap.pop() {
            if position == end {
                println!("Finished after {}s", started_at.elapsed().as_secs());
                return Some(risk);
            }
            if risk > dist[position] {
                continue;
            }
            if risk >= dist[end] {
                continue;
            }

            for neighbour_position in &position_neighbours[position] {
                let neighbour_risk = risk + self.chiton_risks[*neighbour_position] as usize;
                if neighbour_risk < dist[*neighbour_position] && neighbour_risk < dist[end] && !visited.contains(&neighbour_position) {
                    heap.push(Chiton { risk: neighbour_risk, position: *neighbour_position });
                    visited.push(*neighbour_position);
                    dist[*neighbour_position] = neighbour_risk;
                }
            }

            if visited.len() % 1000 == 0 {
                println!("{} nodes remaining, elapsed time is {}s...", dist.len() - visited.len(), started_at.elapsed().as_secs());
            }
        }

        println!("Finished without a path after {}s", started_at.elapsed().as_secs());

        None
    }

    fn point_to_position(&self, x: usize, y: usize) -> usize {
        y * self.grid_size + x
    }

    fn position_to_point(&self, position: usize) -> (usize, usize) {
        (position % self.grid_size, position / self.grid_size)
    }

    fn asplode(&mut self, asplode_by: usize) {
        let mut new_chiton_risks: Vec<u8> = Vec::new();
        let new_grid_size = self.grid_size * asplode_by;

        // asplode the original cave horizontally
        for row in self.chiton_risks.chunks(self.grid_size) {
            let mut new_row: Vec<u8> = vec![0 as u8; new_grid_size];
            for iteration in 0..asplode_by {
                for (x, risk) in row.iter().enumerate() {
                    let mut adjusted_risk = *risk + iteration as u8;
                    if adjusted_risk > 9 {
                        adjusted_risk = (adjusted_risk % 10) + 1;
                    }
                    new_row[iteration * self.grid_size + x] = adjusted_risk;
                }
            }
            new_chiton_risks.extend(new_row);
        }

        let template = new_chiton_risks.clone();

        // repeat template_rows vertically for each additional iteration
        for iteration in 1..asplode_by {
            for template_row in template.chunks(new_grid_size) {
                let mut new_row: Vec<u8> = vec![0 as u8; template_row.len()];
                for (x, risk) in template_row.iter().enumerate() {
                    let mut adjusted_risk = *risk + iteration as u8;
                    if adjusted_risk > 9 {
                        adjusted_risk = (adjusted_risk % 10) + 1;
                    }
                    new_row[x] = adjusted_risk;
                }
                new_chiton_risks.extend(new_row);
            }
        }

        self.chiton_risks = new_chiton_risks;
        self.grid_size = new_grid_size;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Chiton {
    risk: usize,
    position: usize,
}

impl Ord for Chiton {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Chiton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
