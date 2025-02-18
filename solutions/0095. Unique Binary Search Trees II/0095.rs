// Time complexity: O(C_n)
// Space complexity: O(C_n)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        Self::generate_trees_range(1, n)
    }

    fn generate_trees_range(mn: i32, mx: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if mn > mx {
            return vec![None];
        }

        let mut ans = Vec::new();

        for i in mn..=mx {
            let left_trees = Self::generate_trees_range(mn, i - 1);
            let right_trees = Self::generate_trees_range(i + 1, mx);

            for left in &left_trees {
                for right in &right_trees {
                    let node = Rc::new(RefCell::new(TreeNode::new(i)));
                    node.borrow_mut().left = left.clone();
                    node.borrow_mut().right = right.clone();
                    ans.push(Some(node));
                }
            }
        }

        ans
    }
}
