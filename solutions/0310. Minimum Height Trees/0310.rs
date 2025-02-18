// Time complexity: O(V+E)
// Space complexity: O(V+E)
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut degree = vec![0; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
            degree[u] += 1;
            degree[v] += 1;
        }

        let mut leaves = degree
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| if d <= 1 { Some(i) } else { None })
            .collect::<Vec<_>>();

        while n > 2 {
            let mut next_leaves = Vec::new();

            for u in leaves {
                for &v in &graph[u] {
                    degree[v] -= 1;
                    if degree[v] == 1 {
                        next_leaves.push(v);
                    }
                }
                n -= 1;
            }
            leaves = next_leaves;
        }

        leaves.into_iter().map(|v| v as i32).collect()
    }
}
