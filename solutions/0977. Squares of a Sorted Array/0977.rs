// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums.len();
        let mut ans = vec![0; n];
        let (mut l, mut r) = (0, n - 1);

        while n > 0 {
            n -= 1;
            if nums[l].abs() > nums[r].abs() {
                ans[n] = nums[l] * nums[l];
                l += 1;
            } else {
                ans[n] = nums[r] * nums[r];
                r -= 1;
            }
        }

        ans
    }
}
