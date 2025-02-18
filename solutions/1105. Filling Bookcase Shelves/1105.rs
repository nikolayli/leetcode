// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 0..n {
            let mut sum_thickness = 0;
            let mut max_height = 0;
            for j in (0..=i).rev() {
                let thickness = books[j][0];
                let height = books[j][1];
                sum_thickness += thickness;
                if sum_thickness > shelf_width {
                    break;
                }
                max_height = max_height.max(height);
                dp[i + 1] = dp[i + 1].min(dp[j] + max_height);
            }
        }

        dp[n]
    }
}
