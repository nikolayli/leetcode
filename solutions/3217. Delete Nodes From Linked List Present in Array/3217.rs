// Time complexity: O(n+m)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums_set: HashSet<i32> = nums.into_iter().collect();

        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        let mut curr = &mut dummy;

        while let Some(ref mut next_node) = curr.next {
            if nums_set.contains(&next_node.val) {
                curr.next = next_node.next.take();
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
