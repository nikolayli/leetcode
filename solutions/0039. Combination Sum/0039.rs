// Time complexity: O(n^t)
// Space complexity: O(t)
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        candidates.sort_unstable();

        fn dfs(
            candidates: &Vec<i32>,
            start: usize,
            target: i32,
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
                path.push(candidates[i]);
                dfs(candidates, i, target - candidates[i], path, ans);
                path.pop();
            }
        }

        let mut path = Vec::new();
        dfs(&candidates, 0, target, &mut path, &mut ans);

        ans
    }
}
