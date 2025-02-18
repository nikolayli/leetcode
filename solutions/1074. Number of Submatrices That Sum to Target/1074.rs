// Time complexity: O(mn^2)
// Space complexity: O(m)
use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut matrix = matrix;
        let mut ans = 0;

        for row in matrix.iter_mut() {
            for i in 1..n {
                row[i] += row[i - 1];
            }
        }

        for base_col in 0..n {
            for j in base_col..n {
                let mut prefix_count = HashMap::new();
                prefix_count.insert(0, 1);
                let mut sum = 0;
                for i in 0..m {
                    if base_col > 0 {
                        sum -= matrix[i][base_col - 1];
                    }
                    sum += matrix[i][j];
                    if let Some(&count) = prefix_count.get(&(sum - target)) {
                        ans += count;
                    }
                    *prefix_count.entry(sum).or_insert(0) += 1;
                }
            }
        }

        ans
    }
}
