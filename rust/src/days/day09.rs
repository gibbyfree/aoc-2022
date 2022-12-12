use crate::{Solution, SolutionPair, etc::Point};
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////
/// simulate moves. no tricks here! hashset to count unique positions without a 'board'.
/// problem describes tail movement very weirdly, but x-y track delta of leader's x-y
/// simple aside from me taking 30 mins to realize up decrements y, not the other way around...
/// 
/// for some reason i have been very stubborn about processing solutions in parallel.
/// this is the first time i realize i should probably split solve() into functions so that solution 2 is 'cleaner'
/// i feel like its sloppy insertion into solution 1 is wrecking perf. part 1 ran in like .01 ms and now i'm pushing a full ms.

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/9.in");
    let mut current: Vec<Point> = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();
    // everything starts at (0, 0) on the board
    current.push(Point::new(0, 0)); // head
    current.push(Point::new(0, 0)); // tail
    visited.insert(Point::new(0, 0));

    let mut long_rope: Vec<Point> = Vec::new();
    let mut long_visited: HashSet<Point> = HashSet::new();
    for _ in 1..=10 {
        long_rope.push(Point::new(0, 0));
    }
    long_visited.insert(Point::new(0, 0));

    // process commands
    for line in raw_input.split("\n") {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let direction = *parts.get(0).unwrap();
        let count = parts.get(1).unwrap().parse::<i32>().unwrap();

        for _i in 0..count {
            // move head(s)
            let head = current.get(0).unwrap();
            // so new head? *jumps on skateboard*
            let mut new_head = Point::new(head.x, head.y);
            match direction {
                "L" => new_head.x = head.x - 1,
                "R" => new_head.x = head.x + 1,
                "U" => new_head.y = head.y - 1,
                "D" => new_head.y = head.y + 1,
                _ => (),
            }
            current[0] = new_head;
            long_rope[0] = new_head;

            // tail follows
            for j in 1..long_rope.len() {
                if j < 2 {
                    let distance = Point::new(
                        current[j - 1].x - current[j].x,
                        current[j - 1].y - current[j].y,
                    );
                    // move IF more than 2 units away in either direction
                    let mut new_tail = Point::new(current[j].x, current[j].y);
                    if distance.x.abs() == 2 || distance.y.abs() == 2 {
                        new_tail.x = new_tail.x + distance.x.signum();
                        new_tail.y = new_tail.y + distance.y.signum();
                    }

                    current[j] = new_tail;
                }
                let long_distance = Point::new(
                    long_rope[j - 1].x - long_rope[j].x,
                    long_rope[j - 1].y - long_rope[j].y,
                );
                let mut new_long_tail = Point::new(long_rope[j].x, long_rope[j].y);
                if long_distance.x.abs() == 2 || long_distance.y.abs() == 2 {
                    new_long_tail.x = new_long_tail.x + long_distance.x.signum();
                    new_long_tail.y = new_long_tail.y + long_distance.y.signum();
                }
                long_rope[j] = new_long_tail;
            }
            // add to visited AFTER ALL MOVES
            visited.insert(*current.get(1).unwrap());
            long_visited.insert(*long_rope.last().unwrap());
        }
    }

    (
        Solution::I64(visited.len() as i64),
        Solution::I64(long_visited.len() as i64),
    )
}
