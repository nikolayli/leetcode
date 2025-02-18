// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut zeros = 0;

        for &num in &nums {
            if num == 0 {
                zeros += 1;
            }
            if zeros > 1 {
                if nums[l] == 0 {
                    zeros -= 1;
                }
                l += 1;
            }
        }

        (nums.len() as i32) - l as i32 - 1
    }
}
