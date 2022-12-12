use std::collections::VecDeque;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// treat vecs like stacks
/// well, in the end i used vecdeque, but this was a mistake. vecdeque doesn't have extend or drain for part 2 :(

pub fn solve() -> SolutionPair {
    let input = include_str!("../../../input/5.in")
        .split("\n\n")
        .collect::<Vec<&str>>();

    // parse out crates
    let mut stacks: Vec<VecDeque<char>> = vec![];
    for _n in 1..10 {
        // pobodys nerfect xD
        stacks.push(VecDeque::new());
    }
    for line in input.get(0).unwrap().lines() {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                // add to bottom of vec @ crate associated with this index
                let stack_idx = ((i / 4) as f32).trunc() as usize;
                stacks[stack_idx].push_back(c);
            }
        }
    }

    let mut second_stacks = stacks.clone();

    // commands
    // quantity - source - dest
    let commands: Vec<Command> = input
        .get(1)
        .unwrap()
        .lines()
        .map(|c| {
            let words = c.split(' ').collect::<Vec<&str>>();
            return Command::new(
                *words.get(1).unwrap(),
                *words.get(3).unwrap(),
                *words.get(5).unwrap(),
            );
        })
        .collect::<Vec<Command>>();

    for command in commands {
        let mut tmp: VecDeque<char> = VecDeque::new(); // second solution uses temp stack to preserve order
        for _n in 0..command.quantity {
            let e = stacks[command.source - 1].pop_front();
            let second = second_stacks[command.source - 1].pop_front();
            if let Some(item) = e {
                stacks[command.dest - 1].push_front(item);
            }
            if let Some(item) = second {
                tmp.push_back(item);
            }
        }
        for c in tmp.iter().rev() {
            second_stacks[command.dest - 1].push_front(*c);
        }
    }

    (
        Solution::Str(read_top_crates(stacks)),
        Solution::Str(read_top_crates(second_stacks)),
    )
}

pub fn read_top_crates(stacks: Vec<VecDeque<char>>) -> String {
    let mut sol = "".to_string();
    for mut q in stacks {
        sol.push(q.pop_front().unwrap());
    }
    sol
}

pub struct Command {
    pub quantity: i32,
    pub source: usize,
    pub dest: usize,
}

impl Command {
    pub fn new(quantity: &str, source: &str, dest: &str) -> Self {
        Self {
            quantity: quantity.parse::<i32>().unwrap(),
            source: source.parse::<usize>().unwrap(),
            dest: dest.parse::<usize>().unwrap(),
        }
    }
}
