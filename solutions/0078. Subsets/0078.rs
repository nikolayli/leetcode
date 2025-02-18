// Time complexity: O(2^n)
// Space complexity: O(n⋅2^n)
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut path = Vec::new();

        fn dfs(nums: &[i32], start: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            ans.push(path.clone());
            for i in start..nums.len() {
                path.push(nums[i]);
                dfs(nums, i + 1, path, ans);
                path.pop();
            }
        }
        dfs(&nums, 0, &mut path, &mut ans);

        ans
    }
}
