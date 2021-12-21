use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 21");

    println!("sample 1 = {}", play(4, 8, 1000));
    //println!("sample 2 = {}", 0);

    println!("part 1 = {}", play(7, 2, 1000));
    //println!("part 2 = {}", enhance(&real_input, 50));
}

fn play(p1_start: i32, p2_start: i32, winning_score: i32) -> i32 {
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
        if turn {
            player_1.score(rolls);
        } else {
            player_2.score(rolls);
        }
        turn = !turn;
    }

    let loser = if player_1.score > player_2.score { player_2 } else { player_1 };
    loser.score * roll_count
}

struct Player {
    num: i8,
    pos: i32,
    score: i32
}

impl Player {
    fn score(&mut self, rolls: [i32; 3]) {
        let sum: i32 = rolls.iter().sum();
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