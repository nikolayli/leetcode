// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.as_ref()?.next.clone();
        let mut result_head = ListNode::new(0);
        let mut result_tail = &mut result_head;

        while let Some(mut node) = curr {
            let mut running = node.as_mut();
            let mut sum = 0;

            while running.val != 0 {
                sum += running.val;
                running = running.next.as_mut().unwrap();
            }

            result_tail.next = Some(Box::new(ListNode::new(sum)));
            result_tail = result_tail.next.as_mut().unwrap();

            curr = running.next.take();
        }

        result_head.next
    }
}
