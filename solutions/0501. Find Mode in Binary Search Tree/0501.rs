// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut count: HashMap<i32, i32> = HashMap::new();

        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, count: &mut HashMap<i32, i32>) {
            if let Some(n) = node {
                let n_borrowed = n.borrow();
                inorder(&n_borrowed.left, count);
                *count.entry(n_borrowed.val).or_insert(0) += 1;
                inorder(&n_borrowed.right, count);
            }
        }

        inorder(&root, &mut count);
        let max_count = count.values().cloned().max().unwrap_or(0);

        count
            .into_iter()
            .filter(|&(_, freq)| freq == max_count)
            .map(|(val, _)| val)
            .collect()
    }
}
