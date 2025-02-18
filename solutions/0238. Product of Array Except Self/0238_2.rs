// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];

        for i in 1..n {
            ans[i] = ans[i - 1] * nums[i - 1];
        }

        let mut suffix = 1;
        for i in (0..n).rev() {
            ans[i] *= suffix;
            suffix *= nums[i];
        }

        ans
    }
}
