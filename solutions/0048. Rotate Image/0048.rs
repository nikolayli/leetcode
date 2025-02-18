// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();

        for i in 0..matrix.len() {
            for j in (i + 1)..matrix.len() {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }
}
