// Time complexity: O(n^2+nm)
// Space complexity: O(n+m)
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ans: Vec<Vec<i32>> = vec![vec![]; n];
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v as i32);
        }

        fn dfs(
            u: usize,
            ancestor: i32,
            seen: &mut Vec<bool>,
            graph: &Vec<Vec<i32>>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            seen[u] = true;
            for &v in &graph[u] {
                let v = v as usize;
                if seen[v] {
                    continue;
                }
                ans[v].push(ancestor);
                dfs(v, ancestor, seen, graph, ans);
            }
        }

        for i in 0..n {
            let mut seen = vec![false; n];
            dfs(i, i as i32, &mut seen, &graph, &mut ans);
        }

        ans
    }
}
