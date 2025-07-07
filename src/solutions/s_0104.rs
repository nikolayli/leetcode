use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node_ref = node.borrow();
                let left_depth = Self::max_depth(node_ref.left.clone());
                let right_depth = Self::max_depth(node_ref.right.clone());
                1 + left_depth.max(right_depth)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn build_tree(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = nodes.iter().cloned();
        let root = match iter.next() {
            Some(Some(val)) => Rc::new(RefCell::new(TreeNode::new(val))),
            _ => return None,
        };

        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        while let Some(node) = queue.pop_front() {
            if let Some(Some(val)) = iter.next() {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
            }

            if let Some(Some(val)) = iter.next() {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
        }

        Some(root)
    }

    macro_rules! max_depth_test {
        ($name:ident: $nodes:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let tree = build_tree($nodes);
                assert_eq!(Solution::max_depth(tree), $expected);
            }
        };
    }

    max_depth_test!(case1: &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] => 3);
    max_depth_test!(case2: &[Some(1), None, Some(2)] => 2);
}
