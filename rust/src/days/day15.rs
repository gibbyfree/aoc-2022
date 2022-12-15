use std::{collections::HashSet, ops::Range};

use crate::{etc::Point, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// the parsing here is absolutely insane (in a bad way)
/// this does not work at all currently :)

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("../../../input/15.in");
    let sensors: Vec<Sensor> = raw_input
        .split("\n")
        .map(|line| {
            let s_b = line.split(":").collect::<Vec<&str>>();
            let s = *s_b.get(0).unwrap();
            let b = *s_b.get(1).unwrap();
            let s_parts = s.split(',').collect::<Vec<&str>>();
            let s_pt = Point::from_str(
                s_parts
                    .get(0)
                    .unwrap()
                    .split("=")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap(),
                s_parts
                    .get(1)
                    .unwrap()
                    .split("=")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap(),
            );

            let b_parts = b.split(',').collect::<Vec<&str>>();
            let b_pt = Point::from_str(
                b_parts
                    .get(0)
                    .unwrap()
                    .split("=")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap(),
                b_parts
                    .get(1)
                    .unwrap()
                    .split("=")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap(),
            );
            Sensor::new(s_pt, b_pt)
        })
        .collect::<Vec<Sensor>>();

    //println!("sensors be like {:?}", sensors);

    let covered_points = points_excluded_on_row(2000000, &sensors);
    let beacons = count_beacons_on_row(2000000, &sensors);

    (Solution::I32(covered_points - beacons), Solution::I64(0))
}

pub fn count_beacons_on_row(row: i32, sensor: &Vec<Sensor>) -> i32 {
    sensor
        .iter()
        .filter(|s| s.nearest_beacon.x == row)
        .map(|ss| ss.nearest_beacon.y)
        .collect::<HashSet<i32>>()
        .len() as i32
}

pub fn points_excluded_on_row(row: i32, sensor: &Vec<Sensor>) -> i32 {
    let mut ranges = sensor
        .iter()
        .filter_map(|s| s.points_excluded(row))
        .collect::<Vec<Range<i32>>>();
    ranges.sort_unstable_by_key(|r| r.start);

    // merge overlapping ranges
    let mut merged_ranges = vec![ranges[0].clone()];
    for r in &ranges[1..] {
        let prev_idx = merged_ranges.len() - 1;
        let prev = &merged_ranges[prev_idx];
        if r.start <= prev.end || prev.end + 1 == r.start {
            if r.end > prev.end {
                let new_r = merged_ranges[prev_idx].start..r.end;
                merged_ranges[prev_idx] = new_r;
            }
        } else {
            merged_ranges.push(r.clone());
        }
    }

    merged_ranges.iter().map(|r| r.len() as i32).sum()
}

#[derive(Debug)]
pub struct Sensor {
    pub position: Point,
    pub nearest_beacon: Point,
    pub dtb: u32, // dtb == Distance-to-Beacon
}

impl Sensor {
    pub fn new(position: Point, nearest_beacon: Point) -> Self {
        Self {
            position,
            nearest_beacon,
            dtb: get_manhattan_distance(position, nearest_beacon),
        }
    }

    // "if a sensor detects a beacon, you know there are no other beacons that close or closer to that sensor"
    pub fn points_excluded(&self, y: i32) -> Option<Range<i32>> {
        let between = self.dtb as i32 - (self.position.y - y).abs();
        if between < 0 {
            None
        } else {
            Some(self.position.y - between..self.position.y + between + 1)
        }
    }
}

// |x1 - x2| + |y1 - y2|
pub fn get_manhattan_distance(s: Point, b: Point) -> u32 {
    s.x.abs_diff(b.x) + s.y.abs_diff(b.y)
}
