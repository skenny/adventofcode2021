use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 21");

    let mut start = std::time::Instant::now();
    println!("sample 1 = {}", part1(4, 8, 1000));
    println!("sample 2 = {} ({}s)", part2(4, 8, 21), start.elapsed().as_secs());

    start = std::time::Instant::now();
    println!("part 1 = {}", part1(7, 2, 1000));
    println!("part 2 = {} ({}s)", part2(7, 2, 21), start.elapsed().as_secs());
}

fn part1(p1_start: i32, p2_start: i32, winning_score: i32) -> i32 {
    let mut player_1 = Player { num: 1, pos: p1_start, score: 0 };
    let mut player_2 = Player { num: 2, pos: p2_start, score: 0 };
    let mut turn = true;    // true or player 1, false or player 2
    let mut roll_count = 0;
    let mut die = 0;

    while player_1.score < winning_score && player_2.score < winning_score {
        let mut rolls = [0; 3];
        for roll in 0..3 {
            die += 1;
            if die > 100 {
                die %= 100;
            }
            rolls[roll] = die;
            roll_count += 1;
        }
        let score: i32 = rolls.iter().sum();
        if turn {
            player_1.score(score);
        } else {
            player_2.score(score);
        }
        turn = !turn;
    }

    let loser = if player_1.score > player_2.score { player_2 } else { player_1 };
    loser.score * roll_count
}

fn part2(p1_start: i32, p2_start: i32, winning_score: i32) -> i64 {
    let starting_game = Part2Game {
        player_1: Player { num: 1, pos: p1_start, score: 0 },
        player_2: Player { num: 2, pos: p2_start, score: 0 },
        turn: true,
        count: 1
    };

    // let mut score_counts = [0; 10];
    // for combo in starting_game.generate_die_permutations(3) {
    //     let score: i32 = combo.iter().sum();
    //     score_counts[score as usize] += 1;
    // }
    // println!("score counts: {:?}", &score_counts);

    let mut player_1_wins = 0;
    let mut player_2_wins = 0;

    let mut continuing_games = Vec::new();
    continuing_games.push(starting_game);
    let mut counter: i64 = 0;

    // player_2_wins= 12679313610406

    while !continuing_games.is_empty() {
        if counter % 10000000 == 0 {
            println!("after {} loops, there are {} ongoing games; player_1_wins={}, player_2_wins={}", counter, continuing_games.len(), player_1_wins, player_2_wins);
        }
        let mut game = continuing_games.pop().unwrap();
        //println!("playing game {:?}", game);
        for split_game in game.play_turn() {
            //println!("\tgot game {:?}", split_game);
            if split_game.is_over(winning_score) {
                //println!("\t\tgame over! {} vs {}", split_game.player_1.score, split_game.player_2.score);
                if split_game.player_1.score > split_game.player_2.score {
                    player_1_wins += split_game.count;
                } else {
                    player_2_wins += split_game.count;
                }
            } else {
                //println!("\t\tno winner yet, the game continues...");
                continuing_games.push(split_game);
            }
        }
        counter += 1;
    }

    std::cmp::max(player_1_wins, player_2_wins)
}

#[derive(Debug)]
struct Part2Game {
    player_1: Player,
    player_2: Player,
    turn: bool,
    count: i64
}

impl Part2Game {
    fn play_turn(&mut self) -> Vec<Part2Game> {
        let mut games = Vec::new();
        for (score, count) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] { //self.generate_unique_score_counts().iter().enumerate() {
            if count > 0 {
                let mut split_game = Part2Game { 
                    player_1: self.player_1.clone(), 
                    player_2: self.player_2.clone(), 
                    turn: self.turn, 
                    count: self.count * count as i64
                };
                if split_game.turn {
                    split_game.player_1.score(score as i32);
                } else {
                    split_game.player_2.score(score as i32);
                }
                split_game.turn = !split_game.turn;
                games.push(split_game);
            }
        }
        games
    }
    
    fn generate_die_permutations(&self) -> Vec<[i32; 3]> {
        let mut iterations = Vec::new();
        for roll_1 in 1..=3 {
            let mut die_combo = [0; 3];
            die_combo[0] = roll_1;
            for roll_2 in 1..=3 {
                die_combo[1] = roll_2;
                for roll_3 in 1..=3 {
                    die_combo[2] = roll_3;
                    iterations.push(die_combo);
                }
            }
        }
        //println!("die iterations: {:?}", iterations);
        iterations
    }

    fn generate_unique_score_counts(&self) -> [i32; 10] {
        let mut score_counts = [0; 10];
        for combo in self.generate_die_permutations() {
            let score: i32 = combo.iter().sum();
            score_counts[score as usize] += 1;
        }
        score_counts
    }

    fn is_over(&self, winning_score: i32) -> bool {
        self.player_1.score >= winning_score || self.player_2.score >= winning_score
    }
}

#[derive(Debug, Clone)]
struct Player {
    num: i8,
    pos: i32,
    score: i32
}

impl Player {
    fn score(&mut self, sum: i32) {
        let old_pos = self.pos;
        let mut new_pos = self.pos + (sum % 10);
        if new_pos > 10 {
            new_pos %= 10;
        }
        self.pos = new_pos;
        self.score += new_pos;
        //println!("Player {} rolls {} {:?} and moves from space {} to space {} for a total score of {}.", self.num, sum, rolls, old_pos, new_pos, self.score);
    }
}