// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut curr = &mut head;

        while let Some(ref mut curr_node) = curr {
            if let Some(ref mut next_node) = curr_node.next {
                let gcd_val = Self::gcd(curr_node.val, next_node.val);
                let new_node = Box::new(ListNode {
                    val: gcd_val,
                    next: curr_node.next.take(),
                });
                curr_node.next = Some(new_node);
                curr = &mut curr_node.next.as_mut().unwrap().next;
            } else {
                break;
            }
        }

        head
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
