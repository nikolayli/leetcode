// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn merge(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match head {
                None => None,
                Some(mut node) => {
                    if node.next.as_ref().unwrap().val == 0 {
                        let mut new_node = ListNode::new(node.val);
                        new_node.next = merge(node.next.unwrap().next);
                        Some(Box::new(new_node))
                    } else {
                        let mut next = merge(node.next);
                        next.as_mut().unwrap().val += node.val;
                        next
                    }
                }
            }
        }
        merge(head)
    }
}
