// Time complexity: O(E×(V+E)logV)
// Space complexity: (V+E)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        const K_MAX: i32 = 2_000_000_000;
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;
        let mut graph = vec![vec![]; n];

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            if w != -1 {
                graph[u].push((v, w));
                graph[v].push((u, w));
            }
        }

        let mut dist_to_destination = Self::dijkstra(&graph, source, destination);
        if dist_to_destination < target {
            return vec![];
        }
        if dist_to_destination == target {
            for edge in &mut edges {
                if edge[2] == -1 {
                    edge[2] = K_MAX;
                }
            }
            return edges;
        }

        for i in 0..edges.len() {
            let u = edges[i][0] as usize;
            let v = edges[i][1] as usize;
            let w = &mut edges[i][2];
            if *w != -1 {
                continue;
            }
            *w = 1;
            graph[u].push((v, 1));
            graph[v].push((u, 1));
            dist_to_destination = Self::dijkstra(&graph, source, destination);
            if dist_to_destination <= target {
                *w += target - dist_to_destination;
                for j in i + 1..edges.len() {
                    if edges[j][2] == -1 {
                        edges[j][2] = K_MAX;
                    }
                }
                return edges;
            }
        }

        vec![]
    }

    fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, src: usize, dst: usize) -> i32 {
        let mut dist = vec![i32::MAX; graph.len()];
        let mut min_heap = BinaryHeap::new();

        dist[src] = 0;
        min_heap.push(Reverse((0, src)));

        while let Some(Reverse((d, u))) = min_heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                if d + w < dist[v] {
                    dist[v] = d + w;
                    min_heap.push(Reverse((dist[v], v)));
                }
            }
        }

        dist[dst]
    }
}
