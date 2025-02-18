// Time complexity: O(mn)
// Space complexity: O(n)
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let n = points[0].len();
        let mut dp = vec![0i64; n];

        for row in points.iter() {
            let mut left_to_right = vec![0i64; n];
            let mut running_max = 0i64;

            for j in 0..n {
                running_max = (running_max - 1).max(dp[j]);
                left_to_right[j] = running_max;
            }

            let mut right_to_left = vec![0i64; n];
            running_max = 0i64;

            for j in (0..n).rev() {
                running_max = (running_max - 1).max(dp[j]);
                right_to_left[j] = running_max;
            }

            for j in 0..n {
                dp[j] = left_to_right[j].max(right_to_left[j]) + row[j] as i64;
            }
        }

        *dp.iter().max().unwrap()
    }
}
