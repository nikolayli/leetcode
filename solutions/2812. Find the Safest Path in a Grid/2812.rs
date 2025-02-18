// Time complexity: O(n^2)
// Space complexity: O(n^2)
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let bound_check = |r: i32, c: i32| -> bool { r >= 0 && c >= 0 && r < n && c < n };

        let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i as usize][j as usize] != 0 {
                    q.push_back((i, j, 1));
                }
                grid[i as usize][j as usize] -= 1;
            }
        }

        while !q.is_empty() {
            let (i, j, safety) = q.pop_front().unwrap();
            let expand = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
            for (r, c) in expand.iter() {
                if bound_check(*r, *c) && grid[*r as usize][*c as usize] == -1 {
                    grid[*r as usize][*c as usize] = safety;
                    q.push_back((*r, *c, safety + 1));
                }
            }
        }

        let mut min_safety = grid[0][0];
        q.push_back((0, 0, grid[0][0]));

        while !q.is_empty() {
            let (i, j, safety) = q.pop_front().unwrap();
            min_safety = min_safety.min(safety);
            if i == n - 1 && j == n - 1 {
                break;
            }

            let expand = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
            for (r, c) in expand.iter() {
                if bound_check(*r, *c) && grid[*r as usize][*c as usize] != -1 {
                    let safety = grid[*r as usize][*c as usize];
                    grid[*r as usize][*c as usize] = -1;
                    if safety < min_safety {
                        q.push_back((*r, *c, safety));
                    } else {
                        q.push_front((*r, *c, safety));
                    }
                }
            }
        }

        min_safety
    }
}
