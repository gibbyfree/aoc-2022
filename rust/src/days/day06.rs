use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use core::hash::Hash;

///////////////////////////////////////////////////////////////////////////////
/// use .windows() and hashset
/// the +4 and +14 was the hardest part :')

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("/home/gib/aoc/input/6.in").to_string();
    let input = raw_input.chars().collect::<Vec<char>>();
    let input_copy = input.clone();
    let sol = find_index_of_unique_window(4, input) + 4;
    let sol2 = find_index_of_unique_window(14, input_copy) + 14;

    (Solution::I32(sol), Solution::I32(sol2))
}

pub fn find_index_of_unique_window(size: usize, input: Vec<char>) -> i32 {
    for (i, window) in input.windows(size).enumerate() {
        if has_unique_elements(window) {
             return i as i32;
        }
    }
    0
}

// TODO move to util class
pub fn has_unique_elements<T>(iter: T) -> bool 
    where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}