// Time complexity: O(max(n, m))
// Space complexity: O(max(n, m))
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        let mut carry = 0;

        while carry != 0 || l1.is_some() || l2.is_some() {
            if let Some(node) = l1 {
                carry += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                carry += node.val;
                l2 = node.next;
            }
            curr.next = Some(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            curr = curr.next.as_mut().unwrap();
        }

        dummy.next
    }
}
