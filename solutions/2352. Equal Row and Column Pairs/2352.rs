// Time complexity: O(n^3)
// Space complexity: O(1)
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..n {
                let mut k = 0;
                while k < n {
                    if grid[i][k] != grid[k][j] {
                        break;
                    }
                }
            }
        }
    }
}
