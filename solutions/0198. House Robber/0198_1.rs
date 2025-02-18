// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp: Vec<i32> = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);

        for i in 2..nums.len() {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }

        return dp.last().cloned().unwrap();
    }
}
