// Time complexity: O(E+VlogV)
// Space complexity: O(E+V)
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct FloatOrd(f64);

impl Eq for FloatOrd {}

impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let (start, end) = (start as usize, end as usize);
        let graph = {
            let mut graph = vec![Vec::new(); n as usize];
            for (edge, succ) in edges.into_iter().zip(succ_prob.into_iter()) {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                graph[u].push((v, FloatOrd(succ)));
                graph[v].push((u, FloatOrd(succ)));
            }
            graph
        };

        let mut max_prob = vec![None; n as usize];
        let mut max_heap = BinaryHeap::from([(FloatOrd(1.0), start)]);

        while let Some((FloatOrd(prob), u)) = max_heap.pop() {
            if u == end {
                return prob;
            }
            if max_prob[u].is_some() {
                continue;
            }
            max_prob[u] = Some(prob);

            for &(next_node, FloatOrd(edge_prob)) in graph[u].iter() {
                if max_prob[next_node].is_some() {
                    continue;
                }
                max_heap.push((FloatOrd(prob * edge_prob), next_node));
            }
        }

        0.0
    }
}
