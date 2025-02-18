// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_operations(nums: Vec<i32>, mut k: i32) -> i32 {
        for num in nums {
            k ^= num;
        }
        k.count_ones() as i32
    }
}
