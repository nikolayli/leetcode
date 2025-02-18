// Time complexity: O(n^3)
// Space complexity: O(n^2)
use std::collections::HashSet;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut ans = 0;
        let mut graph = vec![Vec::new(); n];

        for i in 0..n {
            let xi = bombs[i][0] as i64;
            let yi = bombs[i][1] as i64;
            let ri = bombs[i][2] as i64;
            for j in 0..n {
                if i == j {
                    continue;
                }
                let (xj, yj) = (bombs[j][0] as i64, bombs[j][1] as i64);
                if ri * ri >= (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj) {
                    graph[i].push(j);
                }
            }
        }

        fn dfs(u: usize, graph: &Vec<Vec<usize>>, seen: &mut HashSet<usize>) {
            for &v in &graph[u] {
                if seen.insert(v) {
                    dfs(v, graph, seen);
                }
            }
        }

        for i in 0..n {
            let mut seen = HashSet::new();
            seen.insert(i);
            dfs(i, &graph, &mut seen);
            ans = ans.max(seen.len() as i32);
        }

        ans
    }
}
