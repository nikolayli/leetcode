// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        for i in 1..n {
            for j in 0..m {
                let min_val = (0.max(j as i32 - 1)..=(m as i32 - 1).min(j as i32 + 1))
                    .map(|k| matrix[i - 1][k as usize])
                    .min()
                    .unwrap();
                matrix[i][j] += min_val;
            }
        }

        *matrix[n - 1].iter().min().unwrap()
    }
}
