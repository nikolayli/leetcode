// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut count = vec![1; n as usize];
        let mut tree = vec![HashSet::new(); n as usize];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].insert(v);
            tree[v].insert(u);
        }

        Self::postorder(&tree, 0, -1, &mut count, &mut ans);
        Self::preorder(&tree, 0, -1, &mut count, &mut ans);

        ans
    }

    fn postorder(
        tree: &Vec<HashSet<usize>>,
        node: usize,
        parent: i32,
        count: &mut Vec<i32>,
        ans: &mut Vec<i32>,
    ) {
        for &child in &tree[node] {
            if child as i32 == parent {
                continue;
            }
            Self::postorder(tree, child, node as i32, count, ans);
            count[node] += count[child];
            ans[node] += ans[child] + count[child];
        }
    }

    fn preorder(
        tree: &Vec<HashSet<usize>>,
        node: usize,
        parent: i32,
        count: &mut Vec<i32>,
        ans: &mut Vec<i32>,
    ) {
        for &child in &tree[node] {
            if child as i32 == parent {
                continue;
            }

            ans[child] = ans[node] - count[child] + (tree.len() as i32 - count[child]);
            Self::preorder(tree, child, node as i32, count, ans);
        }
    }
}
