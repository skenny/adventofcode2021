use crate::aoc;

use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    println!("AOC 2021 - Day 18");

    let sample_input = aoc::read_input("input/day18-sample.txt");
    part1(&sample_input);

    let real_input = aoc::read_input("input/day18.txt");
    part1(&real_input);
}

fn part1(input: &[String]) {
    println!("part 1 = {}", 0);
}

fn parse_input(input: &[String]) -> Vec<Snailfish> {
    input.iter().map(|line| parse_snailfish(&line)).collect()
}

fn parse_snailfish(input: &str) -> Snailfish {
    let fishy = Snailfish::number(None, 5);
    fishy.unwrap().ge // get_mut() = Snailfish::number(None, 10);
}

struct SnailfishNode {
    val: Option<i32>,
    parent: Snailfish,
    left: Snailfish,
    right: Snailfish,
}

type Snailfish = Option<Rc<RefCell<SnailfishNode>>>;

trait SnailfishMaker {
    fn pair(parent: Snailfish, left: Snailfish, right: Snailfish) -> Snailfish {
        Some(Rc::new(RefCell::new(SnailfishNode { 
            val: None, 
            parent: parent, 
            left: left, 
            right: right 
        })))
    }
    fn number(parent: Snailfish, val: i32) -> Snailfish {
        Some(Rc::new(RefCell::new(SnailfishNode {
            val: Some(val),
            parent: parent,
            left: None,
            right: None,
        })))
    }
}

impl SnailfishMaker for Snailfish {}

// struct Snailfish<T> {
//     parent: Option<Box<Snailfish<T>>>,
//     left: Option<Box<Snailfish<T>>>,
//     right: Option<Box<Snailfish<T>>>,
//     value: Option<T>
// }

// impl<T> Snailfish<T> {

//     fn as_number(parent: Option<Snailfish<T>>, value: T) -> Snailfish<T> {
//         Snailfish {
//             parent: if parent.is_some() { Some(Box::new(parent.unwrap())) } else { None },
//             left: None,
//             right: None,
//             value: Some(value)
//         }
//     }

//     fn as_pair(parent: Option<Snailfish<T>>, left: Snailfish<T>, right: Snailfish<T>) -> Snailfish<T> {
//         Snailfish {
//             parent: if parent.is_some() { Some(Box::new(parent.unwrap())) } else { None },
//             left: Some(Box::new(left)),
//             right: Some(Box::new(right)),
//             value: None
//         }
//     }

//     fn is_pair(&self) -> bool {
//         self.value.is_none()
//     }

//     fn is_regular_number(&self) -> bool {
//         self.value.is_some()
//     }

//     fn explode(&self) {
//         // TODO
//     }

//     fn update_left_regular_number(&mut self, value: T) {
//         let mut node = &self.parent;
//         loop {
//             if node.is_none() {
//                 break;
//             }
//             if self.right.is_some() {
//                 let r = self.right.as_mut();
//                 let r2 = r.unwrap();
//                 if r2.is_regular_number() {
//                     r2.value = Some(value);
//                 }
//             }
//             let node = &self.parent;
//         }
//     }

// }
