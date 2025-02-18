// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_non_zero_found_at = 0;
        let mut cur = 0;

        while cur < nums.len() {
            if nums[cur] != 0 {
                nums.swap(last_non_zero_found_at, cur);
                last_non_zero_found_at += 1;
            }
            cur += 1;
        }
    }
}
