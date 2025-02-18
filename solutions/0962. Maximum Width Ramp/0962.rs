// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut ans = 0;

        for (i, &num) in nums.iter().enumerate() {
            if stack.is_empty() || num < nums[*stack.last().unwrap()] {
                stack.push(i);
            }
        }

        for i in (0..nums.len()).rev() {
            while !stack.is_empty() && nums[i] >= nums[*stack.last().unwrap()] {
                ans = ans.max(i - stack.pop().unwrap());
            }
            if i <= ans {
                break;
            }
        }

        ans as i32
    }
}
