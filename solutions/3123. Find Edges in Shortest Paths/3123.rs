// Time complexity: O((n+m)logn)
// Space complexity: O(n+m)
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        const INF: i32 = 1_000_000_007;
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        let mut dist: Vec<i32> = vec![INF; n];
        let mut edge_index: Vec<Vec<usize>> = vec![vec![]; n];
        let mut ans: Vec<bool> = vec![false; edges.len()];

        for i in 0..edges.len() {
            let u = edges[i][0] as usize;
            let v = edges[i][1] as usize;
            let w = edges[i][2];
            graph[u].push((v, w));
            graph[v].push((u, w));
            edge_index[u].push(i);
            edge_index[v].push(i);
        }

        let mut pq: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();

        pq.push((Reverse(0), 0));
        dist[0] = 0;

        while let Some((Reverse(cost), u)) = pq.pop() {
            if cost > dist[u] {
                continue;
            }

            for &(v, w) in &graph[u] {
                if dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    pq.push((Reverse(dist[v]), v));
                }
            }
        }

        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(n - 1);

        while let Some(u) = q.pop_front() {
            for i in 0..graph[u].len() {
                let (v, w) = graph[u][i];
                let idx = edge_index[u][i];

                if dist[u] == dist[v] + w {
                    ans[idx] = true;
                    if v != 0 {
                        q.push_back(v);
                    }
                }
            }
        }

        ans
    }
}
