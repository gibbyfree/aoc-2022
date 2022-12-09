use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// split into line groups, for each group, sum lines
/// sort and pop to get the greatest vals

pub fn solve() -> SolutionPair {
    // effectively a list of rows
    let tree_map: Vec<Vec<u32>> = include_str!("/home/gfree/aoc-2022/input/8.in")
        .split("\n")
        .map(|s| {
            let mut v: Vec<u32> = Vec::new();
            for c in s.chars() {
                v.push(c.to_digit(10).unwrap());
            }
            v
        })
        .collect::<Vec<Vec<u32>>>();

    let mut visible_sum = 0;
    let mut max_scenic = 0;
    for (x, row) in tree_map.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {

            let mut die = 0;
            let mut scenic_die = 0;
            let mut local_scenic = 0;
            let mut scenic_score = [0, 0, 0, 0];

            // check above
            for h in (0..x).rev() {
                if scenic_die == 1 && die == 1 {
                    break;
                }
                if tree_map[h][y] < *col && scenic_die == 0 {
                    local_scenic += 1;
                }
                if tree_map[h][y] == *col && scenic_die == 0 {
                    local_scenic += 1;
                    scenic_die += 1;
                }
                if tree_map[h][y] >= *col && die == 0 {
                    die += 1;
                    if scenic_die == 0 {
                        local_scenic += 1;
                        scenic_die += 1;
                    }
                }
            }
            scenic_score[0] = local_scenic;
            local_scenic = 0;
            // check below
            for i in (x + 1)..tree_map.len() {
                if scenic_die == 2 && die == 2 {
                    break;
                }
                if tree_map[i][y] < *col && scenic_die == 1 {
                    local_scenic += 1;
                }
                if tree_map[i][y] == *col && scenic_die == 1 {
                    local_scenic += 1;
                    scenic_die += 1;
                }
                if tree_map[i][y] >= *col && die == 1 {
                    die += 1;
                    if scenic_die == 1 {
                        local_scenic += 1;
                        scenic_die += 1;
                    }
                }
            }
            scenic_score[1] = local_scenic;
            local_scenic = 0;
            // check left
            for j in (0 .. y).rev() {
                if scenic_die == 3 && die == 3 {
                    break;
                }
                if tree_map[x][j] < *col && scenic_die == 2 {
                    local_scenic += 1;
                }
                if tree_map[x][j] == *col && scenic_die == 2 {
                    local_scenic += 1;
                    scenic_die += 1;
                }
                if tree_map[x][j] >= *col && die == 2 {
                    die += 1;
                    if scenic_die == 2 {
                        local_scenic += 1;
                        scenic_die += 1;
                    }
                }
            }
            scenic_score[2] = local_scenic;
            local_scenic = 0;
            // check right
            for k in (y + 1)..row.len() {
                if scenic_die == 4 && die == 4 {
                    break;
                }
                if tree_map[x][k] < *col && scenic_die == 3 {
                    local_scenic += 1;
                }
                if tree_map[x][k] == *col && scenic_die == 3 {
                    local_scenic += 1;
                    scenic_die += 1;
                }
                if tree_map[x][k] >= *col && die == 3 {
                    die += 1;
                    if scenic_die == 3 {
                        local_scenic += 1;
                        scenic_die += 1;
                    }
                }
            }
            scenic_score[3] = local_scenic;
            let scenic_total = scenic_score[0] * scenic_score[1] * scenic_score[2] * scenic_score[3];

            //if scenic_total > max_scenic { max_scenic = scenic_total; }
            println!("current scenic nums: local:{}, arr:{:?}, max:{}, total:{}", local_scenic, scenic_score, max_scenic, scenic_total);


            if (x == 0 && y == 0)
                || (x == tree_map.len() - 1 && y == 0)
                || (x == 0 && y == row.len() - 1)
                || (x == tree_map.len() - 1 && y == row.len() - 1)
                || (x == 0 || x == tree_map.len() - 1 || y == 0 || y == row.len() - 1)
                || die < 4
            {
                visible_sum += 1;
            }

            if scenic_total > max_scenic { 
                max_scenic = scenic_total; 
                println!("new max scenic: {}", max_scenic);
            }
        }
    }

    (Solution::I64(visible_sum), Solution::I64(max_scenic))
}
