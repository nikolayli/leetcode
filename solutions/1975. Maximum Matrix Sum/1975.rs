// Time complexity: O(mn)
// Space complexity: O(1)
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut abs_sum: i64 = 0;
        let mut min_abs: i32 = i32::MAX;
        let mut odd_neg = 0;

        for row in &matrix {
            for &num in row {
                abs_sum += num.abs() as i64;
                min_abs = min_abs.min(num.abs());
                if num < 0 {
                    odd_neg ^= 1;
                }
            }
        }

        abs_sum - (odd_neg * min_abs as i64 * 2)
    }
}
