// Time complexity: O(2^n)
// Space complexity: O(2^n)
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut path = Vec::new();
        candidates.sort_unstable();
        Self::dfs(&candidates, target, 0, &mut path, &mut ans);

        ans
    }

    fn dfs(
        candidates: &[i32],
        target: i32,
        start: usize,
        path: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            ans.push(path.clone());
            return;
        }

        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }
            path.push(candidates[i]);
            Self::dfs(candidates, target - candidates[i], i + 1, path, ans);
            path.pop();
        }
    }
}
