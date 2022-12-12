use std::collections::VecDeque;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// this is the most fun aoc problem i've ever done!
/// use queue to process commands independent of cycle. 
/// add a new pixel before each cycle increment. 
/// collapse string vec to conform to the screen size.

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/10.in");
    let imp_cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];

    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut cycle = 1;
    let mut reg = 1;
    let mut screen: Vec<String> = Vec::new();
    screen.push("🟩".to_string());

    let mut commands = raw_input.split("\n").collect::<VecDeque<&str>>();
    let mut current = commands.pop_front();

    while !commands.is_empty() && !current.is_none() {
        let parsed_current = current.unwrap();
        if Some(imp_cycles.iter().any(|&x| x == cycle)) == Some(true) {
            signal_strengths.push(reg * cycle);
        }

        screen.push(get_pixel(cycle, reg));
        cycle = cycle + 1;

        if parsed_current.starts_with("a") {
            let val = parsed_current
                .split(' ')
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            if imp_cycle_between(cycle, &imp_cycles) {
                signal_strengths.push(reg * (cycle + 1));
            }

            screen.push(get_pixel(cycle, reg));
            cycle = cycle + 1;
            reg = reg + val;
        } 
        current = commands.pop_front();
    }

    let screen_str = screen.join("");
    let screen_pic = screen_str
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 40 == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>();
    let output_screen: String = "\n".to_owned() + &screen_pic;
    (
        Solution::I32(signal_strengths.iter().sum()),
        Solution::Str(output_screen),
    )
}

pub fn imp_cycle_between(current: i32, imp_cycles: &[i32]) -> bool {
    Some(imp_cycles.iter().any(|&x| x == current + 1)) == Some(true)
}

// pixel is lit if ONE OF the sprite's pixels is currently being drawn
// sprite's pixel is being drawn if the register and cycle val's delta is within sprite width
pub fn get_pixel(cycle: i32, reg: i32) -> String {
    let draw_idx = (cycle - 1) % 40; // draw placement on this line
    if draw_idx.abs_diff(reg) <= 3 / 2 {
        "🟩".to_string()
    } else {
        "🟥".to_string()
    }
}
