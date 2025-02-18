// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }

        let mut mid = Self::find_mid(head);
        let reversed = Self::reverse(&mut mid);
        Self::merge(head, reversed);
    }

    fn find_mid(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head.clone();
        let mut slow = head;

        while fast.is_some() {
            fast = &(fast.as_ref().unwrap().next);
            if fast.is_some() {
                fast = &fast.as_ref().unwrap().next;
                slow = &mut (slow.as_mut().unwrap().next);
            }
        }

        slow.as_mut().unwrap().next.take()
    }

    fn reverse(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head.take();

        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            curr = next;
        }

        prev
    }

    fn merge(l1: &mut Option<Box<ListNode>>, l2: Option<Box<ListNode>>) {
        let mut l1 = l1.as_mut().unwrap();
        let mut l2 = l2;

        while l2.is_some() {
            let next = l1.next.take();
            l1.next = l2.take();
            l1 = l1.next.as_mut().unwrap();
            l2 = next;
        }
    }
}
