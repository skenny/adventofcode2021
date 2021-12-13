use crate::aoc;
use std::collections::HashMap;

pub fn run() {
    println!("AOC 2021 - Day 13");

    let sample_input = aoc::read_input("input/day13-sample.txt");
    println!("sample 1 = {}", part1(&sample_input));
    println!("sample 2 = {}", part2(&sample_input));

    let real_input = aoc::read_input("input/day13.txt");
    println!("part 1 = {}", part1(&real_input));
    println!("part 2 = {}", part2(&real_input));

    let jamie_input = aoc::read_input("input/day13-jamie.txt");
    println!("jamie 1 = {}", part1(&jamie_input));
    println!("jamie 2 = {}", part2(&jamie_input));    
}

fn part1(input: &[String]) -> usize {
    let (mut grid, folds) = parse_input(input);
    for fold in folds {
        apply_fold(&mut grid, fold);
        break
    }
    grid.len()
}

fn part2(input: &[String]) -> usize {
    let (mut grid, folds) = parse_input(input);
    for fold in folds {
        apply_fold(&mut grid, fold);
    }
    
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;

    for (x, y) in grid.keys() {
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    for y in 0..=max_y {
        let mut row: Vec<&str> = Vec::new();
        for x in 0..=max_x {
            if grid.contains_key(&(x, y)) {
                row.push("#");
            } else {
                row.push(" ");
            }
        }
        println!("{}", row.join(""));
    }

    0
}

fn apply_fold(grid: &mut HashMap<(u32, u32), bool>, fold: (&str, u32)) {
    let horizontal = fold.0 == "x";
    let fold_at = fold.1;

    let marked_points: Vec<&(u32, u32)> = grid.keys().collect();
    let mut points_to_add: Vec<(u32, u32)> = Vec::new();
    let mut points_to_remove: Vec<(u32, u32)> = Vec::new();

    for point in marked_points {
        let point_x = point.0;
        let point_y = point.1;

        if horizontal {
            if point_x > fold_at {
                let distance = point_x - fold_at;
                points_to_add.push((fold_at - distance, point_y));
                points_to_remove.push((point_x, point_y));
            }
        } else {
            if point_y > fold_at {
                let distance = point_y - fold_at;
                points_to_add.push((point_x, fold_at - distance));
                points_to_remove.push((point_x, point_y));
            }
        }
    }

    for point in points_to_add {
        grid.insert(point, true);
    }
    for point in points_to_remove {
        grid.remove(&point);
    }
}

fn parse_input(input: &[String]) -> (HashMap<(u32, u32), bool>, Vec<(&str, u32)>) {
    let mut grid: HashMap<(u32, u32), bool> = HashMap::new();
    let mut folds: Vec<(&str, u32)> = Vec::new();

    for (y, line) in input.iter().enumerate() {
        if line.starts_with("fold") {
            for (axis, pos) in line.strip_prefix("fold along ").unwrap().split_once("=") {
                folds.push((axis, pos.parse().unwrap()));
            }
        } else if line.len() > 0 {
            for (x, y) in line.split_once(',') {
                grid.insert((x.parse().unwrap(), y.parse().unwrap()), true);
            }
        }
    }

    (grid, folds)
}
