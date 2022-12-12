use crate::{Solution, SolutionPair};
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////
/// basically just a dumb approach
/// the rucksack struct is pointless

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/3.in");
    let priority = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let second_input = raw_input.clone();
    let output = raw_input
        .split("\n")
        .map(|s| {
            let (first, second) = s.split_at(s.len() / 2);
            return Rucksack::new(first.to_string(), second.to_string());
        })
        .collect::<Vec<Rucksack>>();
    let sum: i32 = output
        .iter()
        .map(|r| {
            let item = find_common(r.first.clone(), r.second.clone());
            return priority.find(item).unwrap() as i32;
        })
        .sum();

    // sol2
    let lines: Vec<&str> = second_input.split("\n").collect();
    let mut second_sum = 0;
    for i in (0..lines.len()).step_by(3) {
        let common = find_common_badge(
            lines.get(i).unwrap().to_string(),
            lines.get(i + 1).unwrap().to_string(),
            lines.get(i + 2).unwrap().to_string(),
        );
        second_sum = second_sum + (priority.find(common).unwrap() as i32); 
    }

    (Solution::I32(sum), Solution::I32(second_sum))
}

pub fn find_common(first: String, second: String) -> char {
    let mut score_set = HashSet::new();
    for c in first.chars() {
        score_set.insert(c);
    }
    for c in second.chars() {
        if score_set.contains(&c) {
            return c;
        }
    }

    ' '
}

pub fn find_common_badge(first: String, second: String, third: String) -> char {
    let mut score_set: HashSet<char> = HashSet::new();
    let mut second_score_set: HashSet<char> = HashSet::new();
    for c in first.chars() {
        score_set.insert(c);
    }
    for c in second.chars() {
        second_score_set.insert(c);
    }
    for c in third.chars() {
        if score_set.contains(&c) && second_score_set.contains(&c) {
            return c;
        }
    }

    ' '
}
pub struct Rucksack {
    pub first: String,
    pub second: String,
}

impl Rucksack {
    pub fn new(first: String, second: String) -> Self {
        Self { first, second }
    }
}