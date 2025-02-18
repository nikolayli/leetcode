// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut max_index = 0;
        let mut same_num_length = 0;

        for i in 0..nums.len() {
            if nums[i] == nums[max_index] {
                same_num_length += 1;
                ans = ans.max(same_num_length);
            } else if nums[i] > nums[max_index] {
                max_index = i;
                same_num_length = 1;
                ans = 1;
            } else {
                same_num_length = 0;
            }
        }

        ans
    }
}
