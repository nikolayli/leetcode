// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn longest_artith_seq_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut dp = vec![vec![0; 1001]; n];

        for i in 0..n {
            for j in 0..i {
                let k = (nums[i] - nums[j] + 500) as usize;
                dp[i][k] = 2.max(dp[j][k] + 1);
                ans = ans.max(dp[i][k]);
            }
        }

        ans
    }
}
