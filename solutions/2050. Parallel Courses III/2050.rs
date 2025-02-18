// Time complexity: O(n+m)
// Space complexity: O(n+m)
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut in_degrees: Vec<usize> = vec![0; n];
        let mut dist = time.clone();

        relations.iter().for_each(|r| {
            let u = (r[0] - 1) as usize;
            let v = (r[1] - 1) as usize;
            graph[u].push(v);
            in_degrees[v] += 1;
        });

        let mut q: VecDeque<usize> = (0..n).filter(|&i| in_degrees[i] == 0).collect();

        while let Some(u) = q.pop_front() {
            graph[u].iter().for_each(|&v| {
                dist[v] = dist[v].max(dist[u] + time[v]);
                in_degrees[v] -= 1;
                if in_degrees[v] == 0 {
                    q.push_back(v);
                }
            });
        }

        *dist.iter().max().unwrap()
    }
}
