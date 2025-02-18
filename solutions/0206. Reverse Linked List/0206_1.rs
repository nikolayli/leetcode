// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse(head, None)
    }

    fn reverse(
        head: Option<Box<ListNode>>,
        new_head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let next = node.next.take();
                node.next = new_head;
                Self::reverse(next, Some(node))
            }
            None => new_head,
        }
    }
}
