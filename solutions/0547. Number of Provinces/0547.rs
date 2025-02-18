// Time complexity: O(n^2)
// Space complexity: O(n)
pub struct UnionFind {
    count: i32,
    id: Vec<usize>,
    rank: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut id: Vec<usize> = Vec::with_capacity(n);
        for i in 0..n {
            id.push(i);
        }
        UnionFind {
            count: n as i32,
            id,
            rank: vec![0; n],
        }
    }

    pub fn union_by_rank(&mut self, u: usize, v: usize) {
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
        self.count -= 1;
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }

    fn find(&mut self, u: usize) -> usize {
        if self.id[u] != u {
            self.id[u] = self.find(self.id[u]);
        }
        self.id[u]
    }
}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            for j in i..n {
                if is_connected[i][j] == 1 {
                    uf.union_by_rank(i, j);
                }
            }
        }

        uf.get_count()
    }
}
