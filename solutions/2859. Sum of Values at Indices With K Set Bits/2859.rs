// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|(index, _)| index.count_ones() as i32 == k)
            .map(|(_, &num)| num)
            .sum::<i32>()
    }
}
