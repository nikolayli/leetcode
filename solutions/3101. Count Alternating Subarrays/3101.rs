// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 1;
        let mut size: i32 = 1;

        for i in 1..nums.len() {
            size = if nums[i - 1] == nums[i] { 1 } else { size + 1 };
            ans += size as i64
        }

        ans
    }
}
