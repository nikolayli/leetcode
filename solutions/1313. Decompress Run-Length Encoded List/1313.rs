// Time complexity: O(nm)
// Space complexity: O(m)
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        ((0..nums.len()).step_by(2))
            .map(|i| vec![nums[i + 1]; nums[i] as usize])
            .flatten()
            .collect::<Vec<i32>>()
    }
}
