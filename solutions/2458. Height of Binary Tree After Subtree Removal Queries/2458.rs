// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn tree_queries(root: TreeNodeRef, queries: Vec<i32>) -> Vec<i32> {
        fn height(node: &TreeNodeRef, cache: &mut HashMap<i32, i32>) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                if let Some(&h) = cache.get(&node.val) {
                    return h;
                }
                let h = 1 + height(&node.left, cache).max(height(&node.right, cache));
                cache.insert(node.val, h);
                h
            } else {
                0
            }
        }

        fn dfs(
            node: &TreeNodeRef,
            depth: i32,
            max_height: i32,
            val_to_max_height: &mut HashMap<i32, i32>,
            height_cache: &mut HashMap<i32, i32>,
        ) {
            if let Some(node) = node {
                let node = node.borrow();
                val_to_max_height.insert(node.val, max_height);
                dfs(
                    &node.left,
                    depth + 1,
                    (depth + height(&node.right, height_cache)).max(max_height),
                    val_to_max_height,
                    height_cache,
                );
                dfs(
                    &node.right,
                    depth + 1,
                    (depth + height(&node.left, height_cache)).max(max_height),
                    val_to_max_height,
                    height_cache,
                );
            }
        }

        let mut height_cache = HashMap::new();
        let mut val_to_max_height = HashMap::new();
        dfs(&root, 0, 0, &mut val_to_max_height, &mut height_cache);

        queries
            .into_iter()
            .map(|q| *val_to_max_height.get(&q).unwrap_or(&0))
            .collect()
    }
}
