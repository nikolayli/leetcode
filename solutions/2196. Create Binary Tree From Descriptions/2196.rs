// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut children = HashSet::new();
        let mut val_to_node: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();

        for d in descriptions.iter() {
            let (p, c, is_left) = (d[0], d[1], d[2] == 1);
            let parent = val_to_node
                .entry(p)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(p))))
                .clone();
            let child = val_to_node
                .entry(c)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(c))))
                .clone();
            if is_left {
                parent.borrow_mut().left = Some(child.clone());
            } else {
                parent.borrow_mut().right = Some(child.clone());
            }
            children.insert(c);
        }

        let root_val = val_to_node
            .keys()
            .find(|&&k| !children.contains(&k))
            .unwrap();
        val_to_node.get(root_val).cloned()
    }
}
