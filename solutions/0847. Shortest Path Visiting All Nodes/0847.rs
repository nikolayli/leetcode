// Time complexity: O(n*2^n)
// Space complexity: O(n*2^n)
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let goal = (1 << n) - 1;

        let mut ans = 0;
        let mut q = VecDeque::new();
        let mut seen = vec![vec![false; 1 << n]; n];

        for i in 0..n {
            q.push_back((i, 1 << i));
        }

        while !q.is_empty() {
            for _ in 0..q.len() {
                let (u, state) = q.pop_front().unwrap();

                if state == goal {
                    return ans;
                }

                if seen[u][state] {
                    continue;
                }

                seen[u][state] = true;

                for &v in &graph[u] {
                    q.push_back((v as usize, state | (1 << v)))
                }
            }
            ans += 1;
        }

        -1
    }
}
