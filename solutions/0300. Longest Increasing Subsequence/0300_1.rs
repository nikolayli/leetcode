// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut dp = vec![1; nums.len()];

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
        }

        return *dp.iter().max().unwrap();
    }
}
