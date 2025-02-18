// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut ans = nums.len() as i32;

        for i in 0..nums.len() {
            ans ^= i as i32 ^ nums[i];
        }

        ans
    }
}
