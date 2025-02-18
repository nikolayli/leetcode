// Time complexity: O(V+E)
// Space complexity: O(V+E)
use std::collections::VecDeque;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n + 1];
        let mut min_time = vec![[i32::MAX; 2]; n + 1];
        let mut queue = VecDeque::new();

        min_time[1][0] = 0;
        queue.push_back((1, 0));

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        while let Some((i, prev_time)) = queue.pop_front() {
            let num_change_signal = prev_time / change;
            let wait_time = if num_change_signal % 2 == 0 {
                0
            } else {
                change - prev_time % change
            };
            let new_time = prev_time + wait_time + time;

            for &j in &graph[i] {
                if new_time < min_time[j][0] {
                    min_time[j][0] = new_time;
                    queue.push_back((j, new_time));
                } else if min_time[j][0] < new_time && new_time < min_time[j][1] {
                    if j == n {
                        return new_time;
                    }
                    min_time[j][1] = new_time;
                    queue.push_back((j, new_time));
                }
            }
        }

        panic!("No solution found");
    }
}
