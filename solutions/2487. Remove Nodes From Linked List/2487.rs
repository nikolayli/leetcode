// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => {
                node.next = Self::remove_nodes(node.next);
                if node.next.is_some() && node.val < node.next.as_ref().unwrap().val {
                    node.next
                } else {
                    Some(node)
                }
            }
        }
    }
}
