// Time complexity: O(∣rowConditions∣+∣colConditions∣+k)
// Space complexity: O(k^2)
use std::collections::VecDeque;

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;
        let row_order = Self::topological_sort(&row_conditions, k);
        if row_order.is_empty() {
            return vec![];
        }
        let col_order = Self::topological_sort(&col_conditions, k);
        if col_order.is_empty() {
            return vec![];
        }
        let mut ans = vec![vec![0; k]; k];
        let mut node_to_row_index = vec![0; k + 1];

        for (i, &node) in row_order.iter().enumerate() {
            node_to_row_index[node as usize] = i;
        }
        for (j, &node) in col_order.iter().enumerate() {
            let i = node_to_row_index[node as usize];
            ans[i][j] = node;
        }

        ans
    }

    fn topological_sort(conditions: &[Vec<i32>], n: usize) -> Vec<i32> {
        let mut order = Vec::new();
        let mut graph = vec![Vec::new(); n + 1];
        let mut in_degrees = vec![0; n + 1];
        let mut q = VecDeque::new();

        for condition in conditions {
            let u = condition[0] as usize;
            let v = condition[1] as usize;
            graph[u].push(v);
            in_degrees[v] += 1;
        }
        for i in 1..=n {
            if in_degrees[i] == 0 {
                q.push_back(i as i32);
            }
        }
        while let Some(u) = q.pop_front() {
            order.push(u);
            for &v in &graph[u as usize] {
                in_degrees[v] -= 1;
                if in_degrees[v] == 0 {
                    q.push_back(v as i32);
                }
            }
        }

        if order.len() == n {
            order
        } else {
            Vec::new()
        }
    }
}
