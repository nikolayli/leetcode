// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev1 = 0;
        let mut prev2 = 0;

        for num in nums {
            let dp = prev1.max(prev2 + num);
            prev2 = prev1;
            prev1 = dp;
        }

        prev1
    }
}
