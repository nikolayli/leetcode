// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums = Vec::new();
        Self::inorder(&root, &mut nums);
        Self::build(&nums, 0, nums.len() as i32 - 1)
    }

    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::inorder(&node.left, nums);
            nums.push(node.val);
            Self::inorder(&node.right, nums);
        }
    }

    fn build(nums: &Vec<i32>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }
        let m = (l + r) / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(nums[m as usize])));
        node.borrow_mut().left = Self::build(nums, l, m - 1);
        node.borrow_mut().right = Self::build(nums, m + 1, r);
        Some(node)
    }
}
