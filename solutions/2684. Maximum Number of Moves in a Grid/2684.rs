// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];

        for j in (0..n - 1).rev() {
            for i in 0..m {
                if grid[i][j + 1] > grid[i][j] {
                    dp[i][j] = dp[i][j].max(1 + dp[i][j + 1]);
                }
                if i > 0 && grid[i - 1][j + 1] > grid[i][j] {
                    dp[i][j] = dp[i][j].max(1 + dp[i - 1][j + 1]);
                }
                if i + 1 < m && grid[i + 1][j + 1] > grid[i][j] {
                    dp[i][j] = dp[i][j].max(1 + dp[i + 1][j + 1]);
                }
            }
        }

        dp.iter().map(|row| row[0]).max().unwrap_or(0)
    }
}
