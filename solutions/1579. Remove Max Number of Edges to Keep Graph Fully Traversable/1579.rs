// Time complexity: O(sort)
// Space complexity: O(n)
struct UnionFind {
    count: usize,
    id: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            count: n,
            id: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn union_by_rank(&mut self, u: usize, v: usize) -> bool {
        let i = self.find(u);
        let j = self.find(v);
        if i == j {
            return false;
        }
        if self.rank[i] < self.rank[j] {
            self.id[i] = j;
        } else if self.rank[i] > self.rank[j] {
            self.id[j] = i;
        } else {
            self.id[i] = j;
            self.rank[j] += 1;
        }
        self.count -= 1;

        true
    }

    fn find(&mut self, u: usize) -> usize {
        if self.id[u] != u {
            self.id[u] = self.find(self.id[u]);
        }

        self.id[u]
    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut alice = UnionFind::new(n);
        let mut bob = UnionFind::new(n);
        let mut required_edges = 0;

        edges.sort_unstable_by(|a, b| b[0].cmp(&a[0]));

        for edge in &edges {
            let edge_type = edge[0];
            let u = (edge[1] - 1) as usize;
            let v = (edge[2] - 1) as usize;
            if edge_type == 3 {
                if alice.union_by_rank(u, v) | bob.union_by_rank(u, v) {
                    required_edges += 1;
                }
            } else if edge_type == 2 {
                if bob.union_by_rank(u, v) {
                    required_edges += 1;
                }
            } else {
                if alice.union_by_rank(u, v) {
                    required_edges += 1;
                }
            }
        }

        if alice.count == 1 && bob.count == 1 {
            (edges.len() - required_edges) as i32
        } else {
            -1
        }
    }
}
