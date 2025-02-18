// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn add_one_row(mut root: Option<Node>, val: i32, depth: i32) -> Option<Node> {
        if depth == 1 {
            let t = TreeNode {
                left: root,
                ..TreeNode::new(val)
            };
            return Some(Rc::new(RefCell::new(t)));
        }

        fn add_row(node: Option<&mut Node>, val: i32, k: i32, depth: i32) {
            match node {
                Some(node) => {
                    let mut node = node.borrow_mut();
                    if k == depth - 1 {
                        node.left = Some(Rc::new(RefCell::new(TreeNode {
                            left: node.left.take(),
                            ..TreeNode::new(val)
                        })));
                        node.right = Some(Rc::new(RefCell::new(TreeNode {
                            right: node.right.take(),
                            ..TreeNode::new(val)
                        })));
                        return;
                    } else {
                        add_row(node.left.as_mut(), val, k + 1, depth);
                        add_row(node.right.as_mut(), val, k + 1, depth);
                    }
                }
                None => (),
            }
        }
        add_row(root.as_mut(), val, 1, depth);
        return root;
    }
}
