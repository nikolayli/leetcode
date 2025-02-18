// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;

        while let Some(mut curr) = head {
            head = curr.next.take();
            curr.next = prev;
            prev = Some(curr);
        }

        prev
    }
}
