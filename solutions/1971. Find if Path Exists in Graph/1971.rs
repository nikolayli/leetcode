// Time complexity: O(V+E)
// Space complexity: O(V+E)
struct UnionFind {
    id: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut id = Vec::with_capacity(n);
        let mut rank = Vec::with_capacity(n);
        for i in 0..n {
            id.push(i);
            rank.push(0);
        }
        UnionFind { id, rank }
    }

    fn union_by_rank(&mut self, u: usize, v: usize) {
        let i = self.find(u);
        let j = self.find(v);
        if i == j {
            return;
        }
        if self.rank[i] < self.rank[j] {
            self.id[i] = j;
        } else if self.rank[i] > self.rank[j] {
            self.id[j] = i;
        } else {
            self.id[i] = j;
            self.rank[j] += 1;
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.id[u] == u {
            u
        } else {
            let root = self.find(self.id[u]);
            self.id[u] = root;
            root
        }
    }
}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut uf = UnionFind::new(n);

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            uf.union_by_rank(u, v);
        }

        uf.find(source as usize) == uf.find(destination as usize)
    }
}
