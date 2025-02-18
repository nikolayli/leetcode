// Time complexity: O(q(n+q))
// Space complexity: O(n+q)
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut ans = Vec::new();
        let mut dist: Vec<i32> = (0..n as i32).collect();
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

        for i in 0..n - 1 {
            graph[i].push(i + 1);
        }

        for query in queries {
            let u = query[0] as usize;
            let v = query[1] as usize;
            graph[u].push(v);
            if dist[u] + 1 < dist[v] {
                dist[v] = dist[u] + 1;
                Self::bfs(&graph, v, &mut dist);
            }
            ans.push(dist[n - 1]);
        }

        ans
    }

    fn bfs(graph: &Vec<Vec<usize>>, start: usize, dist: &mut Vec<i32>) {
        let mut q = VecDeque::new();
        q.push_back(start);

        while let Some(u) = q.pop_front() {
            for &v in &graph[u] {
                if dist[u] + 1 < dist[v] {
                    dist[v] = dist[u] + 1;
                    q.push_back(v);
                }
            }
        }
    }
}
