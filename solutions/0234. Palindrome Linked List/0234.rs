// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut len = 0;
        let mut curr = &head;

        while let Some(node) = curr {
            curr = &node.next;
            len += 1;
        }

        let mut curr = &mut head;
        for _ in 0..(len - 1) / 2 {
            curr = &mut curr.as_mut().unwrap().next;
        }

        let mut prev = None;
        let mut curr = curr.as_mut().unwrap().next.take();
        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }

        let mut right_part = &prev;
        let mut left_part = &head;
        while let (Some(r), Some(l)) = (right_part, left_part) {
            if r.val != l.val {
                return false;
            }
            right_part = &r.next;
            left_part = &l.next;
        }

        true
    }
}
