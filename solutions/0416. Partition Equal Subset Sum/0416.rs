// Time complexity: O(nk)
// Space complexity: O(k)
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        Self::knapsack(&nums, (sum / 2) as usize)
    }

    fn knapsack(nums: &[i32], subset_sum: usize) -> bool {
        let mut dp = vec![false; subset_sum + 1];
        dp[0] = true;

        for &num in nums {
            for i in (num as usize..=subset_sum).rev() {
                dp[i] = dp[i] || dp[i - num as usize];
            }
        }

        dp[subset_sum]
    }
}
