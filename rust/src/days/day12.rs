use std::collections::{HashSet, VecDeque};

use crate::{etc::Point, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// translate input to 2d vec w/ chars translated to their num value
/// find path with basic BFS -- idk why others used djikstra's
/// i'm quite happy with this aside from all the usize casting. but i don't wanna refactor my other solution
/// that also uses Point lol.

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/12.in");
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let mut end = Point::new(0, 0);
    let mut start = Point::new(0, 0);
    let map: Vec<Vec<i32>> = raw_input
        .split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start.x = i as i32;
                        start.y = j as i32;
                        return alpha.find('a').unwrap() as i32;
                    } else if c == 'E' {
                        end.x = i as i32;
                        end.y = j as i32;
                        return alpha.find('z').unwrap() as i32;
                    }
                    return alpha.find(c).unwrap() as i32;
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (
        Solution::I32(find_shortest_path(&map, start, end, false)),
        Solution::I32(find_shortest_path(&map, start, end, true)),
    )
}

pub fn find_shortest_path(map: &Vec<Vec<i32>>, start: Point, end: Point, only_a: bool) -> i32 {
    let mut q: VecDeque<Point> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut output = 0;
    // insert end point -- start from E and go backwards
    q.push_back(end);

    'main: while !q.is_empty() {
        for _ in 0..q.len() {
            let p = q.pop_front().unwrap();
            let val_at_p = *map.get(p.x as usize).unwrap().get(p.y as usize).unwrap();
            if only_a {
                if val_at_p == 0 {
                    // found a valid 'a' starting point
                    break 'main;
                }
            } else {
                if p.x == start.x && p.y == start.y {
                    // found S
                    break 'main;
                }
            }
            // check left paths
            if p.y > 0 {
                let adj = *map
                    .get(p.x as usize)
                    .unwrap()
                    .get((p.y - 1) as usize)
                    .unwrap();
                if -adj + val_at_p <= 1 {
                    let adj_p = Point::new(p.x, p.y - 1);
                    if visited.insert(adj_p) {
                        q.push_back(adj_p);
                    }
                }
            }
            // check right paths
            if p.y < (map.get(0).unwrap().len() - 1) as i32 {
                let adj = *map
                    .get(p.x as usize)
                    .unwrap()
                    .get((p.y + 1) as usize)
                    .unwrap();
                if -adj + val_at_p <= 1 {
                    let adj_p = Point::new(p.x, p.y + 1);
                    if visited.insert(adj_p) {
                        q.push_back(adj_p);
                    }
                }
            }
            // check up paths
            if p.x > 0 {
                let adj = *map
                    .get((p.x - 1) as usize)
                    .unwrap()
                    .get(p.y as usize)
                    .unwrap();
                if -adj + val_at_p <= 1 {
                    let adj_p = Point::new(p.x - 1, p.y);
                    if visited.insert(adj_p) {
                        q.push_back(adj_p);
                    }
                }
            }
            // check down paths
            if p.x < (map.len() - 1) as i32 {
                let adj = *map
                    .get((p.x + 1) as usize)
                    .unwrap()
                    .get(p.y as usize)
                    .unwrap();
                if -adj + val_at_p <= 1 {
                    let adj_p = Point::new(p.x + 1, p.y);
                    if visited.insert(adj_p) {
                        q.push_back(adj_p);
                    }
                }
            }
        }
        output += 1;
    }
    output
}
