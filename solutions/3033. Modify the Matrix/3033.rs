// Time complexity: O(mn)
// Space complexity: O(1)
impl Solution {
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        for i in 0..cols {
            let mut max_val = 0;

            for j in 0..rows {
                max_val = std::cmp::max(max_val, matrix[j][i])
            }

            for j in 0..rows {
                if matrix[j][i] == -1 {
                    matrix[j][i] = max_val;
                }
            }
        }

        matrix
    }
}
