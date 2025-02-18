// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 5 {
            return 0;
        }

        nums.sort_unstable();

        let mut ans = i32::MAX;

        for i in 0..4 {
            ans = ans.min(nums[n - 4 + i] - nums[i]);
        }

        ans
    }
}
