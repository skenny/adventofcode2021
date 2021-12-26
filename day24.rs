use crate::aoc;
use std::collections::HashMap;

pub fn run() {
    println!("AOC 2021 - Day 24");

    let sample_1 = parse_input(&aoc::read_input("input/day24-sample-1.txt"));
    part1(&sample_1, &vec![1]);

    let sample_2 = parse_input(&aoc::read_input("input/day24-sample-2.txt"));
    part1(&sample_2, &vec![1, 2]);
}

fn part1(instructions: &Vec<Instruction>, input: &Vec<i32>) {
    let mut alu = ALU::new();
    alu.execute(instructions, input);
    println!("done; {:?}", alu);
}

fn part2(input: &[String]) -> usize {
    0
}

fn parse_input(input: &[String]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let op = parts.get(0).unwrap();
        let arg_1 = parts.get(1).unwrap();
        let arg_2 = parts.get(2).map(|v| v.to_string());
        instructions.push(Instruction { op: op.to_string(), arg_1: arg_1.to_string(), arg_2: arg_2 });
    }
    instructions
}

#[derive(Debug)]
struct Instruction {
    op: String,
    arg_1: String,
    arg_2: Option<String>
}

impl Instruction {
    fn execute(&mut self) {

    }
}

#[derive(Debug)]
struct ALU {
    vars: HashMap<String, i32>
}

impl ALU {
    fn new() -> ALU {
        let mut vars: HashMap<String, i32> = HashMap::new();
        vars.insert("w".to_string(), 0);
        vars.insert("x".to_string(), 0);
        vars.insert("y".to_string(), 0);
        vars.insert("z".to_string(), 0);
        ALU {
            vars: vars
        }
    }

    fn execute(&mut self, instructions: &Vec<Instruction>, input: &Vec<i32>) {
        let mut input_pointer = 0;
        for instr in instructions {
            match instr.op.as_str() {
                "inp" => { self.inp(&instr.arg_1, input[input_pointer]); input_pointer += 1; },
                "add" => self.add(&instr.arg_1, &instr.arg_2.as_ref().unwrap()),
                "mul" => self.mul(&instr.arg_1, &instr.arg_2.as_ref().unwrap()),
                "div" => self.div(&instr.arg_1, &instr.arg_2.as_ref().unwrap()),
                "mod" => self.modulo(&instr.arg_1, &instr.arg_2.as_ref().unwrap()),
                "eql" => self.eql(&instr.arg_1, &instr.arg_2.as_ref().unwrap()),
                _ => {}
            }
        }
    }

    fn get(&self, arg: &str) -> i32 {
        let maybe_int = arg.parse::<i32>();
        if maybe_int.is_ok() {
            return maybe_int.unwrap();
        }
        self.vars[arg]
    }

    // inp a - Read an input value and write it to variable a.
    fn inp(&mut self, arg: &str, input: i32) {
        self.vars.insert(arg.to_string(), input);
    }

    // add a b - Add the value of a to the value of b, then store the result in variable a.
    fn add(&mut self, arg_1: &str, arg_2: &str) {
        self.vars.insert(arg_1.to_string(), self.vars[arg_1] + self.get(arg_2));
    }

    // mul a b - Multiply the value of a by the value of b, then store the result in variable a.
    fn mul(&mut self, arg_1: &str, arg_2: &str) {
        self.vars.insert(arg_1.to_string(), self.vars[arg_1] * self.get(arg_2));
    }

    // div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
    fn div(&mut self, arg_1: &str, arg_2: &str) {
        self.vars.insert(arg_1.to_string(), self.vars[arg_1] / self.get(arg_2));
    }

    // mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
    fn modulo(&mut self, arg_1: &str, arg_2: &str) {
        self.vars.insert(arg_1.to_string(), self.vars[arg_1] % self.get(arg_2));
    }

    // eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
    fn eql(&mut self, arg_1: &str, arg_2: &str) {
        self.vars.insert(arg_1.to_string(), if self.vars[arg_1] == self.vars[arg_2] { 1 } else { 0 });
    }
}
