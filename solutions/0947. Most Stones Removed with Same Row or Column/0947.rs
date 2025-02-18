// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashSet;

struct UnionFind {
    parent: Vec<i32>,
    component_count: i32,
    unique_nodes: HashSet<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n],
            component_count: 0,
            unique_nodes: HashSet::new(),
        }
    }

    fn find(&mut self, node: i32) -> i32 {
        if !self.unique_nodes.contains(&node) {
            self.component_count += 1;
            self.unique_nodes.insert(node);
        }

        if self.parent[node as usize] == -1 {
            return node;
        }

        let root = self.find(self.parent[node as usize]);
        self.parent[node as usize] = root;

        root
    }

    fn union_nodes(&mut self, node1: i32, node2: i32) {
        let root1 = self.find(node1);
        let root2 = self.find(node2);

        if root1 != root2 {
            self.parent[root1 as usize] = root2;
            self.component_count -= 1;
        }
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut uf = UnionFind::new(20002);

        for stone in stones {
            uf.union_nodes(stone[0], stone[1] + 10001);
        }

        n as i32 - uf.component_count
    }
}
