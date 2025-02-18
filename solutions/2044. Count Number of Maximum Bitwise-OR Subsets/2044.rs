// Time complexity: O(2^n)
// Space complexity: O(2^n)
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let ors = nums.iter().fold(0, |acc, &num| acc | num);
        let mut ans = 0;

        fn dfs(nums: &[i32], i: usize, path: i32, ors: i32, ans: &mut i32) {
            if i == nums.len() {
                if path == ors {
                    *ans += 1;
                }
                return;
            }

            dfs(nums, i + 1, path, ors, ans);
            dfs(nums, i + 1, path | nums[i], ors, ans);
        }

        dfs(&nums, 0, 0, ors, &mut ans);
        ans
    }
}
