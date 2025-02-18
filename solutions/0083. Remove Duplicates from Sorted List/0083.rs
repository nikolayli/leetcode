// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = &mut head;
        while let Some(node) = curr {
            while let Some(next) = &mut node.next {
                if node.val == next.val {
                    node.next = next.next.take();
                } else {
                    break;
                }
            }
            curr = &mut node.next;
        }

        head
    }
}
