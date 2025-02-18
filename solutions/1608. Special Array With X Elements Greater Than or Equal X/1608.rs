// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        if nums[0] >= nums.len() as i32 {
            return nums.len() as i32;
        }

        for i in 0..nums.len() - 1 {
            let count = (nums.len() - i - 1) as i32;
            if nums[i] < count && nums[i + 1] >= count {
                return count;
            }
        }

        -1
    }
}
