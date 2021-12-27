use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 18");

    assert_eq!("[[[[0,9],2],3],4]", reduce(String::from("[[[[[9,8],1],2],3],4]")));
    assert_eq!("[7,[6,[5,[7,0]]]]", reduce(String::from("[7,[6,[5,[4,[3,2]]]]]")));
    assert_eq!("[[6,[5,[7,0]]],3]", reduce(String::from("[[6,[5,[4,[3,2]]]],1]")));
    //assert_eq!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", reduce(String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")));
    assert_eq!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", reduce(String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")));

    let sample_1 = aoc::read_input("input/day18-sample-1.txt");
    //part1(&sample_1);

    //let real_input = aoc::read_input("input/day18.txt");
    //part1(&real_input);
}

fn part1(input: &[String]) {
    println!("part 1 = {}", 0);

    let mut snailfish = Vec::from(input);
    while snailfish.len() > 1 {
        snailfish = add(&snailfish);
    }
}

fn add(snailfish: &Vec<String>) -> Vec<String> {
    let left = &snailfish[0];
    let right = &snailfish[1];
    println!("adding {} and {}...", left, right);
    let combined = format!("[{},{}]", left, right);
    let reduced = reduce(combined);

    let mut new_snailfish = vec![reduced];
    for a in &snailfish[2..] {
        new_snailfish.push(a.to_string());
    }

    new_snailfish
}

fn reduce(snailfish: String) -> String {
    let debug = false;

    if debug {
        println!("reducing {}...", snailfish);
    }

    let mut chars: Vec<char> = snailfish.chars().collect();
    let mut i = 0usize;
    let mut depth = 0i32;

    let mut looking_for_explosions = true;

    while i < chars.len() {
        let ch = chars[i];
        
        // explodes
        if looking_for_explosions {
            if ch == '[' {
                depth += 1;
                if debug { println!("\tat pos {}, depth increases to {}", i, depth); }
            } else if ch == ']' {
                depth -= 1;
                if debug { println!("\tat pos {}, depth decreases to {}", i, depth); }
            } else if ch.is_numeric() {
                let pair_start = i;
                while chars[i+1].is_numeric() {
                    i += 1;
                }
                if chars[i + 1] == ',' {
                    i += 1;
                    if chars[i + 1].is_numeric() {
                        // we have a pair!
                        while chars[i + 1].is_numeric() {
                            i += 1;
                        }
                        let pair_end = i;
                        if depth > 4 {
                            // explode
                            if debug { println!("\tpair from {}..{} explodes!", pair_start-1, pair_end+1); }
                            
                            let pair_str: String = chars[pair_start..=pair_end].iter().collect();
                            let (n1, n2) = pair_str.split_once(",").unwrap();
                            let left_num = i32::from_str_radix(n1, 10).unwrap();
                            let right_num = i32::from_str_radix(n2, 10).unwrap();
        
                            if debug { 
                                println!("\t\tleft num is {}", left_num);
                                println!("\t\tright num is {}", right_num);
                            }
        
                            let mut left_side: String = chars[0..pair_start-1].iter().collect();
                            let mut right_side: String = chars[pair_end+2..].iter().collect();
                            
                            if debug { 
                                println!("\t\tleft side is {}", &left_side);
                                println!("\t\tright side is {}", &right_side);
                            }
        
                            let left_digit_to_update: Option<usize> = left_side.rfind(char::is_numeric);
                            if left_digit_to_update.is_some() {
                                let lde = left_digit_to_update.unwrap();
                                let mut lds = lde;
                                while left_side.chars().nth(lds - 1).unwrap().is_numeric() {
                                    lds -= 1;
                                }
                                if debug { println!("\t\tfound left side number from {}..{}", lds, lde); }
        
                                let left_side_num_str: String = left_side.as_str()[lds..=lde].to_string();
                                let left_side_num: i32 = i32::from_str_radix(&left_side_num_str, 10).unwrap();
                                if debug { println!("\t\tthat number is {}", left_side_num); }
        
                                let p1 = left_side.as_str()[0..lds].to_string();
                                let p2 = left_side.as_str()[lde+1..].to_string();
                                if debug { println!("\t\tjamming {} between {} and {}", left_side_num + left_num, &p1, &p2); }
        
                                left_side = vec![p1, (left_side_num + left_num).to_string(), p2].join("");
                            }
                            if debug { println!("\t\tleft side is now {}", &left_side); }
        
                            let right_digit_to_update: Option<usize> = right_side.find(char::is_numeric);
                            if right_digit_to_update.is_some() {
                                let rds = right_digit_to_update.unwrap();
                                let mut rde = rds;
                                while right_side.chars().nth(rde + 1).unwrap().is_numeric() {
                                    rde += 1;
                                }
                                if debug { println!("\t\tfound right side number from {}..{}", rds, rde); }
        
                                let right_side_num_str: String = right_side.as_str()[rds..=rde].to_string();
                                let right_side_num: i32 = i32::from_str_radix(&right_side_num_str, 10).unwrap();
                                if debug { println!("\t\tthat number is {}", right_side_num); }
        
                                let p1 = right_side.as_str()[0..rds].to_string();
                                let p2 = right_side.as_str()[rde+1..].to_string();
                                if debug { println!("\t\tjamming {} between {} and {}", right_side_num + right_num, &p1, &p2); }
        
                                right_side = vec![p1, (right_side_num + right_num).to_string(), p2].join("");
                            }
                            if debug { println!("\t\tright side is now {}", &right_side); }
        
                            let mut new_snailfish = left_side;
                            new_snailfish.push_str("0");
                            new_snailfish.push_str(&right_side);
                            if debug { println!("\t\tresult is {}", &new_snailfish); }

                            // reset
                            chars = new_snailfish.chars().collect();
                            i = 0;
                            depth = 0;
                            continue;
                        }
                    }
                }
            }

            if i + 1 == chars.len() {
                if debug { println!("\tafter explodes, snailfish is {}", chars.iter().collect::<String>()); }
                // reset
                i = 0;
                depth = 0;
                looking_for_explosions = false;
            }
        } else {
            // splits
            if ch.is_numeric() {
                let num_start = i;
                while chars[i+1].is_numeric() {
                    i += 1;
                }
                let num_str: String = chars[num_start..=i].iter().collect();
                let num = i32::from_str_radix(&num_str, 10).unwrap();
                if num >= 10 {
                    // split
                    if debug { println!("\tnumber {} from {}..{} splits!", num, num_start, i); }
                    
                    let left_num = num / 2;
                    let right_num = (num + 1) / 2;
                    if debug { println!("\t\tleft num is {}", left_num); }
                    if debug { println!("\t\tright num is {}", right_num); }

                    let left_side: String = chars[0..num_start].iter().collect();
                    let right_side: String = chars[i+1..].iter().collect();

                    if debug { println!("\t\tleft side is {}", &left_side); }
                    if debug { println!("\t\tright side is {}", &right_side); }

                    let new_pair = format!("[{},{}]", &left_num, &right_num);

                    let mut new_snailfish = left_side;
                    new_snailfish.push_str(&new_pair);
                    new_snailfish.push_str(&right_side);
                    if debug { println!("\t\tresult is {}", &new_snailfish); }

                    // reset
                    chars = new_snailfish.chars().collect();
                    i = 0;
                    depth = 0;
                    continue;
                }
            }
        }

        i += 1;
    }

    chars.iter().collect()
}
