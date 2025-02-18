// Time complexity: O(mn)
// Space complexity: O(1)
impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if j < n - 1 && grid[i][j] == grid[i][j + 1] {
                    return false;
                }
                if i < m - 1 && grid[i][j] != grid[i + 1][j] {
                    return false;
                }
            }
        }

        true
    }
}
