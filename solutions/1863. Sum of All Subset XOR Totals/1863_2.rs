// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &x| acc | x) << (nums.len() - 1)
    }
}
