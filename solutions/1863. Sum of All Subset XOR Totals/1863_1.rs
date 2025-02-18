// Time complexity: O(2^n)
// Space complexity: O(n)
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn dfs(nums: &Vec<i32>, i: usize, xors: i32) -> i32 {
            if i == nums.len() {
                return xors;
            }
            let x = dfs(nums, i + 1, xors);
            let y = dfs(nums, i + 1, nums[i] ^ xors);

            x + y
        }

        dfs(&nums, 0, 0)
    }
}
