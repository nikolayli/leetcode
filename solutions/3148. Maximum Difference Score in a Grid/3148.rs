// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn max_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let k_max = 200_000;
        let mut ans = -k_max;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let prev_min = if i > 0 { grid[i - 1][j] } else { k_max }.min(if j > 0 {
                    grid[i][j - 1]
                } else {
                    k_max
                });
                ans = ans.max(grid[i][j] - prev_min);
                grid[i][j] = grid[i][j].min(prev_min);
            }
        }

        ans
    }
}
