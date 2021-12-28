use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 18");

    assert_eq!("[[[[0,9],2],3],4]", reduce("[[[[[9,8],1],2],3],4]"));
    assert_eq!("[7,[6,[5,[7,0]]]]", reduce("[7,[6,[5,[4,[3,2]]]]]"));
    assert_eq!("[[6,[5,[7,0]]],3]", reduce("[[6,[5,[4,[3,2]]]],1]"));
    assert_eq!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", reduce("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));
    assert_eq!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", reduce("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
    assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", add_and_reduce("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"));

    assert_eq!(29, calc_magnitude("[9,1]"));
    assert_eq!(21, calc_magnitude("[1,9]"));
    assert_eq!(129, calc_magnitude("[[9,1],[1,9]]"));
    assert_eq!(143, calc_magnitude("[[1,2],[[3,4],5]]"));
    assert_eq!(1384, calc_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    assert_eq!(445, calc_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
    assert_eq!(791, calc_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
    assert_eq!(1137, calc_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    assert_eq!(3488, calc_magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));

    let sample_1 = aoc::read_input("input/day18-sample-1.txt");
    println!("sample 1 1 = {}", part1(&sample_1));
    println!("sample 1 2 = {}", part2(&sample_1));

    let sample_2 = aoc::read_input("input/day18-sample-2.txt");
    println!("sample 2 1 = {}", part1(&sample_2));
    println!("sample 2 2 = {}", part2(&sample_2));

    let real_input = aoc::read_input("input/day18.txt");
    println!("part 1 = {}", part1(&real_input));
    println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> i32 {
    let mut snailfish = Vec::from(input);
    while snailfish.len() > 1 {
        let mut new_snailfish = vec![add_and_reduce(&snailfish[0], &snailfish[1])];
        for a in &snailfish[2..] {
            new_snailfish.push(a.to_string());
        }
        snailfish = new_snailfish;
    }
    calc_magnitude(&snailfish[0])
}

fn part2(input: &[String]) -> i32 {
    let mut max_magnitude = 0i32;
    for (i, left) in input.iter().enumerate() {
        for (j, right) in input.iter().enumerate() {
            if i != j {
                max_magnitude = std::cmp::max(max_magnitude, calc_magnitude(&add_and_reduce(&left, &right)));
            }
        }
    }
    max_magnitude
}

fn calc_magnitude(snailfish: &str) -> i32 {
    if snailfish.chars().all(char::is_numeric) {
        return i32::from_str_radix(snailfish, 10).unwrap();
    }

    let mut magnitude = 0;
    let mut depth = 0;

    for (i, ch) in snailfish.chars().enumerate() {
        if ch == '[' {
            depth += 1;
        } else if ch == ']' {
            depth -= 1;
        } else if ch == ',' && depth == 1 {
            let left = &snailfish[1..i];
            let right = &snailfish[i+1..snailfish.len()-1];
            magnitude += 3 * calc_magnitude(left) + 2 * calc_magnitude(right)
        }
    }

    magnitude
}

fn add_and_reduce(left: &str, right: &str) -> String {
    let reduced = rrreduce(format!("[{},{}]", left, right));
    //println!("  {}\n+ {}\n= {}\n", left, right, &reduced);
    reduced
}

fn rrreduce(snailfish: String) -> String {
    let mut current = snailfish;
    loop {
        let reduced = reduce(&current);
        if reduced.eq(&current) {
            break;
        }
        current = reduced;
    }
    current
}

fn reduce(snailfish: &str) -> String {
    let debug = false;

    if debug {
        println!("reducing {}...", snailfish);
    }

    let chars: Vec<char> = snailfish.chars().collect();
    let mut i = 0usize;
    let mut depth = 0i32;

    // explodes
    while i < chars.len() {
        let ch = chars[i];
        
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

                        return new_snailfish;
                    }
                }
            }
        }

        i += 1;
    }

    // splits
    i = 0usize;
    while i < chars.len() {
        let ch = chars[i];
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

                return new_snailfish;
            }
        }

        i += 1;
    }

    snailfish.to_string()
}
