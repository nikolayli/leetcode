// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        nums.sort_unstable();

        for i in (1..nums.len()).rev() {
            if nums[i] != nums[i - 1] {
                ans += nums.len() - i;
            }
        }

        ans as i32
    }
}
