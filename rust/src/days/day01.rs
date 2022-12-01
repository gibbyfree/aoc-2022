use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// split into line groups, for each group, sum lines

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("/home/gib/aoc/input/1.in");
    let mut output = raw_input.split("\n\n")
        .map(|s| s.lines().map(|i| i.parse::<usize>().unwrap()).sum::<usize>())
        .collect::<Vec<usize>>();
    output.sort();
    let sol = output.pop().unwrap() as i64;
    let sol2 = sol + (output.pop().unwrap() + output.pop().unwrap()) as i64;

    (Solution::I64(sol), Solution::I64(sol2))
}