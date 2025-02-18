// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .copied()
            .filter(|&n| n > 0)
            .zip(nums.iter().copied().filter(|&n| n < 0))
            .flat_map(|(a, b)| [a, b])
            .collect()
    }
}
