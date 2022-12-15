use std::collections::HashSet;

use crate::{etc::Point, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// pretty straight-forward problem once i understood how the falling sand moved ((1,1) being one move, not two)
/// i could potentially improve perf by stripping the repeating lines of input before parsing them.
/// other than that, yeah, the logic here is mostly similar to day9+12

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/14.in");
    let rocks: Vec<Vec<Point>> = raw_input
        .split("\n")
        .map(|line| {
            let points = line.split("->");
            points
                .map(|p| {
                    let parts = p
                        .split(",")
                        .map(|n| n.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    return Point::new(*parts.get(0).unwrap(), *parts.get(1).unwrap());
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let limit = &rocks.iter().flatten().max_by_key(|p| p.y).unwrap().y + 1;
    let cave: HashSet<Point> = build_cave(rocks);
    let clone_cave = cave.clone();

    /* let mut cave_drawing = [['.'; 500]; 600];
    for point in cave {
        cave_drawing[point.x as usize][point.y as usize] = '#';
    }
    for row in cave_drawing {
        println!("{:?}", row);
    } */

    (
        Solution::I32(solve_cave(cave, limit, false)),
        Solution::I32(solve_cave(clone_cave, limit + 2, true)),
    )
}

pub fn solve_cave(mut cave: HashSet<Point>, limit: i32, has_floor: bool) -> i32 {
    let mut output = 0;
    let floor = limit - 1;
    let drop_directions = [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)];

    loop {
        let mut sand_spout = Point::new(500, 0);
        if (has_floor && (cave.contains(&sand_spout) || sand_spout.y == floor))
            || (!has_floor && cave.contains(&sand_spout))
        {
            return output;
        }

        'inner: loop {
            let mut new_sand: Option<Point> = None;

            'dirs: for direction in drop_directions {
                let dropped_sand =
                    Point::new(sand_spout.x + direction.x, sand_spout.y + direction.y);

                if sand_spout.y == limit {
                    return output;
                }

                if (!has_floor && !cave.contains(&dropped_sand))
                    || (has_floor && (!cave.contains(&dropped_sand) && dropped_sand.y != floor))
                {
                    // there is not a rock here (nor is there a floor here)
                    new_sand = Some(dropped_sand);
                    break 'dirs;
                }
            }

            if let None = new_sand {
                cave.insert(sand_spout);
                output += 1;
                break 'inner;
            }

            sand_spout = new_sand.unwrap();
        }
    }
}

pub fn build_cave(rocks: Vec<Vec<Point>>) -> HashSet<Point> {
    let mut cave: HashSet<Point> = HashSet::new();

    for rock_list in rocks {
        // draw each rock
        for i in 0..rock_list.len() - 1 {
            let origin = rock_list.get(i).unwrap();
            let distance = Point::new(
                rock_list[i + 1].x - rock_list[i].x,
                rock_list[i + 1].y - rock_list[i].y,
            );

            // fill whatever is between these two points
            if distance.x > 0 {
                for j in origin.x..=(origin.x + distance.x) {
                    cave.insert(Point::new(j, origin.y));
                }
            } else if distance.x < 0 {
                for k in (origin.x + distance.x)..=origin.x {
                    cave.insert(Point::new(k, origin.y));
                }
            } else if distance.y > 0 {
                for l in origin.y..=(origin.y + distance.y) {
                    cave.insert(Point::new(origin.x, l));
                }
            } else if distance.y < 0 {
                for m in (origin.y + distance.y)..=origin.y {
                    cave.insert(Point::new(origin.x, m));
                }
            }
        }
    }

    cave
}
