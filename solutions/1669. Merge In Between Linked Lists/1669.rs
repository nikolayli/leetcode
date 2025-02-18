// Time complexity: O(n+m)
// Space complexity: O(1)
impl Solution {
    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut node_before_a = Box::new(ListNode::new(0));
        node_before_a.next = list1;
        let mut curr = &mut node_before_a;

        for _ in 0..a {
            curr = curr.next.as_mut().unwrap();
        }

        let mut after = &mut curr.next;
        for _ in a..=b {
            after = &mut after.as_mut().unwrap().next;
        }

        let after_b = after.take();
        curr.next = list2;

        while let Some(ref mut next) = curr.next {
            curr = next;
        }

        curr.next = after_b;
        node_before_a.next
    }
}
