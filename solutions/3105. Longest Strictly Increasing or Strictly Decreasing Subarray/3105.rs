// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let mut inc_length = 1;
        let mut dec_length = 1;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                inc_length += 1;
                dec_length = 1;
            } else if nums[i] < nums[i - 1] {
                dec_length += 1;
                inc_length = 1;
            } else {
                dec_length = 1;
                inc_length = 1;
            }
            ans = ans.max(inc_length).max(dec_length);
        }

        ans
    }
}
