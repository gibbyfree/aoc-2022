use crate::{Solution, SolutionPair, etc::Pair};

///////////////////////////////////////////////////////////////////////////////
/// parse n check

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/4.in");
    let mut first_counter = 0;
    let mut second_counter = 0;
    let _proc = raw_input
        .split("\n")
        .map(|s| {
            let sections = s
                .split(",")
                .map(|p| {
                    let halves = p.split("-").collect::<Vec<&str>>();
                    return Pair::new(*halves.get(0).unwrap(), *halves.get(1).unwrap());
                })
                .collect::<Vec<Pair>>();
            if contain_each_other(*sections.get(0).unwrap(), *sections.get(1).unwrap()) {
                first_counter = first_counter + 1;
                second_counter = second_counter + 1;
                0
            } else if overlap_at_all(*sections.get(0).unwrap(), *sections.get(1).unwrap()) {
                second_counter = second_counter + 1;
                0
            } else {
                0
            }
        }).collect::<Vec<i32>>(); 

    (Solution::I32(first_counter), Solution::I32(second_counter))
}

pub fn overlap_at_all(a: Pair, b: Pair) -> bool {
    (b.start <= a.start && a.start <= b.end) || (a.start <= b.start && b.start <= a.end)
}

pub fn contain_each_other(a: Pair, b: Pair) -> bool {
    (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
}

