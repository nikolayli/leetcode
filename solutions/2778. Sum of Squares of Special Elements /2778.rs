// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|(i, _)| nums.len() % (i + 1) == 0)
            .map(|(_, &num)| num * num)
            .sum()
    }
}
