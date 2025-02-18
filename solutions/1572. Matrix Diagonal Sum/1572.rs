// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut ans = 0;

        for i in 0..n {
            ans += mat[i][i];
            ans += mat[n - 1 - i][i];
        }

        if n % 2 == 0 {
            ans
        } else {
            ans - mat[n / 2][n / 2]
        }
    }
}
