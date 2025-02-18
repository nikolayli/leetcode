// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn replace_value_in_tree(root: TreeNodeRef) -> TreeNodeRef {
        let mut level_sums = vec![];

        fn dfs(node: &TreeNodeRef, level: usize, level_sums: &mut Vec<i32>) {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                if level == level_sums.len() {
                    level_sums.push(0);
                }
                level_sums[level] += n.val;
                dfs(&n.left, level + 1, level_sums);
                dfs(&n.right, level + 1, level_sums);
            }
        }

        fn replace(
            node: &TreeNodeRef,
            level: usize,
            curr: &TreeNodeRef,
            level_sums: &Vec<i32>,
        ) -> TreeNodeRef {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                let next_level = level + 1;
                let next_level_cousins_sum = if next_level < level_sums.len() {
                    level_sums[next_level]
                        - n.left.as_ref().map_or(0, |left| left.borrow().val)
                        - n.right.as_ref().map_or(0, |right| right.borrow().val)
                } else {
                    0
                };
                if let Some(left) = &n.left {
                    let new_left = Rc::new(RefCell::new(TreeNode::new(next_level_cousins_sum)));
                    curr.as_ref().unwrap().borrow_mut().left = Some(new_left.clone());
                    replace(&Some(left.clone()), next_level, &Some(new_left), level_sums);
                }
                if let Some(right) = &n.right {
                    let new_right = Rc::new(RefCell::new(TreeNode::new(next_level_cousins_sum)));
                    curr.as_ref().unwrap().borrow_mut().right = Some(new_right.clone());
                    replace(
                        &Some(right.clone()),
                        next_level,
                        &Some(new_right),
                        level_sums,
                    );
                }
            }
            curr.clone()
        }

        dfs(&root, 0, &mut level_sums);
        replace(
            &root,
            0,
            &Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            &level_sums,
        )
    }
}
