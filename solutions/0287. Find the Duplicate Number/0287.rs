// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[nums[0] as usize];
        let mut fast = nums[nums[nums[0] as usize] as usize];

        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
        }

        slow = nums[0];

        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        slow
    }
}
