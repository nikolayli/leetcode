// Time complexity: O(mn)
// Space complexity: O(1)
impl Solution {
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i > 0 {
                    grid[i][j] += grid[i - 1][j];
                }
                if j > 0 {
                    grid[i][j] += grid[i][j - 1];
                }
                if i > 0 && j > 0 {
                    grid[i][j] -= grid[i - 1][j - 1];
                }

                if grid[i][j] <= k {
                    ans += 1;
                } else {
                    break;
                }
            }
        }

        ans
    }
}
