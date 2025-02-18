// Time complexity: O(mn)
// Space complexity: O(1)
impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        if rows == 0 {
            return 0;
        }
        let cols = matrix[0].len();
        let mut count = 0;

        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == 1 && i > 0 && j > 0 {
                    matrix[i][j] += matrix[i - 1][j - 1]
                        .min(matrix[i - 1][j])
                        .min(matrix[i][j - 1]);
                }
                count += matrix[i][j];
            }
        }

        count
    }
}
