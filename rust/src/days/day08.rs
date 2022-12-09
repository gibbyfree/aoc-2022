use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// this one took me the longest so far. it was a nightmare
/// 2d vec was a mistake. using iter was a mistake. should've just used slices and indices
/// part 2 instructions were super vague to me, and i struggled a lot to incorporate into my existing solution
/// this is accomplished in a psychotic way, using the 'die' vars as semaphore-like things to keep my sums clean
/// anyway i hated every second of it.

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
                if tree_map[h][y] >= *col {
                    if die == 0 {
                        die += 1;
                    }
                    if scenic_die == 0 {
                        local_scenic += 1;
                        scenic_die += 1;
                    }
                }
            }
            scenic_score[0] = local_scenic;
            local_scenic = 0;
            scenic_die = 1;
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
                if tree_map[i][y] >= *col {
                    if die == 1 {
                        die += 1;
                    }
                    if scenic_die == 1 {
                        local_scenic += 1;
                        scenic_die += 1;
                    }
                }
            }
            scenic_score[1] = local_scenic;
            local_scenic = 0;
            scenic_die = 2;
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
                if tree_map[x][j] >= *col {
                    if die == 2 {
                        die += 1;
                    }
                    if scenic_die == 2 {
                        local_scenic += 1;
                        scenic_die += 1;
                    }
                }
            }
            scenic_score[2] = local_scenic;
            local_scenic = 0;
            scenic_die = 3;
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
                if tree_map[x][k] >= *col {
                    if die == 3 {
                        die += 1;
                    }
                    if scenic_die == 3 {
                        local_scenic += 1;
                        scenic_die += 1;
                    }
                }
            }
            scenic_score[3] = local_scenic;
            let scenic_total = scenic_score[0] * scenic_score[1] * scenic_score[2] * scenic_score[3];

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
            }
        }
    }

    (Solution::I64(visible_sum), Solution::I64(max_scenic))
}
