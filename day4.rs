use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct BingoCard {
    grid: [[(i32, bool); 5]; 5],
    winner: bool
}

impl BingoCard {
    fn new() -> BingoCard {
        BingoCard {
            grid: [[(0, false); 5]; 5],
            winner: false
        }
    }

    fn set_num(&mut self, y: usize, x: usize, num: i32) {
        self.grid[y][x] = (num, false);
    }

    fn call_num(&mut self, num: i32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.grid[y][x].0 == num {
                    self.grid[y][x].1 = true;
                }
            }
        }
    }

    fn is_called(&self, y: usize, x: usize) -> bool {
        self.grid[y][x].1
    }

    fn is_winner(&mut self) -> bool {
        for y in 0..5 {
            let mut winner = true;
            for x in 0..5 {
                winner = winner && self.is_called(y, x);
            }
            if winner {
                self.winner = true;
            }
        }
        for x in 0..5 {
            let mut winner = true;
            for y in 0..5 {
                winner = winner && self.is_called(y, x);
            }
            if winner {
                self.winner = true;
            }
        }
        self.winner
    }

    fn score(&self, last_num: i32) -> usize {
        let mut sum: usize = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.grid[y][x].1 {
                    sum += self.grid[y][x].0 as usize;
                }
            }
        }
        sum * last_num as usize
    }
}

impl std::fmt::Debug for BingoCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}\n{:?}\n{:?}\n{:?}\n{:?}", self.grid[0], self.grid[1], self.grid[2], self.grid[3], self.grid[4])
    }
}

pub fn run() {
    println!("AOC 2021 - Day 4");

    let sample_input = read_input("input/day4-sample.txt");
    let real_input = read_input("input/day4.txt");

    println!("sample 1 = {}", part1(&sample_input));
    println!("part 1 = {}", part1(&real_input));

    println!("sample 2 = {}", part2(&sample_input));
    println!("part 2 = {}", part2(&real_input));
}

fn parse_input(input: &[String]) -> (Vec<i32>, Vec<BingoCard>) {
    let numbers: Vec<i32> = input[0].split(',').map(|s| s.parse().unwrap()).collect();

    let mut cards: Vec<BingoCard> = Vec::new();
    let mut row_num: usize = 0;

    for i in 1..input.len() {
        let input_row = &input[i];
        
        if input_row == "" {
            cards.push(BingoCard::new());
            row_num = 0;
            continue;
        }

        let mut card = cards.pop().unwrap();
        let card_row: Vec<&str> = input_row.split_whitespace().collect();
        for (col_num, num_char) in card_row.iter().enumerate() {
            card.set_num(row_num, col_num, num_char.parse().unwrap())
        }

        cards.push(card);
        row_num += 1;
    }

    (numbers, cards)
}

fn part1(input: &[String]) -> usize {
    let (numbers, mut cards) = parse_input(input);

    for number in numbers {
        for card in cards.iter_mut() {
            card.call_num(number);
            if card.is_winner() {
                return card.score(number);
            }
        }
    }

    0
}

fn part2(input: &[String]) -> usize {
    let (numbers, mut cards) = parse_input(input);

    let num_cards = cards.len();
    let mut winner_count = 0;

    for number in numbers {
        for card in cards.iter_mut() {
            card.call_num(number);
            if !card.winner && card.is_winner() {
                winner_count += 1;
                if winner_count == num_cards {
                    return card.score(number);
                }
            }
        }
    }

    0
}

fn read_input(filename: &str) -> Vec<String> {
    let mut v = Vec::new();
    let file = File::open(filename).expect("cannot find file!");
    let reader = BufReader::new(file);

    // very inefficient, creates a new String for each line in the file
    // tutorial has better examples of how to read lines from a file more efficiently
    for line in reader.lines() {
        v.push(line.unwrap());
    }

    v
}
