// Time complexity: O((mn)^2)
// Space complexity: O(mn)
impl Solution {
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let m = grid.len();
        let n = grid[0].len();

        fn dfs(
            grid: &Vec<Vec<i32>>,
            i: usize,
            j: usize,
            seen: &mut Vec<(usize, usize)>,
            dirs: &[(i32, i32)],
            m: usize,
            n: usize,
        ) {
            seen.push((i, j));

            for &(dx, dy) in dirs {
                let x = (i as i32 + dx) as usize;
                let y = (j as i32 + dy) as usize;
                if x >= m || y >= n || grid[x][y] == 0 || seen.contains(&(x, y)) {
                    continue;
                }
                dfs(grid, x, y, seen, dirs, m, n);
            }
        }

        fn disconnected(grid: &Vec<Vec<i32>>, m: usize, n: usize, dirs: &[(i32, i32)]) -> bool {
            let mut islands_count = 0;
            let mut seen = Vec::new();

            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 0 || seen.contains(&(i, j)) {
                        continue;
                    }
                    if islands_count > 1 {
                        return true;
                    }
                    islands_count += 1;
                    dfs(grid, i, j, &mut seen, dirs, m, n);
                }
            }
            islands_count != 1
        }

        if disconnected(&grid, m, n, &dirs) {
            return 0;
        }

        let mut grid = grid.clone();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    if disconnected(&grid, m, n, &dirs) {
                        return 1;
                    }
                    grid[i][j] = 1;
                }
            }
        }

        2
    }
}
