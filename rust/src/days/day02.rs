use crate::{Solution, SolutionPair};
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////
/// part 1: normalize to same 3 chars for each side, judge w/ lookup
/// part 2: same, but transform my shape depending on desired outcome

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/2.in");
    let output: Vec<Vec<&str>> = raw_input
        .split("\n")
        .map(|r| r.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let for_sol2 = output.clone(); 
    let sol = output.into_iter().map(|r| judge_round(r, false)).sum::<i32>();
    let sol2 = for_sol2.into_iter().map(|r| judge_round(r, true)).sum::<i32>();

    (Solution::I32(sol), Solution::I32(sol2))
}

fn judge_round(round: Vec<&str>, shapeless: bool) -> i32 {
    // really hate creating these for each round, but rust hates const hashmaps
    // impl with a massive match would probably be more performant
    // it's ok :)
    let normalize = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let shapes = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    
    let opp = *normalize.get(round.get(0).unwrap()).unwrap();
    let mut me =*round.get(1).unwrap();

    if shapeless {
        match me {
            "Y" => me = opp,
            "X" => me = find_shape(opp, false),
            "Z" => me = find_shape(opp, true),
            _ => (),
        }
    }

    let shape_score = shapes.get(me).unwrap();
    if opp == me {
        return shape_score + 3;
    }
    match (opp, me) {
        ("X", "Z") | ("Y", "X") | ("Z", "Y") => return *shape_score,
        ("X", "Y") | ("Y", "Z") | ("Z", "X") => return shape_score + 6,
        (_, _) => return 0,
    }
}

fn find_shape(opp: &str, win: bool) -> &str {
    match (opp, win) {
            ("X", true) => return "Y",
            ("Y", true) => return "Z",
            ("Z", true) => return "X",
            ("X", false) => return "Z",
            ("Y", false) => return "X",
            ("Z", false) => return "Y",
            _ => return "",
        } 
    }
