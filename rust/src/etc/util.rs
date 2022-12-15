use std::collections::HashSet;
use core::hash::Hash;

#[derive(Copy, Clone)]
pub struct Pair {
    pub start: i32,
    pub end: i32,
}

impl Pair {
    pub fn new(start: &str, end: &str) -> Self {
        Self {
            start: start.parse::<i32>().unwrap(),
            end: end.parse::<i32>().unwrap(),
        }
    }
}

pub fn has_unique_elements<T>(iter: T) -> bool 
    where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
