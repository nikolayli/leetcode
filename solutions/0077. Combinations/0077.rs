// Time complexity: O(C(n,k)⋅k)
// Space complexity: O(C(n,k))
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn dfs(n: i32, k: i32, start: i32, path: Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if path.len() == k as usize {
                ans.push(path);
                return;
            }
            for i in start..=n {
                let mut new_path = path.clone();
                new_path.push(i);
                dfs(n, k, i + 1, new_path, ans);
            }
        }

        let mut ans = Vec::new();
        dfs(n, k, 1, Vec::new(), &mut ans);

        ans
    }
}
