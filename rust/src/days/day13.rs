use crate::{Solution, SolutionPair};
use std::cmp::{min, Ordering};

///////////////////////////////////////////////////////////////////////////////
/// this one would've been super fun to do in typescript...
/// it was a journey. first i tried to do everything with bools, realized that i was misinterpreting the problem, changed to 1,-1,0
/// passed test input but failed my input because my parser assumed all numbers were single digit
/// then with p2 i reworked 1,-1,0 to use Ordering so that i could sort the packets directly

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/13.in");

    let pairs = raw_input
        .split("\n\n")
        .enumerate()
        .map(|(i, line)|{
            let parts = line.split("\n").collect::<Vec<&str>>();
            let first = parse(parts.get(0).unwrap());
            let second = parse(parts.get(1).unwrap());
            PacketPair { left: first, right: second, index: i + 1}
        })
        .collect::<Vec<PacketPair>>();

    let sol1: usize = pairs
        .iter()
        .map(|pair|{
            if compare_pairs(&pair.left, &pair.right) == Ordering::Less {
                return pair.index;
            }
            return 0;
        })
        .sum();

    let div1 = "[2]";
    let div2 = "[6]";
    let mut packets: Vec<PacketData> = Vec::new();

    for line in raw_input.split("\n") {
        if line.len() > 1 {
            packets.push(parse(line));
        }
    }
    packets.push(parse(div1));
    packets.push(parse(div2));
    packets.sort_by(|left, right| compare_pairs(left, right));
    let idx = packets.iter().position(|p| p == &parse(div1)).unwrap() + 1;
    let idx2 = packets.iter().position(|x| x == &parse(div2)).unwrap() + 1;
    
    (Solution::I64(sol1 as i64), Solution::I64((idx * idx2) as i64))
}

pub fn compare_pairs(left: &PacketData, right: &PacketData) -> Ordering {
    match (left, right) {
        (PacketData::PacketValue(lv), PacketData::PacketValue(rv)) => {
            if lv > rv {
                return Ordering::Greater;
            } else if lv == rv {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        }
        (PacketData::PacketList(ll), PacketData::PacketList(rl)) => {
            for i in 0..min(ll.len(), rl.len()) {
                let comparison = compare_pairs(&ll[i], &rl[i]);
                if comparison != Ordering::Equal {
                    return comparison;
                }
            }
            // left needs to be shorter
            if ll.len() < rl.len() {
                return Ordering::Less;
            } else if ll.len() == rl.len() {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        }
        (PacketData::PacketValue(lv), PacketData::PacketList(_)) => {
            let to_list = PacketData::PacketList(vec![PacketData::PacketValue(*lv)]);
            return compare_pairs(&to_list, right);
        }
        (PacketData::PacketList(_), PacketData::PacketValue(rv)) => {
            let to_list = PacketData::PacketList(vec![PacketData::PacketValue(*rv)]);
            return compare_pairs(left, &to_list);
        }
    }
}

pub fn parse(s: &str) -> PacketData {
    let ps = &s[1..s.len() -1]; // ignore first + last bracket
    let mut stack: Vec<(Vec<PacketData>, u32)> = Vec::new(); // holds content of prev lists

    let mut pl: Vec<PacketData> = Vec::new();
    let mut pv: u32 = 100; // arbitrary marking number
    let mut num_before = false; // multi-digit numbers 

    for c in ps.chars() {
        if c.is_numeric() {
            // list element
            if num_before {
                // concat onto previous pv.
                pv = (pv * 10) + c.to_digit(10).unwrap() as u32;
            } else {
                pv = c.to_digit(10).unwrap() as u32;
            }
            num_before = true;
        } else if c == ',' {
            // next c will be list element
            if pv != 100 {
                pl.push(PacketData::PacketValue(pv));
            }
            num_before = false;
        } else if c == '[' {
            // new list start
            stack.push((pl, pv));
            pl = Vec::new();
            pv = 100;
            num_before = false;
        } else if c == ']' {
            // list end
            if pv != 100 {
                pl.push(PacketData::PacketValue(pv));
            }
            let complete = PacketData::PacketList(pl);
            (pl, pv) = stack.pop().unwrap();
            pl.push(complete);
            num_before = false;
        } else {
            num_before = false;
        }
    }

    if pv != 100 {
        pl.push(PacketData::PacketValue(pv));
    }

    PacketData::PacketList(pl)
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum PacketData {
    PacketValue(u32),
    PacketList(Vec<PacketData>),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct PacketPair {
    pub left: PacketData,
    pub right: PacketData,
    pub index: usize,
}