use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 17");

    let sample_input = aoc::read_input("input/day17-sample.txt");
    println!("sample 1 = {}", part1(&sample_input));
    //println!("sample 2 = {}", part2(&sample_input));

    let real_input = aoc::read_input("input/day17.txt");
    //println!("part 1 = {}", part1(&real_input));
    //println!("part 2 = {}", part2(&real_input));
}

fn part1(input: &[String]) -> usize {
    let mut probe = Probe::new();
    probe.launch((7, 2));
    probe.track_flight(10);
    0
}

fn part2(input: &[String]) -> usize {
    0
}

#[derive(Debug)]
struct Probe {
    position: (i32, i32),
    velocity: (i32, i32)
}

impl Probe {
    fn new() -> Probe {
        Probe {
            position: (0, 0),
            velocity: (0, 0)
        }
    }

    fn launch(&mut self, initial_velocity: (i32, i32)) {
        self.velocity = initial_velocity;
    }

    fn track_flight(&mut self, steps: i32) {
        for i in 0..steps {
            self.step();
            println!("after step {} -> {:?}", i, self);
        }
    }

    fn step(&mut self) {
        self.position = (self.position.0 + self.velocity.0, self.position.1 + self.velocity.1);
        self.velocity = (std::cmp::max(0, self.velocity.0 - 1), self.velocity.1 - 1);
    }
}
