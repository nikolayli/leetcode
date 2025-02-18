// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let m = row_sum.len();
        let n = col_sum.len();
        let mut ans = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                ans[i][j] = row_sum[i].min(col_sum[j]);
                row_sum[i] -= ans[i][j];
                col_sum[j] -= ans[i][j];
            }
        }

        ans
    }
}
