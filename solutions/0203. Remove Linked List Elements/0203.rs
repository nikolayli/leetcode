// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        while head.is_some() && head.as_ref().unwrap().val == val {
            head = head.unwrap().next;
        }
        if head.is_none() {
            return None;
        }
        let mut cur = head.as_mut().unwrap();
        assert_ne!(cur.val, val);
        while cur.next.is_some() {
            assert_ne!(cur.val, val);
            if cur.next.as_ref().unwrap().val == val {
                cur.next = cur.next.as_mut().unwrap().next.take();
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }
        head
    }
}
