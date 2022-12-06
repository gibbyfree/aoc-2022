use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// treat vecs like stacks

pub fn solve() -> SolutionPair {
    let mut input = include_str!("/home/gib/aoc/input/5.in")
        .split("\n\n")
        .collect::<Vec<&str>>();

    // parse out crates
    let mut stacks: Vec<Vec<char>> = vec![];
    for n in 1..10 {
        // pobodys nerfect xD
        stacks.push(Vec::new());
    }
    for line in input.get(0).unwrap().lines() {
        stacks.push(Vec::new());
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                // add to bottom of vec @ crate associated with this index
                let stack_idx = ((i / 4) as f32).trunc() as usize;
                stacks[stack_idx].push(c);
            }
        }
    }

    

    println!("{:?}", stacks);

    (Solution::I64(0), Solution::I64(0))
}