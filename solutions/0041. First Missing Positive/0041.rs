// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[i] != nums[(nums[i] - 1) as usize] {
                let temp = nums[i];
                nums[i] = nums[(temp - 1) as usize];
                nums[(temp - 1) as usize] = temp;
            }
        }

        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        (n + 1) as i32
    }
}
