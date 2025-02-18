// Time complexity: O((V + E) log V)
// Space complexity: O(V + E)
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Vertex {
    id: usize,
    distance: i32,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n as usize];

        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            graph[u].push((v, w));
            graph[v].push((u, w));
        }

        Self::dijkstra(&graph, 0, &disappear)
    }

    fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, src: usize, disappear: &Vec<i32>) -> Vec<i32> {
        let mut dist: Vec<i32> = vec![std::i32::MAX; graph.len()];
        let mut min_heap: BinaryHeap<Vertex> = BinaryHeap::new();

        dist[src] = 0;
        min_heap.push(Vertex {
            id: src,
            distance: 0,
        });

        while let Some(Vertex { id: u, distance: d }) = min_heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                if d + w < disappear[v] && d + w < dist[v] {
                    dist[v] = d + w;
                    min_heap.push(Vertex {
                        id: v,
                        distance: dist[v],
                    });
                }
            }
        }

        for d in &mut dist {
            if *d == std::i32::MAX {
                *d = -1;
            }
        }

        dist
    }
}
