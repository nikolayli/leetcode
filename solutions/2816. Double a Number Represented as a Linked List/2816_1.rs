// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let (carry, Some(ans)) = Self::get_carry(head) {
            if carry > 0 {
                Some(Box::new(ListNode {
                    val: carry,
                    next: Some(ans),
                }))
            } else {
                Some(ans)
            }
        } else {
            None
        }
    }

    fn get_carry(head: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
        match head {
            None => (0, None),
            Some(head) => {
                let (carry, next) = Self::get_carry(head.next);
                let val = head.val * 2 + carry;
                (
                    val / 10,
                    Some(Box::new(ListNode {
                        val: val % 10,
                        next: next,
                    })),
                )
            }
        }
    }
}
