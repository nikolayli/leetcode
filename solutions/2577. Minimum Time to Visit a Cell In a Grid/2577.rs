// Time complexity: O(mnlog(mn))
// Space complexity: O(mn)
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let m = grid.len();
        let n = grid[0].len();
        let mut min_heap = BinaryHeap::new();
        let mut seen = HashSet::new();

        min_heap.push(Reverse((0, 0, 0)));
        seen.insert((0, 0));

        while let Some(Reverse((time, i, j))) = min_heap.pop() {
            if i == m - 1 && j == n - 1 {
                return time;
            }
            for &(dx, dy) in &dirs {
                let x = i as isize + dx;
                let y = j as isize + dy;
                if x < 0 || x >= m as isize || y < 0 || y >= n as isize {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if seen.contains(&(x, y)) {
                    continue;
                }
                let extra_wait = if (grid[x][y] - time) % 2 == 0 { 1 } else { 0 };
                let next_time = std::cmp::max(time + 1, grid[x][y] + extra_wait);
                min_heap.push(Reverse((next_time, x, y)));
                seen.insert((x, y));
            }
        }

        -1
    }
}
