// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
        let min_num = nums.iter().min().unwrap();

        if nums.iter().any(|&num| num % min_num > 0) {
            return 1;
        }

        (nums.iter().filter(|&num| num == min_num).count() + 1) as i32 / 2
    }
}
