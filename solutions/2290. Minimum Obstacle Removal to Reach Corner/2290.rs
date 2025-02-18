// Time complexity: O(mn)
// Space complexity: O(mn)

use std::collections::VecDeque;

impl Solution {
    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dist = vec![vec![i32::MAX; n]; m];
        dist[0][0] = 0;
        let mut deque = VecDeque::new();
        deque.push_back((0, 0, 0));

        while let Some((obstacles, row, col)) = deque.pop_front() {
            for &(dr, dc) in &Self::DIRS {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                if new_row >= 0 && new_row < m as i32 && new_col >= 0 && new_col < n as i32 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if dist[new_row][new_col] == i32::MAX {
                        if grid[new_row][new_col] == 1 {
                            dist[new_row][new_col] = obstacles + 1;
                            deque.push_back((obstacles + 1, new_row, new_col));
                        } else {
                            dist[new_row][new_col] = obstacles;
                            deque.push_front((obstacles, new_row, new_col));
                        }
                    }
                }
            }
        }

        dist[m - 1][n - 1]
    }
}
