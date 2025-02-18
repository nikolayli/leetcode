// Time complexity: O(mn)
// Space complexity: O(m+n)
impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp1 = vec![0; n];
        let mut dp2 = vec![0; m];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    dp1[i] += 1;
                    dp2[j] += 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    ans += (dp1[i] - 1) * (dp2[j] - 1);
                }
            }
        }

        ans
    }
}
