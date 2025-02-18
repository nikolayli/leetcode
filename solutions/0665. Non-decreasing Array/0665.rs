// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 0;
        let mut prev = std::i32::MIN;

        for i in 0..nums.len() {
            if nums[i] < prev {
                count += 1;
                if count > 1 {
                    return false;
                }
                if i > 1 && nums[i - 2] > nums[i] {
                    continue;
                }
            }
            prev = nums[i];
        }

        true
    }
}
