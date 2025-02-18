// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = i32::MAX;
        let mut dp = vec![0; 1000001];

        for i in 0..nums.len() {
            let mut val = nums[i];
            for j in i..nums.len() {
                val &= nums[j];
                ans = ans.min((k - val).abs());
                if k - val >= ans || val <= dp[j] {
                    break;
                }
                dp[j] = val;
            }
        }

        ans
    }
}
