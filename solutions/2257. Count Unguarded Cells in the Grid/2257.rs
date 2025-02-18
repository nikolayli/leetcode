// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut ans = 0;
        let mut grid = vec![vec![0; n]; m];
        let mut left = vec![vec![0; n]; m];
        let mut right = vec![vec![0; n]; m];
        let mut up = vec![vec![0; n]; m];
        let mut down = vec![vec![0; n]; m];

        for guard in guards {
            grid[guard[0] as usize][guard[1] as usize] = 'G' as i32;
        }

        for wall in walls {
            grid[wall[0] as usize][wall[1] as usize] = 'W' as i32;
        }

        for i in 0..m {
            let mut last_cell = 0;
            for j in 0..n {
                if grid[i][j] == 'G' as i32 || grid[i][j] == 'W' as i32 {
                    last_cell = grid[i][j];
                } else {
                    left[i][j] = last_cell;
                }
            }
            last_cell = 0;
            for j in (0..n).rev() {
                if grid[i][j] == 'G' as i32 || grid[i][j] == 'W' as i32 {
                    last_cell = grid[i][j];
                } else {
                    right[i][j] = last_cell;
                }
            }
        }

        for j in 0..n {
            let mut last_cell = 0;
            for i in 0..m {
                if grid[i][j] == 'G' as i32 || grid[i][j] == 'W' as i32 {
                    last_cell = grid[i][j];
                } else {
                    up[i][j] = last_cell;
                }
            }
            last_cell = 0;
            for i in (0..m).rev() {
                if grid[i][j] == 'G' as i32 || grid[i][j] == 'W' as i32 {
                    last_cell = grid[i][j];
                } else {
                    down[i][j] = last_cell;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0
                    && left[i][j] != 'G' as i32
                    && right[i][j] != 'G' as i32
                    && up[i][j] != 'G' as i32
                    && down[i][j] != 'G' as i32
                {
                    ans += 1;
                }
            }
        }

        ans
    }
}
