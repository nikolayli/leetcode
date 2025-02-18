// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let mut path_to_start = String::new();
        let mut path_to_dest = String::new();

        Self::dfs(&root, start_value, &mut path_to_start);
        Self::dfs(&root, dest_value, &mut path_to_dest);

        while !path_to_start.is_empty()
            && !path_to_dest.is_empty()
            && path_to_start.chars().last() == path_to_dest.chars().last()
        {
            path_to_start.pop();
            path_to_dest.pop();
        }

        "U".repeat(path_to_start.len()) + &path_to_dest.chars().rev().collect::<String>()
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, path: &mut String) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val == val {
                return true;
            }
            if Self::dfs(&node.left, val, path) {
                path.push('L');
            } else if Self::dfs(&node.right, val, path) {
                path.push('R');
            }
            return !path.is_empty();
        }

        false
    }
}
